use crate::options::{OutputFormat, OverwritePolicy};
use crate::scan_files::get_file_mime_type;
use caesium::parameters::{CSParameters, ChromaSubsampling};
use caesium::{compress_in_memory, compress_to_size_in_memory, convert_in_memory, SupportedFileTypes};
use indicatif::ProgressBar;
use rayon::iter::ParallelIterator;
use rayon::prelude::IntoParallelRefIterator;
use std::error::Error;
use std::ffi::OsString;
use std::fs::{File, FileTimes, Metadata};
use std::io::{BufReader, Read, Write};
use std::path::{absolute, Path, PathBuf};
use std::{fs, io};

#[derive(Debug)]
pub enum CompressionStatus {
    Success,
    Skipped,
    Error,
}
#[derive(Debug)]
pub struct CompressionResult {
    pub original_path: String,
    pub output_path: String,
    pub original_size: u64,
    pub compressed_size: u64,
    pub status: CompressionStatus,
    pub message: String,
}

pub struct CompressionOptions {
    pub quality: Option<u32>,
    pub max_size: Option<usize>,
    pub lossless: bool,
    pub exif: bool,
    pub png_opt_level: u8,
    pub zopfli: bool,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub long_edge: Option<u32>,
    pub short_edge: Option<u32>,
    pub output_folder: Option<PathBuf>,
    pub same_folder_as_input: bool,
    pub base_path: PathBuf,
    pub suffix: Option<String>,
    pub overwrite_policy: OverwritePolicy,
    pub format: OutputFormat,
    pub keep_dates: bool,
    pub keep_structure: bool,
    pub jpeg_chroma_subsampling: ChromaSubsampling,
    pub jpeg_baseline: bool,
}

const MAX_FILE_SIZE: u64 = 500 * 1024 * 1024;

pub fn start_compression(
    input_files: &[PathBuf],
    options: &CompressionOptions,
    progress_bar: &ProgressBar,
    dry_run: bool,
) -> Vec<CompressionResult> {
    input_files
        .par_iter()
        .map(|input_file| {
            let result = perform_compression(input_file, options, dry_run);
            progress_bar.inc(1);
            result
        })
        .collect()
}

fn perform_compression(input_file: &PathBuf, options: &CompressionOptions, dry_run: bool) -> CompressionResult {
    let needs_resize = is_resize_needed(options);
    let mut compression_result = CompressionResult {
        original_path: input_file.display().to_string(),
        output_path: String::new(),
        original_size: 0,
        compressed_size: 0,
        status: CompressionStatus::Error,
        message: String::new(),
    };

    let original_file_size = match get_file_size(input_file, &mut compression_result) {
        Some(size) => size,
        None => return compression_result,
    };

    if original_file_size > MAX_FILE_SIZE {
        compression_result.message = "File exceeds 500Mb, skipping.".to_string();
        compression_result.status = CompressionStatus::Skipped;
        return compression_result;
    }

    compression_result.original_size = original_file_size;

    let output_full_path = match setup_output_path(input_file, options, &mut compression_result) {
        Some(path) => path,
        None => return compression_result,
    };
    compression_result.output_path = output_full_path.display().to_string();

    if skip_due_to_overwrite_policy(options, &output_full_path, original_file_size, &mut compression_result) {
        return compression_result;
    }

    if dry_run {
        compression_result.status = CompressionStatus::Success;
        return compression_result;
    }

    let compressed_image = match perform_image_compression(input_file, options, needs_resize, &mut compression_result) {
        Some(image) => image,
        None => return compression_result,
    };

    let output_file_size = compressed_image.len() as u64;

    if skip_due_to_bigger_policy(
        options,
        &output_full_path,
        output_file_size,
        original_file_size,
        &mut compression_result,
    ) {
        return compression_result;
    }

    if let Err(msg) = write_compressed_file(&output_full_path, &compressed_image, options, input_file) {
        compression_result.message = msg;
        return compression_result;
    }

    compression_result.status = CompressionStatus::Success;
    compression_result.compressed_size = output_file_size;
    compression_result
}

