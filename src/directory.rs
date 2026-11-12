use crate::cli::Args;
use crate::files::get_file_size;
use std::fs;
use std::path::PathBuf;
use crate::ui::{show_single_file_size, show_total_size};
// struct File {
//     path: PathBuf,
//     size: u32,
// }

// struct Dir<'a> {
//     path: &'a PathBuf,
//     files: Vec<&'a File>,
//     dirs: Vec<&'a Dir<'a>>,
// }

pub fn process(args: Args) {
    let path = args.path;
    let is_human_readable = args.human_readable;
    // let max_depth = args.max_depth;
    // let show_files = args.show_files;
    // let tree_view = args.tree_view;

    let total_size = calculate_size(&path, is_human_readable);
    show_total_size(total_size, is_human_readable);
}

fn calculate_size(path: &PathBuf, is_human_readable: bool) -> u64 {
    let entities = fs::read_dir(path).unwrap();
    let mut dir_size = 0;

    for entry in entities {
        let entry = entry.unwrap();
        let entry_path = entry.path();

        if entry_path.is_dir() {
            dir_size += calculate_size(&entry_path, is_human_readable);
        } else {
            let mut file_size: u64 = 0;
            match get_file_size(path.clone()) {
                Ok(size) => file_size = size,
                Err(err) => eprintln!("Error occured: {}", err),
            }
            show_single_file_size(entry_path, file_size, is_human_readable);
            dir_size += file_size;
        }
    }

    dir_size
}
