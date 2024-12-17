use std::path::{absolute, Path, PathBuf};
use std::time::Duration;

use indicatif::{ProgressBar, ProgressDrawTarget, ProgressIterator};
use indicatif::ProgressStyle;
use walkdir::WalkDir;

fn is_filetype_supported(path: &Path) -> bool {
    match get_file_mime_type(path) {
        Some(mime_type) => {
            matches!(mime_type.as_str(), "image/jpeg" | "image/png" | "image/webp" | "image/gif")
        }
        None => false,
    }
}

pub fn get_file_mime_type(path: &Path) -> Option<String> {
    match infer::get_from_path(path) {
        Ok(v) => v.map(|ft| ft.mime_type().to_string()),
        Err(_) => None,
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
                    base_path = make_absolute_and_push(&path, base_path, &mut files);
                }
            }
        } else if is_valid(&input) {
            base_path = make_absolute_and_push(&input, base_path, &mut files);
        }
    }
    
    (base_path, files)
}

fn make_absolute_and_push(path: &Path, mut base_path: PathBuf, files: &mut Vec<PathBuf>) -> PathBuf {
    if let Ok(ap) = absolute(path) {
        base_path = compute_base_folder(&base_path, &ap);
        files.push(ap);
    }

    base_path
}

fn compute_base_folder(base_folder: &Path, new_path: &Path) -> PathBuf {
    if base_folder.as_os_str().is_empty() && new_path.parent().is_some() {
        return new_path.parent().unwrap().to_path_buf();
    }
    
    if base_folder.parent().is_none() {
        return base_folder.to_path_buf();
    }
    
    let mut folder = PathBuf::new();
    let mut new_path_folder = new_path.to_path_buf();
    if new_path.is_file() {
        new_path_folder = new_path
            .parent()
            .unwrap_or(&*PathBuf::from("/"))
            .to_path_buf();
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
        return PathBuf::from("/");
    }

    folder
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

        let result = compute_base_folder(base_folder, new_path);
        assert_eq!(result, Path::new("/base/folder"));

        let base_folder = Path::new("/base/folder/subfolder/another/folder");
        let new_path = Path::new("/base/folder/subfolder/file.jpg");

        let result = compute_base_folder(base_folder, new_path);
        assert_eq!(result, Path::new("/base/folder/subfolder"));

        let base_folder = Path::new("/base/folder/subfolder/another/folder");
        let new_path = Path::new("/file.jpg");

        let result = compute_base_folder(base_folder, new_path);
        assert_eq!(result, Path::new("/"));

        let base_folder = Path::new("/");
        let new_path = Path::new("/base/folder/subfolder/file.jpg");

        let result = compute_base_folder(base_folder, new_path);
        assert_eq!(result, Path::new("/"));

        let base_folder = Path::new("/");
        let new_path = Path::new("/file.jpg");

        let result = compute_base_folder(base_folder, new_path);
        assert_eq!(result, Path::new("/"));

        let base_folder = PathBuf::new();
        let new_path = Path::new("/temp/file.jpg");

        let result = compute_base_folder(&base_folder, new_path);
        assert_eq!(result, Path::new("/temp"));
    }

    #[test]
    fn test_compute_base_folder_with_folders() {
        let base_folder = Path::new("/base/folder");
        let new_path = Path::new("/base/folder/subfolder");

        let result = compute_base_folder(base_folder, new_path);
        assert_eq!(result, Path::new("/base/folder"));

        let base_folder = Path::new("/base/folder/subfolder/another/folder");
        let new_path = Path::new("/base/folder/subfolder");

        let result = compute_base_folder(base_folder, new_path);
        assert_eq!(result, Path::new("/base/folder/subfolder"));

        let base_folder = Path::new("/base/folder/subfolder/another/folder");
        let new_path = Path::new("/");

        let result = compute_base_folder(base_folder, new_path);
        assert_eq!(result, Path::new("/"));

        let base_folder = Path::new("/");
        let new_path = Path::new("/base/folder/subfolder");

        let result = compute_base_folder(base_folder, new_path);
        assert_eq!(result, Path::new("/"));

        let base_folder = Path::new("/");
        let new_path = Path::new("/");

        let result = compute_base_folder(base_folder, new_path);
        assert_eq!(result, Path::new("/"));
    }
}
