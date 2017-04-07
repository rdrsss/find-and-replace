// Include crates
#[macro_use]
extern crate log;
extern crate clap;

use std::path;
use std::env;
use std::string;

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
                 .takes_value(true))
        .arg(clap::Arg::with_name("path")
                 .short("p")
                 .long("path")
                 .help("Path of directory for search to take place, defaults to current dir")
                 .takes_value(true))
        .arg(clap::Arg::with_name("extension")
                 .short("e")
                 .long("extension")
                 .help("Filter out files by extension")
                 .takes_value(true));

    return app;
}

// search all files recursively by extension
fn main() {
    // Build the cli app
    let cli = build_cli();

    // Extract matches
    let matches = cli.get_matches();

    // Set the path to current dir
    let mut path = env::current_dir().unwrap();

    if let Some(p) = matches.value_of("path") {
        println!("setting path");
        let passed_in = path::Path::new(p);
        // Make sure is valid path.
        if passed_in.exists() {
            // Make sure is directory
            if passed_in.is_dir() {
                // Set the path to search in.
                path = passed_in.to_path_buf();
            }
        }
    }

    let mut ext = string::String::from("txt");

    // Construct the file list.
    let pres = watcher::construct_file_list(&path, &ext);

    if let Some(f) = matches.value_of("find") {
        println!("finding");
        // Run find on file list.
        //
        if pres.is_ok() {
            let paths = pres.unwrap();
            for fp in paths {
                println!("{:?}", fp);
                let _ = watcher::search_file(&fp);
            }

        } else {
            // TODO: Report error or some shit.
        }
        // Construct found instances db.
    }


    if let Some(r) = matches.value_of("replace") {
        println!("replacing");
        // Replace found instances.

    }
}
