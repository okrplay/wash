mod pattern;

use clap::clap_app;
use pattern::{PatternVec, Patterns};
use std::fs::{read_dir, remove_dir_all, remove_file};

fn main() {
    let matches = clap_app!(wash =>
        (name: "wash")
        (version: env!("CARGO_PKG_VERSION"))
        (author: "okrplay <32576280+okrplay@users.noreply.github.com>")
        (about: "Automatically cleans build directories like target, node_modules etc.")
        (@arg DIRECTORY: +required "Sets the directory whose subdirectories are washed")
    )
    .get_matches();

    let mut patterns = PatternVec::new();
    patterns.load_default();

    let dir = read_dir(matches.value_of("DIRECTORY").unwrap())
        .expect("error: provided directory is not a directory");
    for entry in dir {
        let entry = entry.unwrap();
        let file_type = entry.file_type().unwrap();
        if patterns.check_all(&entry) {
            if file_type.is_file() {
                remove_file(entry.path()).unwrap();
            } else if file_type.is_dir() {
                remove_dir_all(entry.path()).unwrap();
            }
        }
    }
}
