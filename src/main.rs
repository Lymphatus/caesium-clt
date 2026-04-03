use crate::compressor::{start_compression, CompressionOptions, CompressionResult, CompressionStatus};
use crate::options::{CommandLineArgs, JpegChromaSubsampling};
use crate::scan_files::scan_files;
use bytesize::ByteSize;
use caesium::parameters::ChromaSubsampling;
use clap::Parser;
use colored::Colorize;
use indicatif::{MultiProgress, ProgressBar, ProgressDrawTarget, ProgressStyle};
use serde::Serialize;
use std::num::NonZero;
use std::path::{Path, PathBuf};
use std::process::exit;
use std::time::Duration;

#[derive(Serialize)]
struct JsonSummary {
    total_files: usize,
    success: usize,
    skipped: usize,
    errors: usize,
    original_size: u64,
    compressed_size: u64,
    savings_bytes: i64,
    savings_percent: f64,
}

#[derive(Serialize)]
struct JsonOutput<'a> {
    version: &'static str,
    dry_run: bool,
    error: Option<&'a str>,
    files: &'a [CompressionResult],
    summary: JsonSummary,
}

mod compressor;
mod options;
mod scan_files;

const PROGRESS_UPDATE_INTERVAL: Duration = Duration::from_secs(1);
const FALLBACK_THREAD_COUNT: usize = 1;

