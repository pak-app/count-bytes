use crate::cli::Args;
use crate::ui;
use std::fs;
use std::path::PathBuf;


fn get_file_size(path: PathBuf) -> std::io::Result<u64> {
    Ok(fs::metadata(path)?.len())
}

pub fn process(args: Args) {
    let path = args.path.clone();
    let human_readable = args.human_readable;

    let mut single_file_size: u64 = 0;

    match get_file_size(path.clone()) {
        Ok(size) => single_file_size = size,
        Err(err) => eprintln!("Error occured: {}", err),
    }
    ui::show_single_file_size(path, single_file_size, human_readable);
}
