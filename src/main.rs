// use clap::{ App, Arg};
use clap::{arg, command, crate_authors, crate_version, ArgMatches, Command};

mod fs;
mod helpers;
mod runners;

fn cli() -> Command {
    command!()
        .version(crate_version!())
        .author(crate_authors!("\n"))
        .args([
            arg!(-d --dry-run "Print what files should be generated"),
            arg!(-s --silent "Display less or none information on the execution"),
            arg!(-F --forced "Overwite previously generated webp files"),
            arg!(-q --quality [QUALITY] "Target quality of the webp images"),
            arg!([directory] "Starting directory"),
        ])
}

pub struct Arguments {
    pub dry_run: bool,
    pub silent: bool,
    pub forced: bool,
    pub quality: u8,
    pub path: String,
}

fn get_arguments(args: ArgMatches) -> Arguments {
    let dry_run = args.get_flag("dry-run");
    let silent = args.get_flag("silent");
    let forced = args.get_flag("forced");

    let quality = args.get_one::<u8>("quality").copied().unwrap_or(90);

    let directory = args
        .get_one::<String>("directory")
        .cloned()
        .unwrap_or(String::from("."));

    let path = helpers::working_path(directory.as_str()).unwrap();
    let path = path.to_str().unwrap();

    Arguments {
        dry_run,
        silent,
        forced,
        quality,
        path: path.to_string(),
    }
}

fn main() {
    let args = get_arguments(cli().get_matches());

    if args.dry_run {
        runners::display_files(&args);
        return ();
    }

    if !helpers::check_webp_dependency() {
        return ();
    }

    runners::convert_files(&args);

    ()
}
