use clap::{Args, Parser, ValueEnum};
use std::path::PathBuf;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum OverwritePolicy {
    /// Always overwrite
    All,
    /// Never overwrite
    Never,
    /// Overwrite only if the file to be overwritten is bigger
    Bigger,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum OutputFormat {
    Jpeg,
    Png,
    Webp,
    Tiff,
    Original,
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

    /// convert to the selected output format, or keep the original
    #[arg(long, value_enum, default_value = "original")]
    pub format: OutputFormat,

    /// select level for PNG optimization, between [0-6]
    #[arg(long, default_value = "3")]
    pub png_opt_level: u8,

    /// use zopfli when optimizing PNG files (it may take a very long time to complete)
    #[arg(long)]
    pub zopfli: bool,

    /// keeps EXIF info during compression
    #[arg(short, long)]
    pub exif: bool,

    /// keep original file date information
    #[arg(long)]
    pub keep_dates: bool,

    /// add a suffix to the output filename
    #[arg(long)]
    pub suffix: Option<String>,

    /// if input is a folder, scan subfolders too
    #[arg(short = 'R', long)]
    pub recursive: bool,

    /// keep the folder structure, can be used only with -R
    #[arg(short = 'S', long)]
    pub keep_structure: bool,

    /// do not write output files
    #[arg(long, short, default_value = "false")]
    pub dry_run: bool,

    /// specify the number of parallel jobs (max is the number of processors available)
    #[arg(long, default_value = "0")]
    pub threads: u32,

    /// overwrite policy
    #[arg(short = 'O', long, value_enum, default_value = "all")]
    pub overwrite: OverwritePolicy,

    /// suppress all output
    #[arg(short = 'Q', long, group = "verbosity")]
    pub quiet: bool,

    /// select how much output you want to see
    #[arg(long, default_value = "1", group = "verbosity", value_parser = verbose_validator)]
    pub verbose: u8,

    pub files: Vec<String>,
}

#[derive(Args, Debug)]
#[group(required = true, multiple = false)]
pub struct Compression {
    /// sets output file quality between [0-100]
    #[arg(short, long, value_parser = quality_validator)]
    pub quality: Option<u32>,

    /// perform lossless compression
    #[arg(long, default_value = "false")]
    pub lossless: bool,

    /// set the expected maximum output size in bytes
    #[arg(long)]
    pub max_size: Option<usize>,
}

#[derive(Args, Debug)]
#[group(required = false, multiple = true)]
pub struct Resize {
    /// width of the output image, if height is not set will preserve aspect ratio
    #[arg(long, conflicts_with_all = &["long_edge", "short_edge"])]
    pub width: Option<u32>,

    /// height of the output image, if width is not set will preserve aspect ratio
    #[arg(long, conflicts_with_all = &["long_edge", "short_edge"])]
    pub height: Option<u32>,

    /// sets the size of the longest edge of the image
    #[arg(long, conflicts_with_all = &["width", "height", "short_edge"])]
    pub long_edge: Option<u32>,

    /// sets the size of the shortest edge of the image
    #[arg(long, conflicts_with_all = &["width", "height", "long_edge"])]
    pub short_edge: Option<u32>,
}

#[derive(Args, Debug)]
#[group(required = true, multiple = false)]
pub struct OutputDestination {
    /// output folder
    #[arg(short = 'o', long, group = "output_destination")]
    pub output: Option<PathBuf>,

    /// sets the output folder to be the same as the input folder, overwrites original files
    #[arg(long, default_value = "false", group = "output_destination")]
    pub same_folder_as_input: bool,
}

fn quality_validator(val: &str) -> Result<u32, String> {
    let value: u32 = val.parse().map_err(|_| format!("`{}` is not a valid number", val))?;
    if value > 100 {
        Err(format!("Quality must be between 0 and 100, but found `{}`", value))
    } else {
        Ok(value)
    }
}

fn verbose_validator(val: &str) -> Result<u8, String> {
    let value: u8 = val.parse().map_err(|_| format!("`{}` is not a valid number", val))?;
    if value > 3 {
        Err(format!("Verbosity must be between 0 and 3, but found `{}`", value))
    } else {
        Ok(value)
    }
}
