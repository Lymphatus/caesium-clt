use std::path::{absolute, Path, PathBuf};
use std::time::Duration;

use indicatif::ProgressStyle;
use indicatif::{ProgressBar, ProgressDrawTarget, ProgressIterator};
use walkdir::WalkDir;

fn has_supported_extension(path: &Path) -> bool {
    match path.extension().and_then(|e| e.to_str()) {
        Some(ext) => {
            let ext_lower = ext.to_lowercase();
            matches!(ext_lower.as_str(), "jpg" | "jpeg" | "png" | "webp" | "gif")
        }
        None => false,
    }
}

fn read_first_bytes(path: &Path, count: usize) -> Option<Vec<u8>> {
    use std::fs::File;
    use std::io::Read;

    let mut file = File::open(path).ok()?;
    let mut buffer = vec![0; count];
    match file.read_exact(&mut buffer) {
        Ok(_) => Some(buffer),
        Err(_) => None,
    }
}

fn is_filetype_supported(path: &Path) -> bool {
    let buffer = match read_first_bytes(path, 16) {
        Some(b) => b,
        None => return false,
    };

    infer::image::is_jpeg(&buffer)
        || infer::image::is_png(&buffer)
        || infer::image::is_webp(&buffer)
        || infer::image::is_gif(&buffer)
}

fn is_valid_file(path: &Path, check_extension_only: bool) -> bool {
    if check_extension_only {
        return has_supported_extension(path);
    }

    is_filetype_supported(path)
}

pub fn scan_files(
    args: &[String],
    recursive: bool,
    quiet: bool,
    check_extension_only: bool,
) -> (Option<PathBuf>, Vec<PathBuf>) {
    if args.is_empty() {
        return (None, vec![]);
    }
    let mut files: Vec<PathBuf> = vec![];
    let mut base_path: Option<PathBuf> = None;
    let progress_bar = init_progress_bar(quiet);

    for path in args.iter().progress_with(progress_bar) {
        let input = PathBuf::from(path);
        if input.exists() && input.is_dir() {
            let mut walk_dir = WalkDir::new(&input);
            if !recursive {
                walk_dir = walk_dir.max_depth(1);
            }
            for entry in walk_dir.into_iter().filter_map(|e| e.ok()) {
                if entry.file_type().is_file() {
                    let path = entry.into_path();
                    if is_valid_file(&path, check_extension_only) {
                        base_path = match compute_base_path(&path, base_path.clone()) {
                            Some(p) => Some(p),
                            None => continue,
                        };
                        files.push(path);
                    }
                }
            }
        } else if input.is_file() && is_valid_file(&input, check_extension_only) {
            base_path = match compute_base_path(&input, base_path.clone()) {
                Some(p) => Some(p),
                None => continue,
            };
            files.push(input);
        }
    }

    (base_path, files)
}

fn compute_base_path(path: &Path, base_path: Option<PathBuf>) -> Option<PathBuf> {
    if !path.exists() {
        return None;
    }

    if let Ok(ap) = absolute(path) {
        let bp = compute_base_folder(base_path, &ap)?;
        return Some(bp);
    }

    None
}

fn compute_base_folder(bf: Option<PathBuf>, new_path: &Path) -> Option<PathBuf> {
    if bf.is_none() && new_path.parent().is_none() {
        return None;
    }

    if bf.is_none() && new_path.parent().is_some() {
        return Some(new_path.parent()?.to_path_buf());
    }

    let base_folder = bf.unwrap();
    if base_folder.parent().is_none() {
        return Some(base_folder.to_path_buf());
    }

    let mut folder = PathBuf::new();
    let mut new_path_folder = new_path.to_path_buf();
    if new_path.is_file() {
        new_path_folder = new_path.parent().unwrap_or(&*PathBuf::new()).to_path_buf();
    }
    for (i, component) in base_folder.iter().enumerate() {
        if let Some(new_path_component) = new_path_folder.iter().nth(i) {
            if new_path_component == component {
                folder.push(component);
            } else {
                break;
            }
        } else {
            break;
        }
    }

    if folder.parent().is_none() {
        return Some(folder);
    }

    Some(folder)
}

