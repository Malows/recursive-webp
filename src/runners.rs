use rayon::prelude::*;
use std::env::current_dir;
use std::process::{Command, Output};

use super::fs;
use super::helpers;
use super::Context;

pub fn convert_files(ctx: &Context) {
    let iter = ctx
        .files
        .par_iter()
        .map(|image| convert(image, ctx.quality).unwrap());

    if !ctx.silent {
        let length = ctx.files.len() as u64;

        let progress_bar = helpers::progress_bar(length);

        iter.clone().for_each(|_| progress_bar.inc(1));
    }

    let _ = iter.collect::<Vec<_>>();
}

pub fn display_files(ctx: &Context) {
    let length = ctx.files.len();

    if !ctx.silent {
        let path_buf = current_dir().unwrap();

        let root = format!("{}/", path_buf.to_str().unwrap_or_default());

        println!("\nPosibles images to convert into webp\n");

        for file in ctx.files.iter() {
            println!("\t{}", file.replace(root.as_str(), ""));
        }
    }

    println!("\nA total of {length} files.\n");
}

fn convert(file: &String, quality: u8) -> std::io::Result<Output> {
    let webp = fs::file_to_webp(file);
    let target = webp.as_str();

    return Command::new("cwebp")
        .arg("-q")
        .arg(quality.to_string().as_str())
        .arg(file)
        .arg("-o")
        .arg(target)
        .output();
}
