use clap::{Parser};
use std::path::PathBuf;
use std::fs;

#[derive(Parser, Debug)]
#[command(
    name = "Count Bytes",
    version,
    about = "Simple clone of du command to calcualte the size of the directories and files"
)]
struct Args {
    // Directory path or file path
    #[arg(short = 'p', long, default_value = ".")]
    path: PathBuf,

    #[arg(short = 'H', long)]
    human_readable: bool,

    #[arg(short = 'd', long, value_name = "N")]
    max_depth: Option<usize>,
}

fn main() {
    let args = Args::parse();

    let path = args.path;
    let human_readable = args.human_readable;
    // let max_depth = args.max_depth;
    let mut single_file_size: u64 = 0;
    
    if check_is_file(&path) {
        match get_file_size(path.clone()) {
            Ok(size) => single_file_size = size,
            Err(err) => eprintln!("Error occured: {}", err)
        }
        show_single_file_size(path, single_file_size, human_readable);
    } else {
        println!("Fuck no")
    }

}

fn check_is_file(path: &PathBuf) -> bool {
    if path.is_dir() {
        return false;
    } else {
        return true;
    }
}

fn get_file_size(path: PathBuf) -> std::io::Result<u64> {
    Ok(fs::metadata(path)?.len())
}

fn show_single_file_size(path: PathBuf, size: u64, is_human_readable: bool) {
    if is_human_readable {
        const UNITS: [&str; 6] = ["B", "KiB", "MiB", "GiB", "TiB", "PiB"];

        let mut value = size as f64;
        let mut index = 0;

        while value >= 1024.0 && index < UNITS.len() - 1 {
            value /= 1024.0;
            index += 1;
        }

        let formatted = if value >= 100.0 || value == value.floor() {
            format!("{:.0} {}", value, UNITS[index])
        } else {
            format!("{:.1} {}", value, UNITS[index])
        };

        println!("File {} size is {}", path.display(), formatted);
    } else {
        println!("File {} size is {} bytes", path.display(), size);
    }
}