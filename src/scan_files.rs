use std::path::{absolute, Path, PathBuf};
use std::time::Duration;

use indicatif::ProgressStyle;
use indicatif::{ProgressBar, ProgressDrawTarget, ProgressIterator};
use rayon::prelude::IntoParallelRefIterator;
use walkdir::WalkDir;

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

pub fn get_file_mime_type(path: &Path) -> Option<String> {
    let buffer = read_first_bytes(path, 16)?;

    match infer::get(&buffer) {
        Some(v) => Option::from(v.mime_type().to_string()),
        None => None,
    }
}

fn is_valid(entry: &Path) -> bool {
    entry.exists() && entry.is_file() && is_filetype_supported(entry)
}

pub fn scan_files(args: &[String], recursive: bool, quiet: bool) -> (PathBuf, Vec<PathBuf>) {
    if args.is_empty() {
        return (PathBuf::new(), vec![]);
    }
    let mut files: Vec<PathBuf> = vec![];
    let mut base_path = PathBuf::new();

    let progress_bar = init_progress_bar(quiet);

    for path in args.iter().progress_with(progress_bar) {
        let input = PathBuf::from(path);
        if input.exists() && input.is_dir() {
            let mut walk_dir = WalkDir::new(input);
            if !recursive {
                walk_dir = walk_dir.max_depth(1);
            }
            for entry in walk_dir.into_iter().filter_map(|e| e.ok()) {
                let path = entry.into_path();
                if is_valid(&path) {
                    base_path = match compute_base_path(&path, &base_path) {
                        Some(p) => p,
                        None => continue,
                    };
                    files.push(path);
                }
            }
        } else if is_valid(&input) {
            base_path = match compute_base_path(&input, &base_path) {
                Some(p) => p,
                None => continue,
            };
            files.push(input);
        }
    }

    (base_path, files)
}

fn compute_base_path(path: &Path, base_path: &Path) -> Option<PathBuf> {
    if !path.exists() {
        return None;
    }

    if let Ok(ap) = absolute(path) {
        let bp = compute_base_folder(base_path, &ap)?;
        return Some(bp);
    }

    None
}

