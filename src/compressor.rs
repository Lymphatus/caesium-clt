use crate::options::{CommandLineArgs, OutputFormat, OverwritePolicy};
use crate::scan_files::get_file_mime_type;
use crate::CompressionStatus;
use caesium::parameters::CSParameters;
use caesium::{compress_in_memory, compress_to_size_in_memory, convert_in_memory, SupportedFileTypes};
use filetime::{set_file_times, FileTime};
use indicatif::{ParallelProgressIterator, ProgressBar};
use rayon::iter::ParallelIterator;
use rayon::prelude::IntoParallelRefIterator;
use std::error::Error;
use std::ffi::OsString;
use std::fs::File;
use std::io::{BufReader, Read, Write};
use std::path::{absolute, Path, PathBuf};
use std::{fs, io};

pub struct CompressionResult {
    pub original_path: String,
    pub output_path: String,
    pub original_size: u64,
    pub compressed_size: u64,
    pub status: CompressionStatus,
    pub message: String,
}

pub fn perform_compression(
    input_files: &Vec<PathBuf>,
    args: &CommandLineArgs,
    base_path: &PathBuf,
    progress_bar: ProgressBar,
) -> Vec<CompressionResult> {
    let needs_resize = args.resize.width.is_some()
        || args.resize.height.is_some()
        || args.resize.long_edge.is_some()
        || args.resize.short_edge.is_some();

    input_files
        .par_iter()
        .progress_with(progress_bar)
        .map(|input_file| {
            let mut compression_result = CompressionResult {
                original_path: input_file.display().to_string(),
                output_path: String::new(),
                original_size: 0,
                compressed_size: 0,
                status: CompressionStatus::Error,
                message: String::new(),
            };

            let input_file_metadata = match input_file.metadata() {
                Ok(m) => m,
                Err(_) => {
                    compression_result.message = "Error reading file metadata".to_string();
                    return compression_result;
                }
            };
            let original_file_size = input_file_metadata.len();
            compression_result.original_size = original_file_size;

            let output_directory = if args.output_destination.same_folder_as_input {
                match input_file.parent() {
                    Some(p) => p,
                    None => {
                        compression_result.message = "Error getting parent directory".to_string();
                        return compression_result;
                    }
                }
            } else {
                match args.output_destination.output.as_ref() {
                    Some(p) => p,
                    None => {
                        compression_result.message = "Error getting output directory".to_string();
                        return compression_result;
                    }
                }
            };

            let (output_directory, filename) = match compute_output_full_path(
                output_directory,
                input_file,
                base_path,
                args.keep_structure,
                args.suffix.as_ref().unwrap_or(&String::new()).as_ref(),
                args.format,
            ) {
                Some(p) => p,
                None => {
                    compression_result.message = "Error computing output path".to_string();
                    return compression_result;
                }
            };
            if !output_directory.exists() {
                match fs::create_dir_all(&output_directory) {
                    Ok(_) => {}
                    Err(_) => {
                        compression_result.message = "Error creating output directory".to_string();
                        return compression_result;
                    }
                }
            }
            let output_full_path = output_directory.join(filename);

            if args.dry_run {
                compression_result.status = CompressionStatus::Success;
                return compression_result;
            };

            let mut compression_parameters = match build_compression_parameters(args, input_file, needs_resize) {
                Ok(p) => p,
                Err(e) => {
                    compression_result.message = format!("Error building compression parameters: {}", e);
                    return compression_result;
                }
            };
            let input_file_buffer = match read_file_to_vec(input_file) {
                Ok(b) => b,
                Err(_) => {
                    compression_result.message = "Error reading input file".to_string();
                    return compression_result;
                }
            };
            let compression = if args.compression.max_size.is_some() {
                compress_to_size_in_memory(
                    input_file_buffer,
                    &mut compression_parameters,
                    args.compression.max_size.unwrap() as usize,
                    true,
                )
            } else if args.format != OutputFormat::Original {
                convert_in_memory(
                    input_file_buffer,
                    &compression_parameters,
                    map_supported_formats(args.format),
                )
            } else {
                compress_in_memory(input_file_buffer, &compression_parameters)
            };

            let compressed_image = match compression {
                Ok(v) => v,
                Err(e) => {
                    compression_result.message = format!("Error compressing file: {}", e);
                    return compression_result;
                }
            };
            compression_result.output_path = output_full_path.display().to_string();
            let output_file_size = compressed_image.len() as u64;

            if output_full_path.exists() {
                match args.overwrite {
                    OverwritePolicy::Never | OverwritePolicy::Bigger => {
                        if (matches!(args.overwrite, OverwritePolicy::Bigger) && output_file_size >= original_file_size)
                            || matches!(args.overwrite, OverwritePolicy::Never)
                        {
                            compression_result.status = CompressionStatus::Skipped;
                            compression_result.compressed_size = original_file_size;
                            compression_result.message =
                                "File already exists, skipped due overwrite policy".to_string();
                            return compression_result;
                        }
                    }
                    _ => {}
                }
            }

            let mut output_file = match File::create(&output_full_path) {
                Ok(f) => f,
                Err(_) => {
                    compression_result.message = "Error creating output file".to_string();
                    return compression_result;
                }
            };
            match output_file.write_all(&compressed_image) {
                Ok(_) => {}
                Err(_) => {
                    compression_result.message = "Error writing output file".to_string();
                    return compression_result;
                }
            };

            if args.keep_dates {
                let (last_modification_time, last_access_time) = (
                    FileTime::from_last_modification_time(&input_file_metadata),
                    FileTime::from_last_access_time(&input_file_metadata),
                );
                match preserve_dates(&output_full_path, last_modification_time, last_access_time) {
                    Ok(_) => {}
                    Err(_) => {
                        compression_result.message = "Error preserving file dates".to_string();
                        return compression_result;
                    }
                }
            }

            compression_result.status = CompressionStatus::Success;
            compression_result.compressed_size = output_file_size;
            compression_result
        })
        .collect()
}

