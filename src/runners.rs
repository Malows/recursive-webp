use rayon::prelude::*;
use std::env::current_dir;
use std::process::{Command, Output};

use super::fs;
use super::helpers;
use super::Arguments;

pub fn convert_files(args: &Arguments) {
    let files = fs::get_files(&args.path, args.forced);

    let iter = files
        .par_iter()
        .map(|image| convert(image.as_str(), args.quality).unwrap());

    if !args.silent {
        let length = files.len() as u64;

        let progress_bar = helpers::progress_bar(length);

        iter.clone().for_each(|_| progress_bar.inc(1));
    }

    let _ = iter.collect::<Vec<_>>();
}

pub fn display_files(args: &Arguments) {
    let files = fs::get_files(&args.path, args.forced);

    let length = files.len();

    if !args.silent {
        let path_buf = current_dir().unwrap();

        let root = format!("{}/", path_buf.to_str().unwrap_or_default());

        println!("\nPosibles images to convert into webp\n");

        for file in files {
            println!("\t{}", file.replace(root.as_str(), ""));
        }
    }

    println!("\nA total of {length} files.\n");
}

fn convert(file: &str, quality: u8) -> std::io::Result<Output> {
    let _file = String::from(file);
    let webp = fs::file_to_webp(_file);
    let target = webp.as_str();

    return Command::new("cwebp")
        .arg("-q")
        .arg(quality.to_string().as_str())
        .arg(file)
        .arg("-o")
        .arg(target)
        .output();
}
