use std::path::PathBuf;

fn make_size_human_readable(size: u64) -> String{
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

    formatted
}

pub fn show_single_file_size(path: PathBuf, size: u64, is_human_readable: bool) {
    let file_name = path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("unknown");
    if is_human_readable {
        let formatted = make_size_human_readable(size);

        println!("File {} size is {}", file_name, formatted);
    } else {
        println!("File {} size is {} bytes", file_name, size);
    }
}

pub fn show_total_size(size: u64, is_human_readable: bool) {
    if is_human_readable {
        let formatted = make_size_human_readable(size);
        println!("Total size is {}", formatted);
    }
    else {
        println!("Total size is {} bytes", size);
    }
}