fn main() {
    let args = CommandLineArgs::parse();

    if args.files.is_empty() {
        if args.json {
            write_json_output(&[], args.dry_run, Some("No files to compress"));
        } else {
            eprintln!("No files to compress");
        }
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
    let (base_path, input_files) = scan_files(
        &args.files,
        args.recursive,
        quiet || args.json,
        args.check_extension_only,
    );
    let base_path = match base_path {
        Some(bp) => bp,
        None => {
            if args.json {
                write_json_output(
                    &[],
                    args.dry_run,
                    Some("Unable to compute the base path for the files."),
                );
            } else {
                eprintln!("Unable to compute the base path for the files.");
            }
            exit(-1);
        }
    };
    let total_files = input_files.len();

    let progress_target = if args.json {
        ProgressDrawTarget::stderr()
    } else {
        ProgressDrawTarget::stdout()
    };
    let (multi_progress, progress_bar) = setup_progress_bar(total_files, verbose, progress_target);
    let compression_options = build_compression_options(&args, &base_path);
    let compression_results = start_compression(
        &input_files,
        &compression_options,
        &multi_progress,
        &progress_bar,
        args.dry_run,
    );
    progress_bar.finish();

    if args.json {
        write_json_output(&compression_results, args.dry_run, None);
    } else {
        write_recap_message(&compression_results, verbose);
    }
}

struct CompressionStats {
    total_original_size: u64,
    total_compressed_size: u64,
    success: usize,
    skipped: usize,
    errors: usize,
}

impl CompressionStats {
    fn from_results(results: &[CompressionResult]) -> Self {
        let (total_original_size, total_compressed_size, success, skipped, errors) = results.iter().fold(
            (0u64, 0u64, 0usize, 0usize, 0usize),
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
        Self {
            total_original_size,
            total_compressed_size,
            success,
            skipped,
            errors,
        }
    }

    fn savings_bytes(&self) -> i64 {
        self.total_original_size as i64 - self.total_compressed_size as i64
    }

    fn savings_percent(&self) -> f64 {
        if self.total_original_size > 0 {
            (self.savings_bytes() as f64 / self.total_original_size as f64) * 100.0
        } else {
            0.0
        }
    }
}

fn build_json_output_string(compression_results: &[CompressionResult], dry_run: bool, error: Option<&str>) -> String {
    let stats = CompressionStats::from_results(compression_results);
    let output = JsonOutput {
        version: "1.0",
        dry_run,
        error,
        files: compression_results,
        summary: JsonSummary {
            total_files: compression_results.len(),
            success: stats.success,
            skipped: stats.skipped,
            errors: stats.errors,
            original_size: stats.total_original_size,
            compressed_size: stats.total_compressed_size,
            savings_bytes: stats.savings_bytes(),
            savings_percent: stats.savings_percent(),
        },
    };
    serde_json::to_string(&output).unwrap_or_else(|e| format!("{{\"error\":\"JSON serialization failed: {e}}}"))
}

fn write_json_output(compression_results: &[CompressionResult], dry_run: bool, error: Option<&str>) {
    println!("{}", build_json_output_string(compression_results, dry_run, error));
}

fn write_recap_message(compression_results: &[CompressionResult], verbose: u8) {
    if compression_results.is_empty() {
        return;
    }

    let stats = CompressionStats::from_results(compression_results);
    let (total_original_size, total_compressed_size, total_success, total_skipped, total_errors) = (
        stats.total_original_size,
        stats.total_compressed_size,
        stats.success,
        stats.skipped,
        stats.errors,
    );

    if verbose > 1 {
        for result in compression_results {
            if verbose < 3 && matches!(result.status, CompressionStatus::Success) {
                continue;
            }

            let savings_size = result.original_size as i64 - result.compressed_size as i64;
            let savings_percent = if result.original_size > 0 {
                (savings_size as f64 / result.original_size as f64) * 100.0
            } else {
                0.0
            };

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
        let total_saved = stats.savings_bytes();
        let total_saved_percent = stats.savings_percent();
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

fn setup_progress_bar(len: usize, verbose: u8, target: ProgressDrawTarget) -> (MultiProgress, ProgressBar) {
    let multi_progress = MultiProgress::new();
    let progress_bar = multi_progress.add(ProgressBar::new(len as u64));

    if verbose == 0 {
        multi_progress.set_draw_target(ProgressDrawTarget::hidden());
        return (multi_progress, progress_bar);
    }

    multi_progress.set_draw_target(target);
    progress_bar.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos}/{len}\n{msg}")
            .unwrap_or(ProgressStyle::default_bar())
            .progress_chars("#>-"),
    );
    progress_bar.enable_steady_tick(PROGRESS_UPDATE_INTERVAL);
    (multi_progress, progress_bar)
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
        strip_icc: args.strip_icc,
        min_savings: args.min_savings,
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
        // Test with verbose = 0 (hidden regardless of target)
        let (_multi, progress_bar) = setup_progress_bar(10, 0, ProgressDrawTarget::stdout());
        assert!(progress_bar.is_hidden());
        assert_eq!(progress_bar.length(), Some(10));

        // Test with different lengths
        let (_multi, progress_bar) = setup_progress_bar(0, 1, ProgressDrawTarget::stdout());
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
            min_savings: None,
            quiet: false,
            verbose: 2,
            json: false,
            files: vec!["test1.jpg".to_string(), "test2.png".to_string()],
            strip_icc: false,
            check_extension_only: false,
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
    fn test_compression_stats_from_results() {
        let results = vec![
            CompressionResult {
                original_path: "a.jpg".to_string(),
                output_path: "a_out.jpg".to_string(),
                original_size: 1000,
                compressed_size: 800,
                status: CompressionStatus::Success,
                message: "".to_string(),
            },
            CompressionResult {
                original_path: "b.jpg".to_string(),
                output_path: "b_out.jpg".to_string(),
                original_size: 2000,
                compressed_size: 2000,
                status: CompressionStatus::Skipped,
                message: "".to_string(),
            },
            CompressionResult {
                original_path: "c.jpg".to_string(),
                output_path: "c_out.jpg".to_string(),
                original_size: 500,
                compressed_size: 0,
                status: CompressionStatus::Error,
                message: "".to_string(),
            },
        ];

        let stats = CompressionStats::from_results(&results);
        assert_eq!(stats.success, 1);
        assert_eq!(stats.skipped, 1);
        assert_eq!(stats.errors, 1);
        assert_eq!(stats.total_original_size, 3500);
        assert_eq!(stats.total_compressed_size, 2800);
        assert_eq!(stats.savings_bytes(), 700);
        assert!((stats.savings_percent() - 20.0).abs() < 0.01);
    }

    #[test]
    fn test_compression_stats_empty() {
        let stats = CompressionStats::from_results(&[]);
        assert_eq!(stats.success, 0);
        assert_eq!(stats.skipped, 0);
        assert_eq!(stats.errors, 0);
        assert_eq!(stats.total_original_size, 0);
        assert_eq!(stats.total_compressed_size, 0);
        assert_eq!(stats.savings_bytes(), 0);
        assert_eq!(stats.savings_percent(), 0.0);
    }

    #[test]
    fn test_compression_stats_size_increase() {
        let results = vec![CompressionResult {
            original_path: "a.jpg".to_string(),
            output_path: "a_out.jpg".to_string(),
            original_size: 800,
            compressed_size: 1000,
            status: CompressionStatus::Success,
            message: "".to_string(),
        }];

        let stats = CompressionStats::from_results(&results);
        assert_eq!(stats.savings_bytes(), -200);
        assert!(stats.savings_percent() < 0.0);
    }

    #[test]
    fn test_build_json_output_success() {
        let results = vec![CompressionResult {
            original_path: "input.jpg".to_string(),
            output_path: "output.jpg".to_string(),
            original_size: 1000,
            compressed_size: 600,
            status: CompressionStatus::Success,
            message: "".to_string(),
        }];

        let json = build_json_output_string(&results, false, None);
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed["version"], "1.0");
        assert_eq!(parsed["dry_run"], false);
        assert!(parsed["error"].is_null());
        assert_eq!(parsed["files"].as_array().unwrap().len(), 1);
        assert_eq!(parsed["files"][0]["original_path"], "input.jpg");
        assert_eq!(parsed["files"][0]["status"], "success");
        assert_eq!(parsed["summary"]["total_files"], 1);
        assert_eq!(parsed["summary"]["success"], 1);
        assert_eq!(parsed["summary"]["skipped"], 0);
        assert_eq!(parsed["summary"]["errors"], 0);
        assert_eq!(parsed["summary"]["original_size"], 1000);
        assert_eq!(parsed["summary"]["compressed_size"], 600);
        assert_eq!(parsed["summary"]["savings_bytes"], 400);
        assert!((parsed["summary"]["savings_percent"].as_f64().unwrap() - 40.0).abs() < 0.01);
    }

    #[test]
    fn test_build_json_output_with_error() {
        let json = build_json_output_string(&[], false, Some("No files to compress"));
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed["error"], "No files to compress");
        assert_eq!(parsed["files"].as_array().unwrap().len(), 0);
        assert_eq!(parsed["summary"]["total_files"], 0);
    }

    #[test]
    fn test_build_json_output_dry_run() {
        let json = build_json_output_string(&[], true, None);
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed["dry_run"], true);
    }

    #[test]
    fn test_build_json_output_status_variants() {
        let results = vec![
            CompressionResult {
                original_path: "a.jpg".to_string(),
                output_path: "a_out.jpg".to_string(),
                original_size: 100,
                compressed_size: 80,
                status: CompressionStatus::Success,
                message: "".to_string(),
            },
            CompressionResult {
                original_path: "b.jpg".to_string(),
                output_path: "b_out.jpg".to_string(),
                original_size: 100,
                compressed_size: 100,
                status: CompressionStatus::Skipped,
                message: "min savings not met".to_string(),
            },
            CompressionResult {
                original_path: "c.jpg".to_string(),
                output_path: "c_out.jpg".to_string(),
                original_size: 100,
                compressed_size: 0,
                status: CompressionStatus::Error,
                message: "read error".to_string(),
            },
        ];

        let json = build_json_output_string(&results, false, None);
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed["files"][0]["status"], "success");
        assert_eq!(parsed["files"][1]["status"], "skipped");
        assert_eq!(parsed["files"][2]["status"], "error");
        assert_eq!(parsed["summary"]["success"], 1);
        assert_eq!(parsed["summary"]["skipped"], 1);
        assert_eq!(parsed["summary"]["errors"], 1);
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
