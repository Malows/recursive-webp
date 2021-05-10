use glob::glob;
use std::env::current_dir;
use std::path::Path;
use std::process::{Command, Output};

pub fn convert_files(path: &str, quality: &str, forced: bool) {
    let mut files = get_files(path);

    if !forced {
        files = files
            .into_iter()
            .filter(|x| !exists_webp_file(x.to_string()))
            .collect();
    }

    for image in files {
        convert(image.as_str(), quality).unwrap();
    }
}

pub fn display_files(path: &str, forced: bool) {
    let mut files = get_files(path);

    if !forced {
        files = files
            .into_iter()
            .filter(|x| !exists_webp_file(x.to_string()))
            .collect();
    }

    let path_buf = current_dir().unwrap();

    let mut root = String::from(path_buf.to_str().unwrap_or_default());
    root.push_str("/");

    let length = files.len();

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

fn get_files(path: &str) -> Vec<String> {
    let mut pattern = String::from(path);

    pattern.push_str("/**/*.jpg");

    let list: Vec<String> = glob(pattern.as_str())
        .unwrap()
        .into_iter()
        .map(|x| String::from(x.unwrap().to_str().unwrap()))
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
