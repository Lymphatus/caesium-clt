use crate::compressor::{start_compression, CompressionOptions, CompressionResult, CompressionStatus};
use crate::options::{CommandLineArgs, JpegChromaSubsampling};
use crate::scan_files::scan_files;
use bytesize::ByteSize;
use caesium::parameters::ChromaSubsampling;
use clap::Parser;
use colored::Colorize;
use indicatif::{ProgressBar, ProgressDrawTarget, ProgressStyle};
use std::num::NonZero;
use std::path::{Path, PathBuf};
use std::time::Duration;

mod compressor;
mod options;
mod scan_files;

const PROGRESS_UPDATE_INTERVAL: Duration = Duration::from_secs(1);
const FALLBACK_THREAD_COUNT: usize = 1;

fn main() {
    let args = CommandLineArgs::parse();

    if args.files.is_empty() {
        eprintln!("No files to compress");
        return;
    }

    let threads_number = get_parallelism_count(
        args.threads,
        std::thread::available_parallelism()
            .unwrap_or_else(|_| NonZero::new(FALLBACK_THREAD_COUNT).expect("1 is never zero"))
            .get(),
    );

    rayon::ThreadPoolBuilder::new()
        .num_threads(threads_number)
        .build_global()
        .unwrap_or_default();

    let quiet = args.quiet || args.verbose == 0;
    let verbose = if quiet { 0 } else { args.verbose };
    let (base_path, input_files) = scan_files(&args.files, args.recursive, quiet);
    let total_files = input_files.len();

    let progress_bar = setup_progress_bar(total_files, verbose);
    let compression_options = build_compression_options(&args, &base_path);
    let compression_results = start_compression(&input_files, &compression_options, &progress_bar, args.dry_run);
    progress_bar.finish();
    write_recap_message(&compression_results, verbose);
}

fn write_recap_message(compression_results: &[CompressionResult], verbose: u8) {
    if compression_results.is_empty() {
        return;
    }

    let stats = compression_results.iter().fold(
        (0u64, 0u64, 0usize, 0usize, 0usize), // (original_size, compressed_size, success, skipped, errors)
        |(orig, comp, success, skipped, errors), result| {
            let (new_success, new_skipped, new_errors) = match result.status {
                CompressionStatus::Success => (success + 1, skipped, errors),
                CompressionStatus::Skipped => (success, skipped + 1, errors),
                CompressionStatus::Error => (success, skipped, errors + 1),
            };
            (
                orig + result.original_size,
                comp + result.compressed_size,
                new_success,
                new_skipped,
                new_errors,
            )
        },
    );

    let (total_original_size, total_compressed_size, total_success, total_skipped, total_errors) = stats;

    if verbose > 1 {
        for result in compression_results {
            if verbose < 3 && matches!(result.status, CompressionStatus::Success) {
                continue;
            }

            let savings_size = result.original_size as i64 - result.compressed_size as i64;
            let savings_percent = (savings_size as f64 / result.original_size as f64) * 100.0;

            let savings_size_abs = savings_size.unsigned_abs();
            let (formatted_savings_size, formatted_savings_percentage) = if savings_size >= 0 {
                (
                    format!("-{}", ByteSize::b(savings_size_abs)).green(),
                    format!("-{savings_percent:.2}%").green(),
                )
            } else {
                (
                    format!("+{}", ByteSize::b(savings_size_abs)).red(),
                    format!("+{:.2}%", -savings_percent).red(),
                )
            };

            let status_message = match result.status {
                CompressionStatus::Success => "Success".green(),
                CompressionStatus::Skipped => "Skipped".yellow(),
                CompressionStatus::Error => "Error".red(),
            };
            println!(
                "[{}] {} -> {}\n{} -> {} [{} | {}]",
                status_message,
                result.original_path,
                result.output_path,
                ByteSize::b(result.original_size),
                ByteSize::b(result.compressed_size),
                formatted_savings_size,
                formatted_savings_percentage
            );

            if !result.message.is_empty() {
                let message = match result.status {
                    CompressionStatus::Success => result.message.green(),
                    CompressionStatus::Skipped => result.message.yellow(),
                    CompressionStatus::Error => result.message.red(),
                };
                println!("{message}");
            }
            println!();
        }
    }

    if verbose > 0 {
        let total_saved = total_original_size as i64 - total_compressed_size as i64;
        let total_saved_percent = (total_saved as f64 / total_original_size as f64) * 100.0;

        let total_saved_abs = total_saved.unsigned_abs();
        let (formatted_total_saved_size, formatted_total_saved_percentage) = if total_saved >= 0 {
            (
                format!("-{}", ByteSize::b(total_saved_abs)).green(),
                format!("-{total_saved_percent:.2}%").green(),
            )
        } else {
            (
                format!("+{}", ByteSize::b(total_saved_abs)).red(),
                format!("+{:.2}%", -total_saved_percent).red(),
            )
        };

        println!(
            "Compressed {} files ({} success, {} skipped, {} errors)\n{} -> {} [{} | {}]",
            compression_results.len(),
            total_success.to_string().green(),
            total_skipped.to_string().yellow(),
            total_errors.to_string().red(),
            ByteSize::b(total_original_size),
            ByteSize::b(total_compressed_size),
            formatted_total_saved_size,
            formatted_total_saved_percentage
        );
    }
}

