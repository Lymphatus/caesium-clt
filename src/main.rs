use crate::compressor::{perform_compression, CompressionResult, CompressionStatus};
use crate::options::VerboseLevel::{All, Progress, Quiet};
use crate::options::{CommandLineArgs, VerboseLevel};
use crate::scan_files::scan_files;
use clap::Parser;
use human_bytes::human_bytes;
use indicatif::{ProgressBar, ProgressDrawTarget, ProgressStyle};
use std::num::NonZero;
use std::time::Duration;

mod compressor;
mod options;
mod scan_files;

fn main() {
    let args = CommandLineArgs::parse();

    let threads_number = get_parallelism_count(
        args.threads,
        std::thread::available_parallelism()
            .unwrap_or(NonZero::new(1).unwrap())
            .get(),
    );
    rayon::ThreadPoolBuilder::new()
        .num_threads(threads_number)
        .build_global()
        .unwrap_or_default();

    let quiet = args.quiet || args.verbose == Quiet;
    let verbose = if quiet { Quiet } else { args.verbose };
    let (base_path, input_files) = scan_files(&args.files, args.recursive, quiet);
    let total_files = input_files.len();

    let progress_bar = setup_progress_bar(total_files, verbose);
    let compression_results = perform_compression(&input_files, &args, &base_path, progress_bar);

    write_recap_message(&compression_results, verbose);
}

fn write_recap_message(compression_results: &[CompressionResult], verbose: VerboseLevel) {
    let mut total_original_size = 0;
    let mut total_compressed_size = 0;
    let total_files = compression_results.len();
    let mut total_success = 0;
    let mut total_skipped = 0;
    let mut total_errors = 0;

    for result in compression_results.iter() {
        total_original_size += result.original_size;
        total_compressed_size += result.compressed_size;
        match result.status {
            CompressionStatus::Skipped => total_skipped += 1,
            CompressionStatus::Error => total_errors += 1,
            _ => total_success += 1,
        }

        if verbose > Progress {
            if verbose < All && matches!(result.status, CompressionStatus::Success) {
                continue;
            }
            println!(
                "[{:?}] {} -> {}\n{} -> {} [{:.2}%]",
                result.status,
                result.original_path,
                result.output_path,
                human_bytes(result.original_size as f64),
                human_bytes(result.compressed_size as f64),
                (result.compressed_size as f64 - result.original_size as f64) * 100.0 / result.original_size as f64
            );

            if !result.message.is_empty() {
                println!("{}", result.message);
            }
            println!();
        }
    }

    let total_saved = total_original_size as f64 - total_compressed_size as f64;
    let total_saved_percent = total_saved / total_original_size as f64 * 100.0;

    if verbose > Quiet {
        println!(
            "Compressed {} files ({} success, {} skipped, {} errors)\n{} -> {} [Saved {} | ({:.2}%)]",
            total_files,
            total_success,
            total_skipped,
            total_errors,
            human_bytes(total_original_size as f64),
            human_bytes(total_compressed_size as f64),
            human_bytes(total_saved),
            total_saved_percent * -1.0
        );
    }
}
fn get_parallelism_count(requested_threads: u32, available_threads: usize) -> usize {
    if requested_threads > 0 {
        std::cmp::min(available_threads, requested_threads as usize)
    } else {
        available_threads
    }
}

fn setup_progress_bar(len: usize, verbose: VerboseLevel) -> ProgressBar {
    let progress_bar = ProgressBar::new(len as u64);
    if verbose == Quiet {
        progress_bar.set_draw_target(ProgressDrawTarget::hidden());
    } else {
        progress_bar.set_draw_target(ProgressDrawTarget::stdout());
    }
    progress_bar.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos}/{len}\n{msg}")
            .unwrap_or_else(|_| ProgressStyle::default_bar())
            .progress_chars("#>-"),
    );
    progress_bar.enable_steady_tick(Duration::new(1, 0));
    progress_bar
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_parallelism_count() {
        let result = get_parallelism_count(4, 4);
        assert_eq!(result, 4);

        let result = get_parallelism_count(2, 8);
        assert_eq!(result, 2);

        let result = get_parallelism_count(0, 8);
        assert_eq!(result, 8);

        let result = get_parallelism_count(1, 8);
        assert_eq!(result, 1);

        let result = get_parallelism_count(8, 2);
        assert_eq!(result, 2);

        let result = get_parallelism_count(0, 0);
        assert_eq!(result, 0);
    }
}
