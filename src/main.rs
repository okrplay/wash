mod pattern;

use clap::clap_app;
use pattern::{PatternVec, Patterns};
use std::fs::{read_dir, remove_dir_all, remove_file, ReadDir};

fn main() {
    let matches = clap_app!(wash =>
        (name: "wash")
        (version: env!("CARGO_PKG_VERSION"))
        (author: "okrplay <32576280+okrplay@users.noreply.github.com>")
        (about: "Automatically cleans build directories like target, node_modules etc.")
        (@arg DRY_RUN: -d --("dry-run") "Print directories instead of deleting them")
        (@arg HIDDEN: -h --hidden "Traverse directories starting with a dot, which are ignored by default")
        (@arg DIRECTORY: +required "Set the directory whose subdirectories are washed")
    )
    .get_matches();

    let mut patterns = PatternVec::new();
    patterns.load_default();

    let mut dir = read_dir(matches.value_of("DIRECTORY").unwrap())
        .expect("error: provided directory is not a directory");
    let dry_run = matches.is_present("DRY_RUN");
    let hidden = matches.is_present("HIDDEN");
    check_and_delete(&mut dir, &patterns, dry_run, hidden)
}

fn check_and_delete(dir: &mut ReadDir, patterns: &PatternVec, dry_run: bool, hidden: bool) {
    // vec for dry run results
    let mut paths = Vec::new();

    // check all directory entries
    for entry in dir {
        let entry = entry.unwrap();
        let file_type = entry.file_type().unwrap();
        if patterns.check_all(&entry) {
            if !dry_run {
                if file_type.is_file() {
                    remove_file(entry.path()).unwrap();
                } else if file_type.is_dir() {
                    remove_dir_all(entry.path()).unwrap();
                }
            } else {
                paths.push(entry.path().to_string_lossy().to_string());
            }
        } else if file_type.is_dir()
            && (!entry.file_name().to_string_lossy().starts_with(".") || hidden)
        {
            check_and_delete(
                &mut read_dir(entry.path()).unwrap(),
                patterns,
                dry_run,
                hidden,
            )
        }
    }

    // print dry run results
    if dry_run {
        paths.sort();
        for path in paths {
            println!("{}", path);
        }
    }
}
