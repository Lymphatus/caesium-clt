use clap::{Args, Parser, ValueEnum};
use std::path::PathBuf;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum OverwritePolicy {
    /// Always overwrite existing files
    All,
    /// Never overwrite existing files
    Never,
    /// Overwrite only if the existing file is bigger
    Bigger,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum OutputFormat {
    Jpeg,
    Png,
    Gif,
    Webp,
    Tiff,
    Original,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum JpegChromaSubsampling {
    #[value(name = "4:4:4")]
    ChromaSubsampling444,
    #[value(name = "4:2:2")]
    ChromaSubsampling422,
    #[value(name = "4:2:0")]
    ChromaSubsampling420,
    #[value(name = "4:1:1")]
    ChromaSubsampling411,
    #[value(name = "auto")]
    Auto,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct CommandLineArgs {
    #[command(flatten)]
    pub compression: Compression,

    #[command(flatten)]
    pub resize: Resize,

    #[command(flatten)]
    pub output_destination: OutputDestination,

    /// Convert to the selected output format or keep the original
    #[arg(long, value_enum, default_value = "original")]
    pub format: OutputFormat,

    /// PNG optimization level [0-6], higher values provide better compression
    #[arg(long, default_value = "3", value_parser = png_opt_level_validator)]
    pub png_opt_level: u8,

    /// Chroma subsampling for JPEG files
    #[arg(long, value_enum, default_value = "auto")]
    pub jpeg_chroma_subsampling: JpegChromaSubsampling,

    /// Output baseline JPEG instead of progressive (default)
    #[arg(long)]
    pub jpeg_baseline: bool,

    /// Use zopfli for PNG optimization (significantly slower but better compression)
    #[arg(long)]
    pub zopfli: bool,

    /// Keep EXIF metadata during compression
    #[arg(short, long)]
    pub exif: bool,

    /// Preserve original file timestamps
    #[arg(long)]
    pub keep_dates: bool,

    /// Add suffix to output filenames
    #[arg(long)]
    pub suffix: Option<String>,

    /// Scan subfolders recursively when input is a directory
    #[arg(short = 'R', long)]
    pub recursive: bool,

    /// Preserve directory structure (requires -R/--recursive)
    #[arg(short = 'S', long)]
    pub keep_structure: bool,

    /// Simulate compression without writing files
    #[arg(long, short, default_value = "false")]
    pub dry_run: bool,

    /// Number of parallel jobs (0 = auto-detect, max = available processors)
    #[arg(long, default_value = "0")]
    pub threads: u32,

    /// Policy for handling existing output files
    #[arg(short = 'O', long, value_enum, default_value = "all")]
    pub overwrite: OverwritePolicy,

    /// Suppress all output
    #[arg(short = 'Q', long, group = "verbosity")]
    pub quiet: bool,

    /// Verbosity level: 0 = quiet, 1 = progress only, 2 = errors only, 3 = all
    #[arg(long, default_value = "1", group = "verbosity", value_parser = verbosity_validator)]
    pub verbose: u8,

    /// Input files or directories to process
    pub files: Vec<String>,
}

#[derive(Args, Debug)]
#[group(required = true, multiple = false)]
pub struct Compression {
    /// Compression quality [0-100], higher values mean better quality
    #[arg(short, long, value_parser = quality_validator)]
    pub quality: Option<u32>,

    /// Use lossless compression (may increase file size for some formats)
    #[arg(long)]
    pub lossless: bool,

    /// Target maximum file size in bytes
    #[arg(long)]
    pub max_size: Option<usize>,
}

#[derive(Args, Debug)]
#[group(required = false, multiple = true)]
pub struct Resize {
    /// Output image width in pixels (preserves the aspect ratio if height not set)
    #[arg(long, conflicts_with_all = &["long_edge", "short_edge"])]
    pub width: Option<u32>,

    /// Output image height in pixels (preserves the aspect ratio if width not set)
    #[arg(long, conflicts_with_all = &["long_edge", "short_edge"])]
    pub height: Option<u32>,

    /// Size in pixels for the longest edge of the image
    #[arg(long, conflicts_with_all = &["width", "height", "short_edge"])]
    pub long_edge: Option<u32>,

    /// Size in pixels for the shortest edge of the image
    #[arg(long, conflicts_with_all = &["width", "height", "long_edge"])]
    pub short_edge: Option<u32>,

    #[arg(long)]
    pub no_upscale: bool,
}

#[derive(Args, Debug)]
#[group(required = true, multiple = false)]
pub struct OutputDestination {
    /// Output directory path
    #[arg(short = 'o', long, group = "output_destination")]
    pub output: Option<PathBuf>,

    /// Use input file's directory as output (WARNING: may overwrite originals)
    #[arg(long, group = "output_destination", default_value = "false")]
    pub same_folder_as_input: bool,
}

/// Validates quality values are within the valid range [0-100]
fn quality_validator(val: &str) -> Result<u32, String> {
    validate_range(val, 0, 100, "Quality")
}

/// Validates verbosity levels are within the valid range [0-3]
fn verbosity_validator(val: &str) -> Result<u8, String> {
    validate_range(val, 0, 3, "Verbosity")
}

/// Validates PNG optimization levels are within the valid range [0-6]
fn png_opt_level_validator(val: &str) -> Result<u8, String> {
    validate_range(val, 0, 6, "PNG optimization level")
}

/// Generic validator for numeric ranges
fn validate_range<T>(val: &str, min: T, max: T, field_name: &str) -> Result<T, String>
where
    T: std::str::FromStr + PartialOrd + std::fmt::Display + Copy,
    T::Err: std::fmt::Display,
{
    let value = val.parse::<T>().map_err(|_| format!("'{val}' is not a valid number"))?;

    if value < min || value > max {
        Err(format!("{field_name} must be between {min} and {max}, but got {value}"))
    } else {
        Ok(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quality_validator() {
        assert!(quality_validator("50").is_ok());
        assert!(quality_validator("0").is_ok());
        assert!(quality_validator("100").is_ok());
        assert!(quality_validator("101").is_err());
        assert!(quality_validator("-1").is_err());
        assert!(quality_validator("abc").is_err());
    }

    #[test]
    fn test_verbosity_validator() {
        assert!(verbosity_validator("0").is_ok());
        assert!(verbosity_validator("3").is_ok());
        assert!(verbosity_validator("4").is_err());
        assert!(verbosity_validator("255").is_err());
    }

    #[test]
    fn test_png_opt_level_validator() {
        assert!(png_opt_level_validator("3").is_ok());
        assert!(png_opt_level_validator("0").is_ok());
        assert!(png_opt_level_validator("6").is_ok());
        assert!(png_opt_level_validator("7").is_err());
    }

    #[test]
    fn test_validate_range() {
        // Test with u32
        assert_eq!(validate_range("50", 0u32, 100u32, "Test").unwrap(), 50u32);
        assert!(validate_range("101", 0u32, 100u32, "Test").is_err());
        assert!(validate_range("-1", 0u32, 100u32, "Test").is_err());
        assert!(validate_range("abc", 0u32, 100u32, "Test").is_err());

        // Test with i32
        assert_eq!(validate_range("-5", -10i32, 10i32, "Test").unwrap(), -5i32);
        assert!(validate_range("-11", -10i32, 10i32, "Test").is_err());
        assert!(validate_range("11", -10i32, 10i32, "Test").is_err());

        // Test with f32
        assert_eq!(validate_range("5.5", 0.0f32, 10.0f32, "Test").unwrap(), 5.5f32);
        assert!(validate_range("-0.1", 0.0f32, 10.0f32, "Test").is_err());
        assert!(validate_range("10.1", 0.0f32, 10.0f32, "Test").is_err());
    }

    #[test]
    fn test_overwrite_policy() {
        // Test that the enum variants exist and can be used
        let all = OverwritePolicy::All;
        let never = OverwritePolicy::Never;
        let bigger = OverwritePolicy::Bigger;

        // Verify they're different
        assert_ne!(format!("{all:?}"), format!("{:?}", never));
        assert_ne!(format!("{all:?}"), format!("{:?}", bigger));
        assert_ne!(format!("{never:?}"), format!("{:?}", bigger));
    }

    #[test]
    fn test_output_format() {
        // Test that the enum variants exist and can be used
        let jpeg = OutputFormat::Jpeg;
        let png = OutputFormat::Png;
        let webp = OutputFormat::Webp;
        let tiff = OutputFormat::Tiff;
        let gif = OutputFormat::Gif;
        let original = OutputFormat::Original;

        // Verify they're different
        assert_ne!(format!("{jpeg:?}"), format!("{:?}", png));
        assert_ne!(format!("{jpeg:?}"), format!("{:?}", webp));
        assert_ne!(format!("{jpeg:?}"), format!("{:?}", tiff));
        assert_ne!(format!("{jpeg:?}"), format!("{:?}", original));
        assert_ne!(format!("{png:?}"), format!("{:?}", webp));
        assert_ne!(format!("{png:?}"), format!("{:?}", tiff));
        assert_ne!(format!("{png:?}"), format!("{:?}", original));
        assert_ne!(format!("{webp:?}"), format!("{:?}", tiff));
        assert_ne!(format!("{webp:?}"), format!("{:?}", original));
        assert_ne!(format!("{tiff:?}"), format!("{:?}", original));
        assert_ne!(format!("{gif:?}"), format!("{:?}", original));
    }

    #[test]
    fn test_jpeg_chroma_subsampling() {
        // Test that the enum variants exist and can be used
        let cs444 = JpegChromaSubsampling::ChromaSubsampling444;
        let cs422 = JpegChromaSubsampling::ChromaSubsampling422;
        let cs420 = JpegChromaSubsampling::ChromaSubsampling420;
        let cs411 = JpegChromaSubsampling::ChromaSubsampling411;
        let auto = JpegChromaSubsampling::Auto;

        // Verify they're different
        assert_ne!(format!("{cs444:?}"), format!("{:?}", cs422));
        assert_ne!(format!("{cs444:?}"), format!("{:?}", cs420));
        assert_ne!(format!("{cs444:?}"), format!("{:?}", cs411));
        assert_ne!(format!("{cs444:?}"), format!("{:?}", auto));
        assert_ne!(format!("{cs422:?}"), format!("{:?}", cs420));
        assert_ne!(format!("{cs422:?}"), format!("{:?}", cs411));
        assert_ne!(format!("{cs422:?}"), format!("{:?}", auto));
        assert_ne!(format!("{cs420:?}"), format!("{:?}", cs411));
        assert_ne!(format!("{cs420:?}"), format!("{:?}", auto));
        assert_ne!(format!("{cs411:?}"), format!("{:?}", auto));
    }
}