fn is_resize_needed(options: &CompressionOptions) -> bool {
    options.width.is_some() || options.height.is_some() || options.long_edge.is_some() || options.short_edge.is_some()
}

fn get_file_size(input_file: &Path, compression_result: &mut CompressionResult) -> Option<u64> {
    match input_file.metadata() {
        Ok(metadata) => Some(metadata.len()),
        Err(_) => {
            compression_result.message = "Error reading file metadata".to_string();
            None
        }
    }
}

fn setup_output_path(
    input_file: &Path,
    options: &CompressionOptions,
    compression_result: &mut CompressionResult,
) -> Option<PathBuf> {
    let output_directory = determine_output_directory(input_file, options, compression_result)?;
    let (output_directory, filename) = compute_output_full_path(
        output_directory,
        input_file,
        &options.base_path,
        options.keep_structure,
        options.suffix.as_ref().unwrap_or(&String::new()).as_ref(),
        options.format,
    )?;

    if !output_directory.exists() && fs::create_dir_all(&output_directory).is_err() {
        compression_result.message = "Error creating output directory".to_string();
        return None;
    }

    Some(output_directory.join(filename))
}

fn determine_output_directory<'a>(
    input_file: &'a Path,
    options: &'a CompressionOptions,
    compression_result: &mut CompressionResult,
) -> Option<&'a Path> {
    if options.same_folder_as_input {
        match input_file.parent() {
            Some(p) => Some(p),
            None => {
                compression_result.message = "Error getting parent directory".to_string();
                None
            }
        }
    } else {
        match options.output_folder.as_ref() {
            Some(p) => Some(p),
            None => {
                compression_result.message = "Error getting output directory".to_string();
                None
            }
        }
    }
}

fn skip_due_to_overwrite_policy(
    options: &CompressionOptions,
    output_path: &Path,
    original_size: u64,
    compression_result: &mut CompressionResult,
) -> bool {
    if options.overwrite_policy == OverwritePolicy::Never && output_path.exists() {
        compression_result.status = CompressionStatus::Skipped;
        compression_result.compressed_size = original_size;
        compression_result.message = "File already exists, skipped due overwrite policy".to_string();
        return true;
    }

    false
}

fn perform_image_compression(
    input_file: &PathBuf,
    options: &CompressionOptions,
    needs_resize: bool,
    compression_result: &mut CompressionResult,
) -> Option<Vec<u8>> {
    let mut compression_parameters = match build_compression_parameters(options, input_file, needs_resize) {
        Ok(p) => p,
        Err(e) => {
            compression_result.message = format!("Error building compression parameters: {e}");
            return None;
        }
    };

    let input_file_buffer = match read_file_to_vec(input_file) {
        Ok(b) => b,
        Err(_) => {
            compression_result.message = "Error reading input file".to_string();
            return None;
        }
    };

    let compression_result_data = if options.max_size.is_some() {
        compress_to_size_in_memory(
            input_file_buffer,
            &mut compression_parameters,
            options.max_size.unwrap(),
            true,
        )
    } else if options.format != OutputFormat::Original {
        convert_in_memory(
            input_file_buffer,
            &compression_parameters,
            map_supported_formats(options.format),
        )
    } else {
        compress_in_memory(input_file_buffer, &compression_parameters)
    };

    match compression_result_data {
        Ok(compressed_image) => Some(compressed_image),
        Err(e) => {
            compression_result.message = format!("Error compressing file: {e}");
            None
        }
    }
}

