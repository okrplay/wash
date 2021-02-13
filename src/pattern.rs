use regex::Regex;
use serde::Deserialize;
use std::fs::{read_dir, DirEntry};

pub type PatternVec = Vec<Pattern>;

#[derive(Debug)]
pub struct Pattern {
    check: Regex,
    sibling_check: Option<Regex>,
}

#[derive(Deserialize)]
struct PatternFile {
    pattern: Vec<PatternRaw>,
}

#[derive(Deserialize)]
struct PatternRaw {
    check: String,
    sibling_check: Option<String>,
}

pub trait Patterns {
    fn load_default(&mut self);
    fn load_str(&mut self, patterns_str: &str);
    fn check_single(&self, dir_entry: &DirEntry, regex: &Regex) -> bool;
    fn check_all(&self, dir_entry: &DirEntry) -> bool;
}

impl Patterns for PatternVec {
    fn load_default(&mut self) {
        let default = include_str!("default_patterns.toml");
        self.load_str(default);
    }

    fn load_str(&mut self, patterns_str: &str) {
        let loaded_patterns_raw: PatternFile =
            toml::from_str(patterns_str).expect("couldn't serialize pattern file");
        let mut loaded_patterns: Vec<Pattern> = loaded_patterns_raw
            .pattern
            .iter()
            .map(|pattern_raw| Pattern {
                check: Regex::new(&pattern_raw.check).expect("invalid regex"),
                sibling_check: match &pattern_raw.sibling_check {
                    Some(sibling_check_raw) => {
                        Some(Regex::new(sibling_check_raw).expect("invalid regex"))
                    }
                    None => None,
                },
            })
            .collect();
        self.append(&mut loaded_patterns);
    }

    fn check_single(&self, dir_entry: &DirEntry, regex: &Regex) -> bool {
        let file_type = dir_entry.file_type().unwrap();
        if !file_type.is_symlink() {
            let file_name_os = dir_entry.file_name();
            let file_name = &*file_name_os.to_string_lossy();
            if regex.is_match(file_name) {
                return true;
            }
        }
        // return false if true isn't
        // explicitly returned anywhere
        false
    }

    fn check_all(&self, dir_entry: &DirEntry) -> bool {
        for pattern in self.iter() {
            if self.check_single(dir_entry, &pattern.check) {
                if let Some(sibling_check) = &pattern.sibling_check {
                    let mut parent_path = dir_entry.path();
                    parent_path.push("..");
                    for sibling in read_dir(parent_path).unwrap() {
                        let sibling = sibling.unwrap();
                        if self.check_single(&sibling, sibling_check) {
                            return true;
                        }
                    }
                } else {
                    return true;
                }
            }
        }
        false
    }
}
