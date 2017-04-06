// Include crates
#[macro_use]
extern crate log;
extern crate clap;

// Pull in watcher module
pub mod watcher;


fn build_cli<'a, 'b>() -> clap::App<'a, 'b> {
    let app = clap::App::new("fnr");

    return app;
}

// search all files recursively by extension
//
fn main() {
    println!("Hello, world!");

    let cli = build_cli();
}
