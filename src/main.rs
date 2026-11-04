mod cli;
mod files;
mod ui;
use clap::Parser;

fn main() {
    let args = cli::Args::parse();

    let path = args.path;
    let human_readable = args.human_readable;
    // let max_depth = args.max_depth;
    // tree_view = args.tree_view;
    // dir_only = args.show_files;
    let mut single_file_size: u64 = 0;

    if files::check_is_file(&path) {
        match files::get_file_size(path.clone()) {
            Ok(size) => single_file_size = size,
            Err(err) => eprintln!("Error occured: {}", err),
        }
        ui::show_single_file_size(path, single_file_size, human_readable);
    } else {
        println!("Fuck no")
    }
}
