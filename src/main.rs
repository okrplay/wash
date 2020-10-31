use clap::clap_app;
use regex::Regex;
use std::fs::{read_dir, remove_dir_all, remove_file, DirEntry, File, ReadDir};

struct DeletePattern {
    check: Regex,
    delete: Option<fn(DirEntry)>,
}

fn main() {
    let matches = clap_app!(wash =>
        (name: "wash")
        (version: env!("CARGO_PKG_VERSION"))
        (author: "okrplay <32576280+okrplay@users.noreply.github.com>")
        (about: "Automatically cleans build directories like target, node_modules etc.")
        (@arg DIRECTORY: +required "Sets the directory whose subdirectories are washed")
    )
    .get_matches();

    let delete_patterns: Vec<DeletePattern> = vec![
        DeletePattern {
            check: Regex::new("^node_modules$").unwrap(),
            delete: None,
        },
        DeletePattern {
            check: Regex::new("^target$").unwrap(),
            delete: Some(delete_target),
        },
    ];

    let dir = read_dir(matches.value_of("DIRECTORY").unwrap())
        .expect("error: provided directory is not a directory");
    clean_dir(dir, &delete_patterns);
}

fn clean_dir(dir: ReadDir, delete_patterns: &Vec<DeletePattern>) {
    'entries: for entry in dir {
        let entry = entry.unwrap();
        let file_type = entry.file_type().unwrap();
        if !file_type.is_symlink() {
            let file_name = entry.file_name();
            for pattern in delete_patterns {
                if pattern.check.is_match(&*file_name.to_string_lossy()) {
                    match pattern.delete {
                        None => {
                            if file_type.is_dir() {
                                remove_dir_all(entry.path()).unwrap();
                            } else {
                                remove_file(entry.path()).unwrap();
                            }
                        }
                        Some(run) => (run(entry)),
                    }
                    continue 'entries;
                }
            }

            if file_type.is_dir() {
                let dir = read_dir(entry.path()).unwrap();
                clean_dir(dir, delete_patterns);
            }
        }
    }
}

fn delete_target(entry: DirEntry) {
    if entry.file_type().unwrap().is_dir() {
        let mut cargo_toml_path = entry.path();
        cargo_toml_path.push("../Cargo.toml");
        if File::open(cargo_toml_path).is_ok() {
            remove_dir_all(entry.path()).unwrap();
        }
    }
}