fn skip_due_to_bigger_policy(
    options: &CompressionOptions,
    output_path: &Path,
    output_size: u64,
    original_size: u64,
    compression_result: &mut CompressionResult,
) -> bool {
    if output_path.exists() && options.overwrite_policy == OverwritePolicy::Bigger {
        match output_path.metadata() {
            Ok(existing_metadata) => {
                if existing_metadata.len() <= output_size {
                    compression_result.status = CompressionStatus::Skipped;
                    compression_result.compressed_size = original_size;
                    compression_result.message = "File already exists, skipped due overwrite policy".to_string();
                    return true;
                }
            }
            Err(_) => {
                compression_result.message = "Error reading existing file metadata".to_string();
                return false;
            }
        }
    }

    false
}

fn write_compressed_file(
    output_path: &PathBuf,
    compressed_image: &[u8],
    options: &CompressionOptions,
    input_file: &Path,
) -> Result<(), String> {
    let mut output_file = File::create(output_path).map_err(|_| "Error creating output file".to_string())?;

    output_file
        .write_all(compressed_image)
        .map_err(|_| "Error writing output file".to_string())?;

    if options.keep_dates {
        let input_metadata = input_file
            .metadata()
            .map_err(|_| "Error reading input file metadata for date preservation".to_string())?;

        preserve_file_times(&output_file, &input_metadata).map_err(|_| "Error preserving file times".to_string())?;
    }

    Ok(())
}

fn build_compression_parameters(
    options: &CompressionOptions,
    input_file: &Path,
    needs_resize: bool,
) -> Result<CSParameters, Box<dyn Error>> {
    let mut parameters = CSParameters::new();
    let quality = options.quality.unwrap_or(80);

    parameters.jpeg.quality = quality;
    parameters.png.quality = quality;
    parameters.webp.quality = quality;
    parameters.gif.quality = quality;

    parameters.optimize = options.lossless;

    parameters.keep_metadata = options.exif;

    parameters.jpeg.chroma_subsampling = options.jpeg_chroma_subsampling;
    parameters.jpeg.progressive = !options.jpeg_baseline;

    parameters.png.optimization_level = options.png_opt_level;
    parameters.png.force_zopfli = options.zopfli;

    if needs_resize {
        let mime_type = get_file_mime_type(input_file);
        build_resize_parameters(options, &mut parameters, input_file, mime_type)?;
    }

    Ok(parameters)
}

fn compute_output_full_path(
    output_directory: &Path,
    input_file_path: &Path,
    base_directory: &PathBuf,
    keep_structure: bool,
    suffix: &str,
    format: OutputFormat,
) -> Option<(PathBuf, OsString)> {
    let extension = match format {
        OutputFormat::Jpeg => "jpg".into(),
        OutputFormat::Png => "png".into(),
        OutputFormat::Webp => "webp".into(),
        OutputFormat::Tiff => "tiff".into(),
        OutputFormat::Original => input_file_path.extension().unwrap_or_default().to_os_string(),
    };

    let base_name = input_file_path.file_stem().unwrap_or_default().to_os_string();
    let mut output_file_name = base_name;
    output_file_name.push(suffix);
    if !extension.is_empty() {
        output_file_name.push(".");
        output_file_name.push(extension);
    }

    if keep_structure {
        let parent = match absolute(input_file_path.parent()?) {
            Ok(p) => p,
            Err(_) => return None,
        };

        let output_path_prefix = match parent.strip_prefix(base_directory) {
            Ok(p) => p,
            Err(_) => return None,
        };
        let full_output_directory = output_directory.join(output_path_prefix);
        Some((full_output_directory, output_file_name))
    } else {
        Some((PathBuf::from(output_directory), output_file_name))
    }
}