fn get_parallelism_count(requested_threads: u32, available_threads: usize) -> usize {
    match requested_threads {
        0 => available_threads,
        n => (n as usize).min(available_threads),
    }
}

fn setup_progress_bar(len: usize, verbose: u8) -> ProgressBar {
    let progress_bar = ProgressBar::new(len as u64);
    if verbose == 0 {
        progress_bar.set_draw_target(ProgressDrawTarget::hidden());
        return progress_bar;
    }

    progress_bar.set_draw_target(ProgressDrawTarget::stdout());

    progress_bar.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos}/{len}\n{msg}")
            .unwrap_or(ProgressStyle::default_bar())
            .progress_chars("#>-"),
    );
    progress_bar.enable_steady_tick(PROGRESS_UPDATE_INTERVAL);
    progress_bar
}

fn build_compression_options(args: &CommandLineArgs, base_path: &Path) -> CompressionOptions {
    CompressionOptions {
        quality: args.compression.quality,
        lossless: args.compression.lossless,
        output_folder: args.output_destination.output.clone(),
        same_folder_as_input: args.output_destination.same_folder_as_input,
        overwrite_policy: args.overwrite,
        format: args.format,
        suffix: args.suffix.clone(),
        keep_structure: args.keep_structure,
        width: args.resize.width,
        height: args.resize.height,
        long_edge: args.resize.long_edge,
        short_edge: args.resize.short_edge,
        max_size: args.compression.max_size,
        keep_dates: args.keep_dates,
        exif: args.exif,
        png_opt_level: args.png_opt_level,
        jpeg_chroma_subsampling: parse_jpeg_chroma_subsampling(args.jpeg_chroma_subsampling),
        jpeg_baseline: args.jpeg_baseline,
        zopfli: args.zopfli,
        base_path: PathBuf::from(base_path),
        no_upscale: args.resize.no_upscale,
    }
}

