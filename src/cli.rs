use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    name = "Count Bytes",
    version,
    about = "Simple clone of du command to calcualte the size of the directories and files"
)]
pub struct Args {
    /// Directory path or file path
    #[arg(short = 'p', long, default_value = ".")]
    pub path: PathBuf,

    /// Show size in human readable format
    #[arg(short = 'H', long)]
    pub human_readable: bool,

    /// Show sizes of directories and files in maximum depth
    #[arg(short = 'd', long, value_name = "N")]
    pub max_depth: Option<usize>,

    /// Show files sizes too inside directories
    #[arg(long)]
    pub show_files: bool,

    /// Show sizes in tree view (only for directories)
    #[arg(short = 't', long)]
    pub tree_view: bool,
}