fn build_resize_parameters(
    options: &CompressionOptions,
    parameters: &mut CSParameters,
    input_file_path: &Path,
    mime_type: Option<String>,
) -> Result<(), Box<dyn Error>> {
    let (width, height) = get_real_resolution(input_file_path, mime_type, options.exif)?;

    if options.width.is_some() {
        parameters.width = options.width.unwrap_or(0);
    } else if options.height.is_some() {
        parameters.height = options.height.unwrap_or(0);
    } else if options.long_edge.is_some() {
        let long_edge = options.long_edge.unwrap_or(0);
        if width > height {
            parameters.width = long_edge;
        } else {
            parameters.height = long_edge;
        }
    } else if options.short_edge.is_some() {
        let short_edge = options.short_edge.unwrap_or(0);
        if width < height {
            parameters.width = short_edge;
        } else {
            parameters.height = short_edge;
        }
    }

    Ok(())
}

fn get_real_resolution(
    file: &Path,
    mime_type: Option<String>,
    keep_metadata: bool,
) -> Result<(usize, usize), Box<dyn Error>> {
    let resolution = imagesize::size(file)?;
    let mut orientation = 1;
    let mime = mime_type.unwrap_or("".to_string());
    if mime == "image/jpeg" && keep_metadata {
        let f = File::open(file)?;
        if let Ok(e) = exif::Reader::new().read_from_container(&mut BufReader::new(&f)) {
            let exif_field = match e.get_field(exif::Tag::Orientation, exif::In::PRIMARY) {
                Some(f) => f,
                None => return Ok((resolution.width, resolution.height)),
            };
            orientation = exif_field.value.get_uint(0).unwrap_or(1);
        };
    }
    let (width, height) = match orientation {
        5..=8 => (resolution.height, resolution.width),
        _ => (resolution.width, resolution.height),
    };

    Ok((width, height))
}

fn preserve_file_times(output_file: &File, original_file_metadata: &Metadata) -> io::Result<()> {
    let (last_modification_time, last_access_time) =
        (original_file_metadata.modified()?, original_file_metadata.accessed()?);
    output_file.set_times(
        FileTimes::new()
            .set_modified(last_modification_time)
            .set_accessed(last_access_time),
    )?;
    Ok(())
}
fn map_supported_formats(format: OutputFormat) -> SupportedFileTypes {
    match format {
        OutputFormat::Jpeg => SupportedFileTypes::Jpeg,
        OutputFormat::Png => SupportedFileTypes::Png,
        OutputFormat::Webp => SupportedFileTypes::WebP,
        OutputFormat::Tiff => SupportedFileTypes::Tiff,
        _ => SupportedFileTypes::Unkn,
    }
}

