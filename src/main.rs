#![allow(unused_variables, dead_code)]
use std::collections::HashMap;
use std::path::PathBuf;

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
    unimplemented!()
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
    unimplemented!()
}

fn is_valid_web_link(l: &str) -> bool {
    unimplemented!()
}
