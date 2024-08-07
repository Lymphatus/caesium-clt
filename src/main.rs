use std::fs;
use std::path::Path;
use std::sync::{Arc, Mutex};

use caesium::SupportedFileTypes;
use filetime::{FileTime, set_file_times};
use human_bytes::human_bytes;
use indicatif::ProgressBar;
use indicatif::ProgressDrawTarget;
use indicatif::ProgressStyle;
use rand::{Rng, thread_rng};
use rand::distributions::Alphanumeric;
use rayon::prelude::*;

use crate::logger::ErrorLevel::{Error, Log, Notice, Warning};
use crate::logger::log;
use crate::options::OverwritePolicy;

mod scanfiles;
mod options;
mod logger;

struct CompressionResult {
    pub path: String,
    pub output_path: String,
    pub original_size: u64,
    pub compressed_size: u64,
    pub error: String,
    pub result: bool,
}

struct OutputFormat {
    pub file_type: SupportedFileTypes,
    pub extension: String,
}

fn main() {
    let opt = options::get_opts();
    let mut verbose = opt.verbose;
    let args = opt.files;
    let dry_run = opt.dry_run;
    let output_dir = opt.output;
    let output_format = map_output_format(opt.output_format);
    let convert = output_format.file_type != SupportedFileTypes::Unkn;
    let keep_dates = opt.keep_dates;

    let compress_by_size = opt.max_size.is_some();

    if opt.quiet {
        verbose = 0;
    }
    let cpus = if opt.threads > 0 {
        std::cmp::min(num_cpus::get(), opt.threads as usize)
    } else {
        num_cpus::get()
    };
    rayon::ThreadPoolBuilder::new().num_threads(cpus).build_global().unwrap_or_default();

    if dry_run {
        log("Running in dry run mode", 0, Notice, verbose);
    } else {
        match fs::create_dir_all(output_dir.clone()) {
            Ok(_) => {}
            Err(_) => log("Cannot create output path. Check your permissions.", 201, Error, verbose)
        }
    }

    let (base_path, files) = scanfiles::scanfiles(args, opt.recursive);

    let mut compression_parameters = caesium::initialize_parameters();

    if opt.quality.is_some() {
        let quality = opt.quality.unwrap();
        if quality == 0 {
            compression_parameters.optimize = true;
            compression_parameters.png.force_zopfli = opt.zopfli;
        } else {
            compression_parameters.jpeg.quality = quality;
            compression_parameters.png.quality = quality;
            compression_parameters.gif.quality = quality;
            compression_parameters.webp.quality = quality;
        }
    }

    compression_parameters.keep_metadata = opt.exif;

    if opt.width > 0 {
        compression_parameters.width = opt.width;
    }

    if opt.height > 0 {
        compression_parameters.height = opt.height;
    }

    let overwrite_policy = opt.overwrite;
    let keep_structure = opt.keep_structure;

    if opt.zopfli {
        log("Using zopfli may take a very long time, especially with large images!", 0, Notice, verbose);
    }

    let progress_bar = setup_progress_bar(files.len() as u64, verbose);
    progress_bar.set_message("Compressing...");

    let results = Arc::new(Mutex::new(Vec::new()));
    files.par_iter().for_each(|input_file| {
        let input_file_metadata = fs::metadata(input_file);
        let (input_size, input_mtime, input_atime) = match input_file_metadata {
            Ok(s) => (s.len(), FileTime::from_last_modification_time(&s), FileTime::from_last_access_time(&s)),
            Err(e) => {
                let error_message = format!("Cannot get file size for {}, Error: {}", input_file.display(), e);
                log(error_message.as_str(), 202, Warning, verbose);
                (0, FileTime::now(), FileTime::now())
            }
        };

        let mut compression_result = CompressionResult {
            path: input_file.display().to_string(),
            output_path: "".to_string(),
            original_size: input_size,
            compressed_size: 0,
            error: "Unknown".to_string(),
            result: false,
        };

        let filename = if keep_structure {
            input_file.strip_prefix(base_path.clone()).unwrap_or_else(|_| Path::new("")).as_os_str()
        } else {
            input_file.file_name().unwrap_or_default()
        };

        if filename.is_empty() {
            compression_result.error = "Cannot retrieve filename for {}. Skipping.".to_string();
            results.lock().unwrap().push(compression_result);
            return;
        }
        let filename_str = match filename.to_str() {
            None => {
                compression_result.error = "Cannot convert filename for {}. Skipping.".to_string();
                results.lock().unwrap().push(compression_result);
                return;
            }
            Some(fs) => fs
        };


        let random_suffix: String = (&mut thread_rng()).sample_iter(Alphanumeric)
            .take(8)
            .map(char::from)
            .collect();
        let random_suffixed_name = format!("{}.{}", filename_str, random_suffix);
        let mut final_output_full_path = output_dir.clone().join(filename);
        if convert {
            final_output_full_path.set_extension(output_format.extension.clone());
        }

        let output_full_path = output_dir.clone().join(random_suffixed_name);
        let output_full_dir = output_full_path.parent().unwrap_or_else(|| Path::new("/"));
        compression_result.output_path = final_output_full_path.display().to_string();
        if !output_full_dir.exists() {
            match fs::create_dir_all(output_full_dir) {
                Ok(_) => {}
                Err(e) => {
                    compression_result.error = format!("Cannot create output directory. Error: {}.", e);
                    results.lock().unwrap().push(compression_result);
                    return;
                }
            };
        }
        if !matches!(overwrite_policy, OverwritePolicy::All) && final_output_full_path.exists() {
            if let OverwritePolicy::None = overwrite_policy { return; }
        }
        let input_full_path = input_file.to_str().unwrap();
        let output_full_path_str = match output_full_path.to_str() {
            None => {
                compression_result.error = "Cannot convert output_full_path. Skipping.".to_string();
                return;
            }
            Some(ofp) => ofp
        };
        if !dry_run {
            let result = if convert {
                caesium::convert(input_full_path.to_string(), output_full_path_str.to_string(), &compression_parameters, output_format.file_type)
            } else if compress_by_size {
                caesium::compress_to_size(input_full_path.to_string(), output_full_path_str.to_string(), &mut compression_parameters.clone(), opt.max_size.unwrap() as usize, true)
            } else {
                caesium::compress(input_full_path.to_string(), output_full_path_str.to_string(), &compression_parameters)
            };

            match result {
                Ok(_) => {
                    compression_result.result = true;
                    let output_metadata = fs::metadata(output_full_path.clone());
                    let output_size = if let Ok(..) = output_metadata {
                        output_metadata.unwrap().len()
                    } else {
                        0
                    };
                    let mut final_output_size = output_size;
                    if matches!(overwrite_policy, OverwritePolicy::Bigger) && final_output_full_path.exists() {
                        let existing_file_metadata = fs::metadata(final_output_full_path.clone());
                        let existing_file_size = if let Ok(..) = existing_file_metadata {
                            existing_file_metadata.unwrap().len()
                        } else {
                            0
                        };
                        if output_size >= existing_file_size {
                            match fs::remove_file(output_full_path) {
                                Ok(_) => {}
                                Err(e) => {
                                    compression_result.error = format!("Cannot remove existing file. Error: {}.", e);
                                    compression_result.result = false;
                                }
                            };
                            final_output_size = existing_file_size;
                        } else {
                            match fs::rename(output_full_path, final_output_full_path.clone()) {
                                Ok(_) => {}
                                Err(e) => {
                                    compression_result.error = format!("Cannot rename existing file. Error: {}.", e);
                                    compression_result.result = false;
                                }
                            };
                        }
                    } else {
                        match fs::rename(output_full_path, final_output_full_path.clone()) {
                            Ok(_) => {}
                            Err(e) => {
                                compression_result.error = format!("Cannot rename existing file. Error: {}.", e);
                                compression_result.result = false;
                            }
                        };
                    }
                    compression_result.compressed_size = final_output_size;
                    if compression_result.result && keep_dates {
                        match set_file_times(final_output_full_path, input_atime, input_mtime) {
                            Ok(_) => {}
                            Err(_) => {
                                compression_result.error = "Cannot set original file dates.".into();
                            }
                        }
                    }
                    results.lock().unwrap().push(compression_result);
                }
                Err(e) => {
                    compression_result.error = e.to_string();
                    results.lock().unwrap().push(compression_result);
                }
            }
        } else {
            results.lock().unwrap().push(compression_result)
        }
        progress_bar.inc(1);
    });


    progress_bar.finish_with_message("Compression completed!");

    let mut total_original_size = 0.0;
    let mut total_compressed_size = 0.0;
    let mut total_errors: u32 = 0;
    let mut total_compressed_files = 0;

    results.lock().unwrap().iter().for_each(|result| {
        if result.result {
            total_compressed_size += result.compressed_size as f64;
            if verbose > 1 {
                let message = format!("{} -> {}\n{} -> {} [{:.2}%]",
                                      result.path,
                                      result.output_path,
                                      human_bytes(result.original_size as f64),
                                      human_bytes(result.compressed_size as f64),
                                      (result.compressed_size as f64 - result.original_size as f64) * 100.0 / result.original_size as f64
                );
                log(message.as_str(), 0, Log, verbose);
            }
            total_compressed_files += 1;
        } else {
            total_compressed_size += result.original_size as f64;
            if !dry_run {
                total_errors += 1;

                log(format!("File {} was not compressed. Reason: {}", result.path, result.error).as_str(), 210, Warning, verbose);
            }
        }
        total_original_size += result.original_size as f64;
    });

    let recap_message = format!("\nCompressed {} files ({} errors)\n{} -> {} [{:.2}% | -{}]",
                                total_compressed_files,
                                total_errors,
                                human_bytes(total_original_size),
                                human_bytes(total_compressed_size),
                                (total_compressed_size - total_original_size) * 100.0 / total_original_size,
                                human_bytes(total_original_size - total_compressed_size) //TODO can be positive
    );

    log(recap_message.as_str(), 0, Log, verbose);
}

fn setup_progress_bar(len: u64, verbose: u8) -> ProgressBar {
    let progress_bar = ProgressBar::new(len);
    progress_bar.set_draw_target(ProgressDrawTarget::stdout());
    progress_bar.set_style(ProgressStyle::default_bar()
        .template("[{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos}/{len}\n{msg}")
        .unwrap()
        .progress_chars("#>-"));

    if verbose == 0 {
        progress_bar.set_draw_target(ProgressDrawTarget::hidden());
    }

    progress_bar
}

fn map_output_format(format: String) -> OutputFormat {
    match format.to_lowercase().as_str() {
        "jpg|jpeg" => OutputFormat {
            file_type: SupportedFileTypes::Jpeg,
            extension: format,
        },
        "png" => OutputFormat {
            file_type: SupportedFileTypes::Png,
            extension: format,
        },
        "webp" => OutputFormat {
            file_type: SupportedFileTypes::WebP,
            extension: format,
        },
        "tiff|tif" => OutputFormat {
            file_type: SupportedFileTypes::Tiff,
            extension: format,
        },
        _ => OutputFormat {
            file_type: SupportedFileTypes::Unkn,
            extension: "".to_string(),
        },
    }
}
