use std::env::current_dir;
use std::process::{Command, Output};

use super::fs;
use super::helpers;

pub fn convert_files(path: &str, quality: &str, forced: bool, silent: bool) {
    let files = fs::get_files(path, forced);

    if silent {
        for image in files {
            convert(image.as_str(), quality).unwrap();
        }
        return ();
    }

    let length = files.len();

    let progress_bar = helpers::progress_bar(length as u64);

    for image in files {
        convert(image.as_str(), quality).unwrap();
        progress_bar.inc(1);
    }
}

pub fn display_files(path: &str, forced: bool, silent: bool) {
    let files = fs::get_files(path, forced);

    let length = files.len();

    if !silent {
        let path_buf = current_dir().unwrap();

        let mut root = String::from(path_buf.to_str().unwrap_or_default());
        root.push_str("/");

        println!("\nPosibles images to convert into webp\n");

        for file in files {
            println!("\t{}", file.replace(root.as_str(), ""));
        }
    }

    println!("\nA total of {} files.\n", length);
}

fn convert(file: &str, quality: &str) -> std::io::Result<Output> {
    let _file = String::from(file);
    let webp = fs::file_to_webp(_file);
    let target = webp.as_str();

    return Command::new("cwebp")
        .arg("-q")
        .arg(quality)
        .arg(file)
        .arg("-o")
        .arg(target)
        .output();
}
