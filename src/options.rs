use std::path::PathBuf;

use structopt::clap::arg_enum;
use structopt::StructOpt;

use crate::logger::ErrorLevel::Error;
use crate::logger::log;

arg_enum! {
    #[derive(Debug, Clone, Copy)]
    pub enum OverwritePolicy {
        All,
        None,
        Bigger
    }
}


#[derive(StructOpt, Debug)]
#[structopt(name = "", about = "CaesiumCLT - Command Line Tools for image compression")]
pub struct Opt {
    /// sets output file quality between [0-100], 0 for optimization
    #[structopt(short = "q", long)]
    pub quality: u32,

    /// keeps EXIF info during compression
    #[structopt(short = "e", long)]
    pub exif: bool,

    /// width of the output image, if height is not set will preserve aspect ratio
    #[structopt(long, default_value = "0")]
    pub width: u32,

    /// height of the output image, if width is not set will preserve aspect ratio
    #[structopt(long, default_value = "0")]
    pub height: u32,

    /// output folder
    #[structopt(short = "o", long, parse(from_os_str))]
    pub output: PathBuf,

    /// if input is a folder, scan subfolders too
    #[structopt(short = "R", long)]
    pub recursive: bool,

    /// keep the folder structure, can be used only with -R
    #[structopt(short = "S", long)]
    pub keep_structure: bool,

    /// overwrite policy
    #[structopt(short = "O", long, default_value = "all")]
    pub overwrite: OverwritePolicy,

    /// do not compress files but just show output paths
    #[structopt(short = "d", long)]
    pub dry_run: bool,

    /// suppress all output
    #[structopt(short = "Q", long)]
    pub quiet: bool,

    /// specify the number of parallel jobs (max is the number of processors available)
    #[structopt(long, default_value = "0")]
    pub threads: u32,

    /// use zopfli when optimizing PNG files (it may take a very long time to complete)
    #[structopt(long)]
    pub zopfli: bool,

    /// select how much output you want to see, 0 is equal to -Q, --quiet
    #[structopt(long, default_value = "1")]
    pub verbose: u8,

    /// Files to process
    #[structopt(name = "FILE", parse(from_os_str))]
    pub files: Vec<PathBuf>,
}

pub fn get_opts() -> Opt {
    let opt = Opt::from_args();
    validate_opts(&opt);

    opt
}

fn validate_opts(opt: &Opt) {
    let args = &opt.files;
    let verbose = opt.verbose;

    if args.is_empty() {
        log("Please provide at least one file or folder.", 101, Error, verbose);
    }
}