fn read_file_to_vec(file_path: &PathBuf) -> io::Result<Vec<u8>> {
    let mut file = File::open(file_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    Ok(buffer)
}

#[cfg(test)]
mod tests {
    use super::*;
    use indicatif::ProgressDrawTarget;
    use std::path::Path;
    use std::time::UNIX_EPOCH;
    use tempfile::tempdir;

    #[cfg(not(target_os = "windows"))]
    #[test]
    fn test_compute_output_full_path() {
        let output_directory = PathBuf::from("/output");
        let base_directory = PathBuf::from("/base");

        // Test case 1: keep_structure = true
        let input_file_path = PathBuf::from("/base/folder/test.jpg");
        let result = compute_output_full_path(
            &output_directory,
            &input_file_path,
            &base_directory,
            true,
            "_suffix",
            OutputFormat::Original,
        )
        .unwrap();
        assert_eq!(
            result,
            (Path::new("/output/folder").to_path_buf(), "test_suffix.jpg".into())
        );

        // Test case 2: keep_structure = false
        let result = compute_output_full_path(
            &output_directory,
            &input_file_path,
            &base_directory,
            false,
            "_suffix",
            OutputFormat::Original,
        )
        .unwrap();
        assert_eq!(result, (Path::new("/output").to_path_buf(), "test_suffix.jpg".into()));

        // Test case 3: input file without extension
        let input_file_path = PathBuf::from("/base/folder/test");
        let result = compute_output_full_path(
            &output_directory,
            &input_file_path,
            &base_directory,
            false,
            "_suffix",
            OutputFormat::Original,
        )
        .unwrap();
        assert_eq!(result, (Path::new("/output").to_path_buf(), "test_suffix".into()));

        // Test case 4: input file with different base directory
        let input_file_path = PathBuf::from("/different_base/folder/test.jpg");
        let result = compute_output_full_path(
            &output_directory,
            &input_file_path,
            &base_directory,
            false,
            "_suffix",
            OutputFormat::Original,
        )
        .unwrap();
        assert_eq!(result, (Path::new("/output").to_path_buf(), "test_suffix.jpg".into()));

        // Test case 5: input file with OutputFormat::Jpeg
        let result = compute_output_full_path(
            &output_directory,
            &input_file_path,
            &base_directory,
            false,
            "_suffix",
            OutputFormat::Jpeg,
        )
        .unwrap();
        assert_eq!(result, (Path::new("/output").to_path_buf(), "test_suffix.jpg".into()));

        // Test case 6: input file with OutputFormat::Png
        let result = compute_output_full_path(
            &output_directory,
            &input_file_path,
            &base_directory,
            false,
            "_suffix",
            OutputFormat::Png,
        )
        .unwrap();
        assert_eq!(result, (Path::new("/output").to_path_buf(), "test_suffix.png".into()));

        // Test case 7: input file with OutputFormat::Webp
        let result = compute_output_full_path(
            &output_directory,
            &input_file_path,
            &base_directory,
            false,
            "_suffix",
            OutputFormat::Webp,
        )
        .unwrap();
        assert_eq!(result, (Path::new("/output").to_path_buf(), "test_suffix.webp".into()));

        // Test case 8: input file with OutputFormat::Tiff
        let result = compute_output_full_path(
            &output_directory,
            &input_file_path,
            &base_directory,
            false,
            "_suffix",
            OutputFormat::Tiff,
        )
        .unwrap();
        assert_eq!(result, (Path::new("/output").to_path_buf(), "test_suffix.tiff".into()));
    }

    #[cfg(target_os = "windows")]
    #[test]
    fn test_compute_output_full_path() {
        let output_directory = PathBuf::from(r"C:\output");
        let base_directory = PathBuf::from(r"C:\base");

        // Test case 1: keep_structure = true
        let input_file_path = PathBuf::from(r"C:\base\folder\test.jpg");
        let result = compute_output_full_path(
            &output_directory,
            &input_file_path,
            &base_directory,
            true,
            "_suffix",
            OutputFormat::Original,
        )
        .unwrap();
        assert_eq!(
            result,
            (Path::new(r"C:\output\folder").to_path_buf(), "test_suffix.jpg".into())
        );

        // Test case 2: keep_structure = false
        let result = compute_output_full_path(
            &output_directory,
            &input_file_path,
            &base_directory,
            false,
            "_suffix",
            OutputFormat::Original,
        )
        .unwrap();
        assert_eq!(
            result,
            (Path::new(r"C:\output").to_path_buf(), "test_suffix.jpg".into())
        );

        // Test case 3: input file without extension
        let input_file_path = PathBuf::from(r"C:\base\folder\test");
        let result = compute_output_full_path(
            &output_directory,
            &input_file_path,
            &base_directory,
            false,
            "_suffix",
            OutputFormat::Original,
        )
        .unwrap();
        assert_eq!(result, (Path::new(r"C:\output").to_path_buf(), "test_suffix".into()));

        // Test case 4: input file with different base directory
        let input_file_path = PathBuf::from(r"C:\different_base\folder\test.jpg");
        let result = compute_output_full_path(
            &output_directory,
            &input_file_path,
            &base_directory,
            false,
            "_suffix",
            OutputFormat::Original,
        )
        .unwrap();
        assert_eq!(
            result,
            (Path::new(r"C:\output").to_path_buf(), "test_suffix.jpg".into())
        );

        // Test case 5: input file with OutputFormat::Jpeg
        let result = compute_output_full_path(
            &output_directory,
            &input_file_path,
            &base_directory,
            false,
            "_suffix",
            OutputFormat::Jpeg,
        )
        .unwrap();
        assert_eq!(
            result,
            (Path::new(r"C:\output").to_path_buf(), "test_suffix.jpg".into())
        );

        // Test case 6: input file with OutputFormat::Png
        let result = compute_output_full_path(
            &output_directory,
            &input_file_path,
            &base_directory,
            false,
            "_suffix",
            OutputFormat::Png,
        )
        .unwrap();
        assert_eq!(
            result,
            (Path::new(r"C:\output").to_path_buf(), "test_suffix.png".into())
        );

        // Test case 7: input file with OutputFormat::Webp
        let result = compute_output_full_path(
            &output_directory,
            &input_file_path,
            &base_directory,
            false,
            "_suffix",
            OutputFormat::Webp,
        )
        .unwrap();
        assert_eq!(
            result,
            (Path::new(r"C:\output").to_path_buf(), "test_suffix.webp".into())
        );

        // Test case 8: input file with OutputFormat::Tiff
        let result = compute_output_full_path(
            &output_directory,
            &input_file_path,
            &base_directory,
            false,
            "_suffix",
            OutputFormat::Tiff,
        )
        .unwrap();
        assert_eq!(
            result,
            (Path::new(r"C:\output").to_path_buf(), "test_suffix.tiff".into())
        );
    }

    #[test]
    fn test_perform_compression() {
        let input_files = vec![
            absolute(PathBuf::from("samples/j0.JPG")).unwrap(),
            absolute(PathBuf::from("samples/p0.png")).unwrap(),
            absolute(PathBuf::from("samples/w0.webp")).unwrap(),
            absolute(PathBuf::from("samples/t0.tif")).unwrap(),
        ];

        let mut options = setup_options();
        options.base_path = absolute(PathBuf::from("samples")).unwrap();
        let progress_bar = ProgressBar::new(input_files.len() as u64);
        progress_bar.set_draw_target(ProgressDrawTarget::hidden());
        let temp_dir = tempdir().unwrap().path().to_path_buf();
        options.output_folder = Some(temp_dir.clone());

        let mut results = start_compression(&input_files, &options, &progress_bar, false);
        assert_eq!(results.len(), 4);
        assert!(results.iter().all(|r| matches!(r.status, CompressionStatus::Success)));
        assert!(results.iter().all(|r| fs::exists(&r.output_path).unwrap_or(false)));
        assert_eq!(PathBuf::from(&results[0].output_path), temp_dir.join("j0.JPG"));
        assert_eq!(PathBuf::from(&results[1].output_path), temp_dir.join("p0.png"));
        assert_eq!(PathBuf::from(&results[2].output_path), temp_dir.join("w0.webp"));
        assert_eq!(PathBuf::from(&results[3].output_path), temp_dir.join("t0.tif"));

        let input_files = vec![
            absolute(PathBuf::from("samples/j0.JPG")).unwrap(),
            absolute(PathBuf::from("samples/p0.png")).unwrap(),
            absolute(PathBuf::from("samples/w0.webp")).unwrap(),
            absolute(PathBuf::from("samples/t0.tif")).unwrap(),
            absolute(PathBuf::from("samples/level_1_0/level_2_0/p2.png")).unwrap(),
            absolute(PathBuf::from("samples/level_1_0/j1.jpg")).unwrap(),
            absolute(PathBuf::from("samples/level_1_1/w1.webp")).unwrap(),
        ];

        let temp_dir = tempdir().unwrap().path().to_path_buf();
        options.output_folder = Some(temp_dir.clone());
        options.keep_structure = true;
        results = start_compression(&input_files, &options, &progress_bar, false);
        assert_eq!(results.len(), 7);
        assert!(results.iter().all(|r| matches!(r.status, CompressionStatus::Success)));
        assert!(results.iter().all(|r| fs::exists(&r.output_path).unwrap_or(false)));
        assert_eq!(PathBuf::from(&results[0].output_path), temp_dir.join("j0.JPG"));
        assert_eq!(PathBuf::from(&results[1].output_path), temp_dir.join("p0.png"));
        assert_eq!(PathBuf::from(&results[2].output_path), temp_dir.join("w0.webp"));
        assert_eq!(PathBuf::from(&results[3].output_path), temp_dir.join("t0.tif"));
        assert_eq!(
            PathBuf::from(&results[4].output_path),
            temp_dir.join("level_1_0/level_2_0/p2.png")
        );
        assert_eq!(
            PathBuf::from(&results[5].output_path),
            temp_dir.join("level_1_0/j1.jpg")
        );
        assert_eq!(
            PathBuf::from(&results[6].output_path),
            temp_dir.join("level_1_1/w1.webp")
        );

        options.quality = Some(100);

        options.overwrite_policy = OverwritePolicy::Never;
        results = start_compression(&input_files, &options, &progress_bar, false);
        assert!(results.iter().all(|r| matches!(r.status, CompressionStatus::Skipped)));
        assert!(results.iter().all(|r| fs::exists(&r.output_path).unwrap_or(false)));

        options.quality = Some(100);
        options.overwrite_policy = OverwritePolicy::Bigger;
        results = start_compression(&input_files, &options, &progress_bar, false);
        assert!(results.iter().all(|r| matches!(r.status, CompressionStatus::Skipped)));
        assert!(results.iter().all(|r| fs::exists(&r.output_path).unwrap_or(false)));

        options.quality = Some(100);
        options.overwrite_policy = OverwritePolicy::All;
        results = start_compression(&input_files, &options, &progress_bar, true);
        assert!(results.iter().all(|r| matches!(r.status, CompressionStatus::Success)));
        assert!(results.iter().all(|r| fs::exists(&r.output_path).unwrap_or(false)));

        options.quality = Some(100);
        options.png_opt_level = 6;
        options.lossless = true;
        options.overwrite_policy = OverwritePolicy::All;
        results = start_compression(&input_files, &options, &progress_bar, true);
        assert!(results.iter().all(|r| matches!(r.status, CompressionStatus::Success)));
        assert!(results.iter().all(|r| fs::exists(&r.output_path).unwrap_or(false)));

        options.quality = Some(80);
        options.keep_dates = true;
        results = start_compression(&input_files, &options, &progress_bar, false);

        assert!(results.iter().all(|r| matches!(r.status, CompressionStatus::Success)));
        assert!(results.iter().all(|r| {
            let original_metadata = fs::metadata(&r.original_path).unwrap();
            let o_mtime = original_metadata
                .modified()
                .unwrap()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs();
            let o_ltime = original_metadata
                .accessed()
                .unwrap()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs();

            let compressed_metadata = fs::metadata(&r.output_path).unwrap();
            let c_mtime = compressed_metadata
                .modified()
                .unwrap()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs();
            let c_ltime = compressed_metadata
                .accessed()
                .unwrap()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs();
            o_mtime == c_mtime && (o_ltime + 10) >= c_ltime
        }));
        assert!(results.iter().all(|r| fs::exists(&r.output_path).unwrap_or(false)));
    }

    fn setup_options() -> CompressionOptions {
        CompressionOptions {
            quality: Some(80),
            lossless: false,
            output_folder: None,
            same_folder_as_input: false,
            overwrite_policy: OverwritePolicy::All,
            format: OutputFormat::Original,
            suffix: None,
            keep_structure: false,
            width: None,
            height: None,
            long_edge: None,
            short_edge: None,
            max_size: None,
            keep_dates: false,
            exif: true,
            png_opt_level: 0,
            jpeg_chroma_subsampling: ChromaSubsampling::Auto,
            jpeg_baseline: false,
            zopfli: false,
            base_path: PathBuf::new(),
        }
    }
}
