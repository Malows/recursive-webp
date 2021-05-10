extern crate clap;

use clap::{crate_authors, crate_version, App, Arg};

mod helpers;
mod runners;

fn main() {
    let matches = App::new("Recursive webp")
        .version(crate_version!())
        .author(crate_authors!("\n"))
        .about("Walk recursively a directory and create a webp of every image")
        .arg_from_usage("[dry-run] -d --dry-run 'Print what files should be generated'")
        .arg_from_usage("[silent] -s --silent 'Display less or none information on the execution'")
        .arg_from_usage("[forced] -F --forced 'Overwite previously generated webp files'")
        .arg(
            Arg::with_name("quality")
                .help("Target quality of the webp images")
                .short("q")
                .long("quality")
                .default_value("90")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("directory")
                .help("Starting directory")
                .default_value("."),
        )
        .get_matches();

    let quality = matches.value_of("quality").unwrap();

    let path = helpers::working_path(matches.value_of("directory").unwrap()).unwrap();

    let forced = matches.is_present("forced");
    let slient = matches.is_present("silent");

    if matches.is_present("dry-run") {
        runners::display_files(path.to_str().unwrap(), forced, slient);
        return ();
    }

    if !helpers::check_webp_dependency() {
        return ();
    }

    runners::convert_files(path.to_str().unwrap(), quality, forced, slient);

    ()
}
