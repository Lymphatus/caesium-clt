use std::path::PathBuf;
use std::time::Duration;

use indicatif::ProgressBar;
use indicatif::ProgressStyle;
use walkdir::WalkDir;

pub fn is_filetype_supported(path: &PathBuf) -> bool {
    let file_path = match path.to_str() {
        None => return false,
        Some(p) => p
    };
    match infer::get_from_path(file_path) {
        Ok(v) => match v {
            None => false,
            Some(ft) => matches!(ft.mime_type(), "image/jpeg" | "image/png" | "image/gif" | "image/webp"),
        },
        Err(_) => false
    }
}


fn is_valid(entry: &PathBuf) -> bool {
    entry.exists() && entry.is_file() && is_filetype_supported(entry)
}

pub fn scanfiles(args: Vec<PathBuf>, recursive: bool) -> (PathBuf, Vec<PathBuf>) {
    let mut files: Vec<PathBuf> = vec![];
    let mut base_path = PathBuf::new();

    let progress_bar = init_progress_bar();

    for input in args.into_iter() {
        if input.exists() && input.is_dir() {
            let mut walk_dir = WalkDir::new(input);
            if !recursive {
                walk_dir = walk_dir.max_depth(1);
            }
            for entry in walk_dir.into_iter().filter_map(|e| e.ok()) {
                let path = entry.into_path();
                if is_valid(&path) {
                    if let Ok(ap) = path.canonicalize() {
                        base_path = compute_base_folder(&base_path, &ap);
                        files.push(ap);
                    }
                }
            }
        } else if is_valid(&input) {
            if let Ok(ap) = input.canonicalize() {
                base_path = compute_base_folder(&base_path, &ap);
                files.push(ap);
            }
        }

        progress_bar.tick();
    }

    progress_bar.finish_and_clear();

    (base_path, files)
}

fn compute_base_folder(base_folder: &PathBuf, new_path: &PathBuf) -> PathBuf {
    if base_folder.parent().is_none() {
        return if new_path.is_dir() {
            new_path.clone()
        } else {
            new_path.parent().unwrap_or(&*PathBuf::from("/")).to_path_buf()
        };
    }
    let mut folder = PathBuf::new();
    let mut new_path_folder = new_path.clone();
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
        return PathBuf::from("/");
    }

    folder
}


fn init_progress_bar() -> ProgressBar {
    let progress_bar = ProgressBar::new_spinner();
    let style = match ProgressStyle::default_spinner()
        .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"])
        .template("{spinner:.cyan} {msg}") {
        Ok(s) => s,
        Err(_) => ProgressStyle::default_spinner()
    };

    progress_bar.set_message("Collecting files...");
    progress_bar.enable_steady_tick(Duration::from_millis(80));
    progress_bar.set_style(style);

    progress_bar
}