fn build_compression_parameters(args: &CommandLineArgs, input_file: &Path, needs_resize: bool) -> Result<CSParameters, Box<dyn Error>> {
    let mut parameters = CSParameters::new();
    let quality = args.compression.quality.unwrap_or(80) as u32;

    parameters.jpeg.quality = quality;
    parameters.png.quality = quality;
    parameters.webp.quality = quality;
    parameters.gif.quality = quality;

    parameters.keep_metadata = args.exif;

    parameters.png.optimization_level = args.png_opt_level;
    parameters.png.force_zopfli = args.zopfli;

    if needs_resize {
        let mime_type = get_file_mime_type(input_file);
        build_resize_parameters(args, &mut parameters, input_file, mime_type)?;
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
    args: &CommandLineArgs,
    parameters: &mut CSParameters,
    input_file_path: &Path,
    mime_type: Option<String>,
) -> Result<(), Box<dyn Error>> {
    let (width, height) = get_real_resolution(input_file_path, mime_type, args.exif)?;

    if args.resize.width.is_some() {
        parameters.width = args.resize.width.unwrap_or(0);
    } else if args.resize.height.is_some() {
        parameters.height = args.resize.height.unwrap_or(0);
    } else if args.resize.long_edge.is_some() {
        let long_edge = args.resize.long_edge.unwrap_or(0);
        if width > height {
            parameters.width = long_edge;
        } else {
            parameters.height = long_edge;
        }
    } else if args.resize.short_edge.is_some() {
        let short_edge = args.resize.short_edge.unwrap_or(0);
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

fn preserve_dates(output_file: &PathBuf, input_atime: FileTime, input_mtime: FileTime) -> io::Result<()> {
    set_file_times(output_file, input_atime, input_mtime)
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
    use std::path::Path;

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
}
