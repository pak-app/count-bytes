use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    name = "Count Bytes",
    version,
    about = "Simple clone of du command to calcualte the size of the directories and files"
)]
pub struct Args {
    // Directory path or file path
    #[arg(short = 'p', long, default_value = ".")]
    pub path: PathBuf,

    #[arg(short = 'H', long)]
    pub human_readable: bool,

    #[arg(short = 'd', long, value_name = "N")]
    pub max_depth: Option<usize>,

    #[arg(long)]
    show_files: bool,

    #[arg(short = 't', long)]
    tree_view: bool,
}
