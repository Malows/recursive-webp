use glob::glob;
use std::path::Path;

pub fn get_files(path: &str, forced: bool) -> Vec<String> {
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

pub fn file_to_webp(path: String) -> String {
    let (_, extension) = path.rsplit_once(".").unwrap();

    let possible_file = path.replace(extension, "webp");

    return possible_file;
}

pub fn exists_webp_file(path: String) -> bool {
    return Path::new(file_to_webp(path).as_str()).exists();
}
