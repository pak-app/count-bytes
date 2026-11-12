use crate::cli::Args;
use crate::files::get_file_size;
use crate::ui::{show_single_file_size, show_total_size};
use std::fs;
use std::path::PathBuf;
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
    let max_depth: Option<usize> = args.max_depth;
    // let show_files = args.show_files;
    // let tree_view = args.tree_view;

    let total_size = calculate_size(&path, is_human_readable, &path, max_depth);
    show_total_size(total_size, is_human_readable);
}

fn depth_diff(ancestor: &PathBuf, child: &PathBuf) -> Option<usize> {
    let a: Vec<_> = ancestor.components().collect();
    let c: Vec<_> = child.components().collect();
    if c.starts_with(&a) {
        Some(c.len() - a.len())
    } else {
        None
    }
}

fn calculate_size(
    path: &PathBuf,
    is_human_readable: bool,
    root_path: &PathBuf,
    max_depth: Option<usize>,
) -> u64 {
    let entities = fs::read_dir(path).unwrap();
    let mut dir_size = 0;

    let d_diff = depth_diff(root_path, path);
    if d_diff > max_depth {
        return 0;
    }
    
    for entry in entities {
        let entry = entry.unwrap();
        let entry_path = entry.path();

        if entry_path.is_dir() {
            dir_size += calculate_size(&entry_path, is_human_readable, root_path, max_depth);
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
