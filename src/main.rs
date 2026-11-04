mod cli;
mod files;
mod ui;
mod utils;
use utils::check_is_file;
use clap::Parser;

fn main() {
    let args = cli::Args::parse();

    let path = &args.path;

    if check_is_file(&path) {
        files::process(args)
    } else {
        println!("Fuck no")
    }
}
