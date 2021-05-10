use glob::glob;
use std::env::current_dir;
use std::path::Path;
use std::process::{Command, Output};
use indicatif::{ProgressBar, ProgressStyle};


pub fn convert_files(path: &str, quality: &str, forced: bool) {
    let files = get_files(path, forced);

    let length = files.len();

    let progress_bar = ProgressBar::new(length as u64);

    progress_bar.set_style(ProgressStyle::default_bar().template("{elapsed_precise} {wide_bar} {pos:>7}/{len:7}"));

    for image in files {
        convert(image.as_str(), quality).unwrap();
        progress_bar.inc(1);
    }
}

pub fn display_files(path: &str, forced: bool) {
    let files = get_files(path, forced);

    let length = files.len();

    let path_buf = current_dir().unwrap();

    let mut root = String::from(path_buf.to_str().unwrap_or_default());
    root.push_str("/");

    println!("\nPosibles images to convert into webp\n");

    for file in files {
        println!("\t{}", file.replace(root.as_str(), ""));
    }

    println!("\nA total of {} files.\n", length);
}

fn convert(file: &str, quality: &str) -> std::io::Result<Output> {
    let _file = String::from(file);
    let webp = file_to_webp(_file);
    let target = webp.as_str();

    return Command::new("cwebp")
        .arg("-q")
        .arg(quality)
        .arg(file)
        .arg("-o")
        .arg(target)
        .output();
}

fn get_files(path: &str, forced: bool) -> Vec<String> {
    let mut pattern = String::from(path);

    pattern.push_str("/**/*.jpg");

    let list: Vec<String> = glob(pattern.as_str())
        .unwrap()
        .into_iter()
        .map(|x| String::from(x.unwrap().to_str().unwrap()))
        .filter(|x| forced || !exists_webp_file(x.to_string()))
        .collect();

    return list;
}

fn file_to_webp(path: String) -> String {
    let (_, extension) = path.rsplit_once(".").unwrap();

    let possible_file = path.replace(extension, "webp");

    return possible_file;
}

fn exists_webp_file(path: String) -> bool {
    return Path::new(file_to_webp(path).as_str()).exists();
}