fn parse_jpeg_chroma_subsampling(arg: JpegChromaSubsampling) -> ChromaSubsampling {
    match arg {
        JpegChromaSubsampling::ChromaSubsampling444 => ChromaSubsampling::CS444,
        JpegChromaSubsampling::ChromaSubsampling422 => ChromaSubsampling::CS422,
        JpegChromaSubsampling::ChromaSubsampling420 => ChromaSubsampling::CS420,
        JpegChromaSubsampling::ChromaSubsampling411 => ChromaSubsampling::CS411,
        _ => ChromaSubsampling::Auto,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::options::{
        Compression, JpegChromaSubsampling, OutputDestination, OutputFormat, OverwritePolicy, Resize,
    };
    use std::path::PathBuf;

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

    #[test]
    fn test_setup_progress_bar() {
        // Test with verbose = 0 (hidden)
        let progress_bar = setup_progress_bar(10, 0);
        assert!(progress_bar.is_hidden());
        assert_eq!(progress_bar.length(), Some(10));

        // Test with verbose > 0 (visible)
        // let progress_bar = setup_progress_bar(20, 1);
        // assert!(!progress_bar.is_hidden());
        // assert_eq!(progress_bar.length(), Some(20));

        // Test with different lengths
        let progress_bar = setup_progress_bar(0, 1);
        assert_eq!(progress_bar.length(), Some(0));
    }

    #[test]
    fn test_parse_jpeg_chroma_subsampling() {
        assert!(parse_jpeg_chroma_subsampling(JpegChromaSubsampling::ChromaSubsampling444) == ChromaSubsampling::CS444);
        assert!(parse_jpeg_chroma_subsampling(JpegChromaSubsampling::ChromaSubsampling422) == ChromaSubsampling::CS422);
        assert!(parse_jpeg_chroma_subsampling(JpegChromaSubsampling::ChromaSubsampling420) == ChromaSubsampling::CS420);
        assert!(parse_jpeg_chroma_subsampling(JpegChromaSubsampling::ChromaSubsampling411) == ChromaSubsampling::CS411);
        assert!(parse_jpeg_chroma_subsampling(JpegChromaSubsampling::Auto) == ChromaSubsampling::Auto);
    }

    #[test]
    fn test_build_compression_options() {
        let args = create_test_args();
        let base_path = Path::new("/test/base");

        let options = build_compression_options(&args, base_path);

        // Test that all fields are correctly mapped
        assert_eq!(options.quality, Some(80));
        assert!(!options.lossless);
        assert_eq!(options.max_size, Some(1024));
        assert_eq!(options.output_folder, Some(PathBuf::from("/output")));
        assert!(!options.same_folder_as_input);
        assert_eq!(options.overwrite_policy, OverwritePolicy::All);
        assert_eq!(options.format, OutputFormat::Jpeg);
        assert_eq!(options.suffix, Some("_compressed".to_string()));
        assert!(options.keep_structure);
        assert_eq!(options.width, Some(800));
        assert_eq!(options.height, Some(600));
        assert_eq!(options.long_edge, None);
        assert_eq!(options.short_edge, None);
        assert!(options.keep_dates);
        assert!(options.exif);
        assert_eq!(options.png_opt_level, 5);
        assert!(options.jpeg_chroma_subsampling == ChromaSubsampling::CS420);
        assert!(options.jpeg_baseline);
        assert!(options.zopfli);
        assert_eq!(options.base_path, PathBuf::from(base_path));
    }

    #[test]
    fn test_write_recap_message_empty_results() {
        // Test with empty results - should return early without printing
        let results: Vec<CompressionResult> = vec![];

        // This test mainly ensures the function doesn't panic with empty input
        write_recap_message(&results, 0);
        write_recap_message(&results, 1);
        write_recap_message(&results, 2);
        write_recap_message(&results, 3);
    }

    #[test]
    fn test_write_recap_message_statistics_calculation() {
        let results = vec![
            CompressionResult {
                original_path: "test1.jpg".to_string(),
                output_path: "out1.jpg".to_string(),
                original_size: 1000,
                compressed_size: 800,
                status: CompressionStatus::Success,
                message: "".to_string(),
            },
            CompressionResult {
                original_path: "test2.jpg".to_string(),
                output_path: "out2.jpg".to_string(),
                original_size: 2000,
                compressed_size: 1500,
                status: CompressionStatus::Skipped,
                message: "File skipped".to_string(),
            },
            CompressionResult {
                original_path: "test3.jpg".to_string(),
                output_path: "out3.jpg".to_string(),
                original_size: 500,
                compressed_size: 0,
                status: CompressionStatus::Error,
                message: "Compression failed".to_string(),
            },
        ];

        // Test with verbose = 0 (should not print detailed results)
        write_recap_message(&results, 0);

        // Test with verbose = 1 (should print summary only)
        write_recap_message(&results, 1);

        // Test with verbose = 2 (should print some details)
        write_recap_message(&results, 2);

        // Test with verbose = 3 (should print all details)
        write_recap_message(&results, 3);
    }

    #[test]
    fn test_write_recap_message_zero_division_handling() {
        let results = vec![CompressionResult {
            original_path: "test.jpg".to_string(),
            output_path: "out.jpg".to_string(),
            original_size: 0, // Test zero division case
            compressed_size: 0,
            status: CompressionStatus::Success,
            message: "".to_string(),
        }];

        // Should not panic with zero original sizes
        write_recap_message(&results, 3);
    }

    // Helper function to create test CommandLineArgs
    fn create_test_args() -> CommandLineArgs {
        CommandLineArgs {
            compression: Compression {
                quality: Some(80),
                lossless: false,
                max_size: Some(1024),
            },
            resize: Resize {
                width: Some(800),
                height: Some(600),
                long_edge: None,
                short_edge: None,
                no_upscale: false,
            },
            output_destination: OutputDestination {
                output: Some(PathBuf::from("/output")),
                same_folder_as_input: false,
            },
            format: OutputFormat::Jpeg,
            png_opt_level: 5,
            jpeg_chroma_subsampling: JpegChromaSubsampling::ChromaSubsampling420,
            jpeg_baseline: true,
            zopfli: true,
            exif: true,
            keep_dates: true,
            suffix: Some("_compressed".to_string()),
            recursive: true,
            keep_structure: true,
            dry_run: false,
            threads: 4,
            overwrite: OverwritePolicy::All,
            quiet: false,
            verbose: 2,
            files: vec!["test1.jpg".to_string(), "test2.png".to_string()],
        }
    }

    #[test]
    fn test_build_compression_options_with_defaults() {
        let mut args = create_test_args();

        // Test with some None/default values
        args.compression.quality = None;
        args.compression.max_size = None;
        args.suffix = None;
        args.output_destination.output = None;
        args.output_destination.same_folder_as_input = true;
        args.resize.width = None;
        args.resize.height = None;
        args.resize.long_edge = Some(1200);
        args.resize.short_edge = None;
        args.resize.no_upscale = true;

        let base_path = Path::new("/different/base");
        let options = build_compression_options(&args, base_path);

        assert_eq!(options.quality, None);
        assert_eq!(options.max_size, None);
        assert_eq!(options.suffix, None);
        assert_eq!(options.output_folder, None);
        assert!(options.same_folder_as_input);
        assert_eq!(options.width, None);
        assert_eq!(options.height, None);
        assert_eq!(options.long_edge, Some(1200));
        assert_eq!(options.short_edge, None);
        assert_eq!(options.base_path, PathBuf::from(base_path));
        assert!(options.no_upscale);
    }

    #[test]
    fn test_build_compression_options_edge_cases() {
        let mut args = create_test_args();
        args.format = OutputFormat::Original;
        args.jpeg_chroma_subsampling = JpegChromaSubsampling::Auto;

        let options = build_compression_options(&args, Path::new(""));

        assert_eq!(options.format, OutputFormat::Original);
        assert!(options.jpeg_chroma_subsampling == ChromaSubsampling::Auto);
        assert_eq!(options.base_path, PathBuf::from(""));
    }
}
