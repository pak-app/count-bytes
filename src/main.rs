mod cli;
mod files;
mod ui;
mod directory;
use clap::Parser;


fn main() {
    let args = cli::Args::parse();

    let path = &args.path;

    if path.is_file() {
        files::process(args)
    } else {
        directory::process(args)
    }
}
