// Include crates
#[macro_use]
extern crate log;
extern crate clap;

// Pull in watcher module
pub mod watcher;


/// build_cli build's the cli app.
fn build_cli<'a, 'b>() -> clap::App<'a, 'b> {
    let app = clap::App::new("fnr")
        .version("0.1")
        .author("Manuel A. Rodriguez <manuel@rdrsss.io>")
        .about("Finds and replaces text within text files.")
        .arg(clap::Arg::with_name("find")
                 .short("f")
                 .long("find")
                 .help("The string to find")
                 .takes_value(true))
        .arg(clap::Arg::with_name("replace")
                 .short("r")
                 .long("replace")
                 .help("The string to replace with")
                 .takes_value(true));

    return app;
}

// search all files recursively by extension
//
fn main() {
    // Build the cli app
    let cli = build_cli();

    // Extract matches
    let matches = cli.get_matches();
}
