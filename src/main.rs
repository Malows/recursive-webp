use clap::{arg, command, crate_authors, crate_version, value_parser, Arg, ArgMatches, Command};

mod fs;
mod helpers;
mod runners;

fn cli() -> Command {
    command!()
        .version(crate_version!())
        .author(crate_authors!("\n"))
        .about("Walk recursively a directory and create a webp of every image")
        .args([
            arg!(-d --dry-run "Print what files should be generated"),
            arg!(-s --silent "Display less or none information on the execution"),
            arg!(-F --forced "Overwrite previously generated webp files"),
            Arg::new("quality")
                .short('q')
                .long("quality")
                .help("Target quality of the webp images")
                .default_value("90")
                .value_parser(value_parser!(u8)),
            Arg::new("extension")
                .short('e')
                .long("extension")
                .help("Extension of files to match")
                .default_value("jpg")
                .value_parser(value_parser!(String)),
            Arg::new("directory")
                .help("Starting directory")
                .default_value(".")
                .value_parser(value_parser!(String)),
        ])
}

pub struct Context {
    pub dry_run: bool,
    pub silent: bool,
    pub quality: u8,
    pub files: Vec<String>,
}

fn get_context(args: ArgMatches) -> Context {
    let dry_run = args.get_flag("dry-run");
    let silent = args.get_flag("silent");
    let forced = args.get_flag("forced");

    let quality = args.get_one::<u8>("quality").copied().unwrap();

    let directory = args
        .get_one::<String>("directory")
        .cloned()
        .unwrap();

    let extension = args
        .get_one::<String>("extension")
        .cloned()
        .unwrap();

    let path = helpers::working_path(directory.as_str()).unwrap();
    let path = path.to_str().unwrap();

    let files = fs::get_files(path, extension.as_str(), forced);

    Context {
        dry_run,
        silent,
        quality,
        files,
    }
}

fn main() {
    let ctx = get_context(cli().get_matches());

    if ctx.dry_run {
        runners::display_files(&ctx);
        return ();
    }

    if !helpers::check_webp_dependency() {
        return ();
    }

    runners::convert_files(&ctx);
}
