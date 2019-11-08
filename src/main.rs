#![allow(unused_variables, dead_code)]
use std::collections::HashMap;
use std::path::PathBuf;

use regex::Regex;

#[macro_use]
extern crate lazy_static;

enum Link {
    Web(String),
    Local(String),
}

impl std::fmt::Display for Link {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Link::Web(s) => write!(f, "{}", s),
            Link::Local(s) => {
                let out = format!("{}\n\t{}", s, similar_filename_to(s.to_string()));
                write!(f, "{}", out)
            }
        }
    }
}

fn main() {
    println!("Hello, world!");
    let mut files_and_broken_links: HashMap<String, Vec<Link>> = HashMap::new();
    let filenames: Vec<PathBuf> = Vec::new();
    for filename in filenames {
        let broken = get_broken_links_from_file(&filename);
        if !broken.is_empty() {
            files_and_broken_links.insert(filename.to_string_lossy().to_string(), broken);
        }
    }
    for (filename, broken_links) in files_and_broken_links {
        println!("{}", filename);
        for link in broken_links {
            println!("{}", link);
        }
    }
}

fn similar_filename_to(s: String) -> String {
    unimplemented!()
}

fn get_links_from_file(p: &PathBuf) -> Vec<String> {
    lazy_static! {
        static ref RE_URL: Regex = Regex::new(
            r#"(https+://)*[-a-zA-Z0-9@:%._\+~#=]{1,256}\.[a-zA-Z0-9()]{1,6}\b[-a-zA-Z0-9@:%_\+.~#?&//=]*"#
        )
        .unwrap();
    }
    let contents = std::fs::read_to_string(p).unwrap();
    let mut links = Vec::new();
    for line in contents.split("\n") {
        if RE_URL.is_match(line) {
            let l: Vec<String> = RE_URL
                .captures_iter(line)
                .map(|x| x[0].to_string().clone())
                .map(|x| x.to_owned())
                .collect();
            links.extend(l);
        }
    }
    links
}

fn get_broken_links_from_file(p: &PathBuf) -> Vec<Link> {
    let links = get_links_from_file(p);
    let mut broken_links = Vec::new();
    for link in links {
        if is_local_link(&link) {
            if !is_valid_local_link(&link) {
                broken_links.push(Link::Local(link));
            }
        } else if !is_valid_web_link(&link) {
            broken_links.push(Link::Web(link));
        }
    }
    broken_links
}

fn is_local_link(s: &str) -> bool {
    unimplemented!()
}

fn is_web_link(s: &str) -> bool {
    unimplemented!()
}

fn is_valid_local_link(l: &str) -> bool {
    let p = PathBuf::from(l);
    p.exists()
}

fn is_valid_web_link(l: &str) -> bool {
    unimplemented!()
}

mod test {
    use super::*;

    #[test]
    fn verify_link_is_local_link() {
        let tests = vec!["./test.md"];
        for test in tests {
            assert_eq!(is_local_link(test), true);
        }
    }

    #[test]
    fn verify_link_is_web_link() {
        let tests = vec!["www.google.com", "https://www.stat.us/200"];
        for test in tests {
            assert_eq!(is_web_link(test), true);
        }
    }

    #[test]
    fn verify_web_link_is_alive() {
        assert_eq!(is_valid_web_link("https://www.google.com"), true);
    }

    #[test]
    fn verify_local_link_exists() {
        assert_eq!(is_valid_local_link("./test/test.md"), true);
    }

    #[test]
    fn get_all_links_from_file() {
        let testfile = PathBuf::from("./test/test.md");
        let links: Vec<&str> = vec![
            "https://www.google.com",
            "https://www.google.com",
            "https://www.one.com",
            "https://www.two.com",
        ];
        assert_eq!(links, get_links_from_file(&testfile));
    }
}