fn init_progress_bar(quiet: bool) -> ProgressBar {
    let progress_bar = ProgressBar::new_spinner();
    if quiet {
        progress_bar.set_draw_target(ProgressDrawTarget::hidden());
        return progress_bar;
    }
    let style = ProgressStyle::default_spinner()
        .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"])
        .template("{spinner:.cyan} {msg}")
        .unwrap_or_else(|_| ProgressStyle::default_spinner());

    progress_bar.set_message("Collecting files...");
    progress_bar.enable_steady_tick(Duration::from_millis(100));
    progress_bar.set_style(style);

    progress_bar
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::RgbImage;
    use std::fs::File;
    use std::io::{Cursor, Write};
    use std::path::Path;
    use tempfile::NamedTempFile;

    #[test]
    fn test_has_supported_extension() {
        assert!(has_supported_extension(Path::new("test.jpg")));
        assert!(has_supported_extension(Path::new("test.png")));
        assert!(has_supported_extension(Path::new("test.webp")));
        assert!(has_supported_extension(Path::new("test.gif")));

        assert!(!has_supported_extension(Path::new("test.tiff")));
        assert!(!has_supported_extension(Path::new("test.tif")));
        assert!(!has_supported_extension(Path::new("test.txt")));
        assert!(!has_supported_extension(Path::new("test.avif")));
        assert!(!has_supported_extension(Path::new("test")));
    }

    #[test]
    fn test_is_filetype_supported() {
        let supported_file_types = [
            image::ImageFormat::Jpeg,
            image::ImageFormat::Png,
            image::ImageFormat::WebP,
            image::ImageFormat::Gif,
        ];

        for supported_file in supported_file_types {
            let mut temp_file = NamedTempFile::new().unwrap();
            let rgb_image = RgbImage::new(1, 1);
            let mut bytes: Vec<u8> = Vec::new();
            rgb_image
                .write_to(&mut Cursor::new(&mut bytes), supported_file)
                .unwrap();
            temp_file.write_all(bytes.as_slice()).unwrap();

            assert!(is_filetype_supported(temp_file.path()));
        }

        let unsupported_file_types = [image::ImageFormat::Tiff, image::ImageFormat::Avif];

        for unsupported_file in unsupported_file_types {
            let mut temp_file = NamedTempFile::new().unwrap();
            let rgb_image = RgbImage::new(1, 1);
            let mut bytes: Vec<u8> = Vec::new();
            rgb_image
                .write_to(&mut Cursor::new(&mut bytes), unsupported_file)
                .unwrap();
            temp_file.write_all(bytes.as_slice()).unwrap();

            assert!(!is_filetype_supported(temp_file.path()));
        }
    }

    #[test]
    fn test_compute_base_folder_with_files() {
        let base_folder = Path::new("/base/folder");
        let new_path = Path::new("/base/folder/subfolder/file.jpg");

        let result =
            compute_base_folder(Some(PathBuf::from(base_folder)), new_path).expect("Failed to compute base folder");
        assert_eq!(result, Path::new("/base/folder"));

        let base_folder = Path::new("/base/folder/subfolder/another/folder");
        let new_path = Path::new("/base/folder/subfolder/file.jpg");

        let result =
            compute_base_folder(Some(PathBuf::from(base_folder)), new_path).expect("Failed to compute base folder");
        assert_eq!(result, Path::new("/base/folder/subfolder"));

        let base_folder = Path::new("/base/folder/subfolder/another/folder");
        let new_path = Path::new("/file.jpg");

        let result =
            compute_base_folder(Some(PathBuf::from(base_folder)), new_path).expect("Failed to compute base folder");
        assert_eq!(result, Path::new("/"));

        let base_folder = Path::new("/");
        let new_path = Path::new("/base/folder/subfolder/file.jpg");

        let result =
            compute_base_folder(Some(PathBuf::from(base_folder)), new_path).expect("Failed to compute base folder");
        assert_eq!(result, Path::new("/"));

        let base_folder = Path::new("/");
        let new_path = Path::new("/file.jpg");

        let result =
            compute_base_folder(Some(PathBuf::from(base_folder)), new_path).expect("Failed to compute base folder");
        assert_eq!(result, Path::new("/"));

        let new_path = Path::new("/temp/file.jpg");

        let result = compute_base_folder(None, new_path).expect("Failed to compute base folder");
        assert_eq!(result, Path::new("/temp"));

        let base_folder = Path::new("C:\\Pictures\\image.png");
        let new_path = Path::new("D:\\temp\\file.jpg");

        let result =
            compute_base_folder(Some(PathBuf::from(base_folder)), new_path).expect("Failed to compute base folder");
        assert_eq!(result, PathBuf::new());
    }

    #[test]
    fn test_compute_base_folder_with_folders() {
        let base_folder = Path::new("/base/folder");
        let new_path = Path::new("/base/folder/subfolder");

        let result =
            compute_base_folder(Some(PathBuf::from(base_folder)), new_path).expect("Failed to compute base folder");
        assert_eq!(result, Path::new("/base/folder"));

        let base_folder = Path::new("/base/folder/subfolder/another/folder");
        let new_path = Path::new("/base/folder/subfolder");

        let result =
            compute_base_folder(Some(PathBuf::from(base_folder)), new_path).expect("Failed to compute base folder");
        assert_eq!(result, Path::new("/base/folder/subfolder"));

        let base_folder = Path::new("/base/folder/subfolder/another/folder");
        let new_path = Path::new("/");

        let result =
            compute_base_folder(Some(PathBuf::from(base_folder)), new_path).expect("Failed to compute base folder");
        assert_eq!(result, Path::new("/"));

        let base_folder = Path::new("/");
        let new_path = Path::new("/base/folder/subfolder");

        let result =
            compute_base_folder(Some(PathBuf::from(base_folder)), new_path).expect("Failed to compute base folder");
        assert_eq!(result, Path::new("/"));

        let base_folder = Path::new("/");
        let new_path = Path::new("/");

        let result =
            compute_base_folder(Some(PathBuf::from(base_folder)), new_path).expect("Failed to compute base folder");
        assert_eq!(result, Path::new("/"));
    }

    #[test]
    fn test_compute_base_path() {
        // Test with a valid path
        let path = Path::new(".");
        let base_path = PathBuf::new();
        let result = compute_base_path(path, Some(base_path));
        assert!(result.is_some());

        // Test with an invalid path
        let path = Path::new("/non/existent/path");
        let base_path = PathBuf::new();
        let result = compute_base_path(path, Some(base_path));
        assert!(result.is_none());
    }

    #[test]
    fn test_scan_files() {
        // Create a temporary directory with some image files
        let temp_dir = tempfile::tempdir().unwrap();
        let temp_path = temp_dir.path();

        // Create a JPEG file
        let jpeg_path = temp_path.join("test.jpg");
        let mut jpeg_file = File::create(&jpeg_path).unwrap();
        let rgb_image = RgbImage::new(1, 1);
        let mut bytes: Vec<u8> = Vec::new();
        rgb_image
            .write_to(&mut Cursor::new(&mut bytes), image::ImageFormat::Jpeg)
            .unwrap();
        jpeg_file.write_all(bytes.as_slice()).unwrap();

        // Create a PNG file
        let png_path = temp_path.join("test.png");
        let mut png_file = File::create(&png_path).unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        rgb_image
            .write_to(&mut Cursor::new(&mut bytes), image::ImageFormat::Png)
            .unwrap();
        png_file.write_all(bytes.as_slice()).unwrap();

        // Create an extensionless file containing a valid image
        let extless_path = temp_path.join("test_no_ext");
        let mut extless_file = File::create(&extless_path).unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        rgb_image
            .write_to(&mut Cursor::new(&mut bytes), image::ImageFormat::Jpeg)
            .unwrap();
        extless_file.write_all(bytes.as_slice()).unwrap();

        // Create a text file (unsupported)
        let txt_path = temp_path.join("test.txt");
        let mut txt_file = File::create(&txt_path).unwrap();
        txt_file.write_all(b"This is a text file").unwrap();

        // Test with recursive = false, quiet = true, check_extension_only = false
        let args = vec![temp_path.to_string_lossy().to_string()];
        let (base_path, files) = scan_files(&args, false, true, false);
        assert!(!base_path.unwrap().as_os_str().is_empty());
        assert_eq!(files.len(), 3); // Should find 3 image files (jpg, png, and the extensionless one)

        // Test with recursive = false, quiet = true, check_extension_only = true
        let args = vec![temp_path.to_string_lossy().to_string()];
        let (base_path, files) = scan_files(&args, false, true, true);
        assert!(!base_path.unwrap().as_os_str().is_empty());
        assert_eq!(files.len(), 2); // Should find ONLY the 2 files with extensions

        // Test with empty args
        let args: Vec<String> = vec![];
        let (base_path, files) = scan_files(&args, false, true, false);
        assert!(base_path.is_none());
        assert_eq!(files.len(), 0);

        // Test with a non-existent path
        let args = vec!["/non/existent/path".to_string()];
        let (base_path, files) = scan_files(&args, false, true, false);
        assert!(base_path.is_none());
        assert_eq!(files.len(), 0);

        // Test with a file path directly
        let args = vec![jpeg_path.to_string_lossy().to_string()];
        let (base_path, files) = scan_files(&args, false, true, false);
        assert!(!base_path.unwrap().as_os_str().is_empty());
        assert_eq!(files.len(), 1);
    }

    #[test]
    fn test_init_progress_bar() {
        // Test with quiet = true
        let progress_bar = init_progress_bar(true);
        assert!(progress_bar.is_hidden());

        // Test with quiet = false
        // let progress_bar = init_progress_bar(false);
        // assert_eq!(progress_bar.is_hidden(), false);
    }
}
