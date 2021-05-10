use std::{env, process::Command};

pub fn working_path(directory: &str) -> std::io::Result<std::path::PathBuf> {
    let root_path = env::current_dir()?;

    if directory != "." {
        return Ok(root_path.join(directory));
    }

    return Ok(root_path);
}

fn print_missing_webp_dependency () {
    println!("\nCan't find `cwebp` on your env path\n");
    println!("If you're running this tool on a Ubuntu-like system try the next command\n");
    println!("\tsudo apt-get install webp\n");
    println!("In other cases, please read the installing intructions from the maintainer\n");
    println!("\thttps://developers.google.com/speed/webp/download\n");
}

pub fn check_webp_dependency () -> bool {
    return match Command::new("cwebp").arg("-h").output() {
        Ok(_) => true,
        _ => {
            print_missing_webp_dependency();
            return false;
        },
    };
}