fn compute_base_folder(base_folder: &Path, new_path: &Path) -> Option<PathBuf> {
    if base_folder.as_os_str().is_empty() && new_path.parent().is_some() {
        return Some(new_path.parent()?.to_path_buf());
    }

    if base_folder.parent().is_none() {
        return Some(base_folder.to_path_buf());
    }

    let mut folder = PathBuf::new();
    let mut new_path_folder = new_path.to_path_buf();
    if new_path.is_file() {
        new_path_folder = new_path.parent().unwrap_or(&*PathBuf::from("/")).to_path_buf();
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
    fn test_is_valid() {
        let mut temp_file = NamedTempFile::new().unwrap();
        let rgb_image = RgbImage::new(1, 1);
        let mut bytes: Vec<u8> = Vec::new();
        rgb_image
            .write_to(&mut Cursor::new(&mut bytes), image::ImageFormat::Jpeg)
            .unwrap();
        temp_file.write_all(bytes.as_slice()).unwrap();

        assert!(is_valid(temp_file.path()));
        assert!(!is_valid(temp_file.path().parent().unwrap()));
        assert!(!is_valid(temp_file.path().join("test").as_path()));

        let mut temp_file = NamedTempFile::new().unwrap();
        let rgb_image = RgbImage::new(1, 1);
        let mut bytes: Vec<u8> = Vec::new();
        rgb_image
            .write_to(&mut Cursor::new(&mut bytes), image::ImageFormat::Avif)
            .unwrap();
        temp_file.write_all(bytes.as_slice()).unwrap();
        assert!(!is_valid(temp_file.path()));
    }

    #[test]
    fn test_compute_base_folder_with_files() {
        let base_folder = Path::new("/base/folder");
        let new_path = Path::new("/base/folder/subfolder/file.jpg");

        let result = compute_base_folder(base_folder, new_path).expect("Failed to compute base folder");
        assert_eq!(result, Path::new("/base/folder"));

        let base_folder = Path::new("/base/folder/subfolder/another/folder");
        let new_path = Path::new("/base/folder/subfolder/file.jpg");

        let result = compute_base_folder(base_folder, new_path).expect("Failed to compute base folder");
        assert_eq!(result, Path::new("/base/folder/subfolder"));

        let base_folder = Path::new("/base/folder/subfolder/another/folder");
        let new_path = Path::new("/file.jpg");

        let result = compute_base_folder(base_folder, new_path).expect("Failed to compute base folder");
        assert_eq!(result, Path::new("/"));

        let base_folder = Path::new("/");
        let new_path = Path::new("/base/folder/subfolder/file.jpg");

        let result = compute_base_folder(base_folder, new_path).expect("Failed to compute base folder");
        assert_eq!(result, Path::new("/"));

        let base_folder = Path::new("/");
        let new_path = Path::new("/file.jpg");

        let result = compute_base_folder(base_folder, new_path).expect("Failed to compute base folder");
        assert_eq!(result, Path::new("/"));

        let base_folder = PathBuf::new();
        let new_path = Path::new("/temp/file.jpg");

        let result = compute_base_folder(&base_folder, new_path).expect("Failed to compute base folder");
        assert_eq!(result, Path::new("/temp"));
    }

    #[test]
    fn test_compute_base_folder_with_folders() {
        let base_folder = Path::new("/base/folder");
        let new_path = Path::new("/base/folder/subfolder");

        let result = compute_base_folder(base_folder, new_path).expect("Failed to compute base folder");
        assert_eq!(result, Path::new("/base/folder"));

        let base_folder = Path::new("/base/folder/subfolder/another/folder");
        let new_path = Path::new("/base/folder/subfolder");

        let result = compute_base_folder(base_folder, new_path).expect("Failed to compute base folder");
        assert_eq!(result, Path::new("/base/folder/subfolder"));

        let base_folder = Path::new("/base/folder/subfolder/another/folder");
        let new_path = Path::new("/");

        let result = compute_base_folder(base_folder, new_path).expect("Failed to compute base folder");
        assert_eq!(result, Path::new("/"));

        let base_folder = Path::new("/");
        let new_path = Path::new("/base/folder/subfolder");

        let result = compute_base_folder(base_folder, new_path).expect("Failed to compute base folder");
        assert_eq!(result, Path::new("/"));

        let base_folder = Path::new("/");
        let new_path = Path::new("/");

        let result = compute_base_folder(base_folder, new_path).expect("Failed to compute base folder");
        assert_eq!(result, Path::new("/"));
    }

    #[test]
    fn test_compute_base_path() {
        // Test with a valid path
        let path = Path::new(".");
        let base_path = PathBuf::new();
        let result = compute_base_path(path, &base_path);
        assert!(result.is_some());

        // Test with an invalid path
        let path = Path::new("/non/existent/path");
        let base_path = PathBuf::new();
        let result = compute_base_path(path, &base_path);
        assert!(result.is_none());
    }

    #[test]
    fn test_get_file_mime_type() {
        // Create a temporary JPEG file
        let mut temp_file = NamedTempFile::new().unwrap();
        let rgb_image = RgbImage::new(1, 1);
        let mut bytes: Vec<u8> = Vec::new();
        rgb_image
            .write_to(&mut Cursor::new(&mut bytes), image::ImageFormat::Jpeg)
            .unwrap();
        temp_file.write_all(bytes.as_slice()).unwrap();

        // Test with a JPEG file
        let mime_type = get_file_mime_type(temp_file.path());
        assert!(mime_type.is_some());
        assert_eq!(mime_type.unwrap(), "image/jpeg");

        // Test with a non-existent file
        let mime_type = get_file_mime_type(Path::new("/non/existent/file.jpg"));
        assert!(mime_type.is_none());
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

        // Create a text file (unsupported)
        let txt_path = temp_path.join("test.txt");
        let mut txt_file = File::create(&txt_path).unwrap();
        txt_file.write_all(b"This is a text file").unwrap();

        // Test with recursive = false, quiet = true
        let args = vec![temp_path.to_string_lossy().to_string()];
        let (base_path, files) = scan_files(&args, false, true);
        assert!(!base_path.as_os_str().is_empty());
        assert_eq!(files.len(), 2); // Should find 2 image files

        // Test with empty args
        let args: Vec<String> = vec![];
        let (base_path, files) = scan_files(&args, false, true);
        assert!(base_path.as_os_str().is_empty());
        assert_eq!(files.len(), 0);

        // Test with a non-existent path
        let args = vec!["/non/existent/path".to_string()];
        let (base_path, files) = scan_files(&args, false, true);
        assert!(base_path.as_os_str().is_empty());
        assert_eq!(files.len(), 0);

        // Test with a file path directly
        let args = vec![jpeg_path.to_string_lossy().to_string()];
        let (base_path, files) = scan_files(&args, false, true);
        assert!(!base_path.as_os_str().is_empty());
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
