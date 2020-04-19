#![allow(unused_variables, dead_code)]
use std::path::PathBuf;

use glob::glob;
use structopt::StructOpt;

#[macro_use]
extern crate lazy_static;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(StructOpt, Debug)]
struct Opts {
    /// Files to check links in
    files: Vec<std::path::PathBuf>,

    /// Only check local links
    #[structopt(short, long)]
    local: bool,

    /// Only check web links
    #[structopt(short, long)]
    web: bool,
}

fn main() -> Result<()> {
    let args = Opts::from_args();
    let files = if args.files.is_empty() {
        get_md_files_in_curdir()?
    } else {
        args.files
    };
    for filename in files {
        let lt = if args.local {
            Some(links::LinkType::Local)
        } else if args.web {
            Some(links::LinkType::Web)
        } else {
            None
        };
        let links: Vec<links::Link> = links::from_file(&filename, lt)
            .iter()
            .filter(|l| !l.is_alive())
            .map(|x| x.to_owned())
            .collect();

        let fn_str = filename.to_string_lossy().to_string();

        for link in links {
            println!("{}:{:?}:{}", fn_str, link.linktype, link.text);
        }
    }
    Ok(())
}

/// Glob for markdown files under the current working directory
fn get_md_files_in_curdir() -> Result<Vec<PathBuf>> {
    Ok(glob("**/*.md")?
        .filter(|x| x.is_ok())
        .map(|x| x.expect("Already tested each glob is ok"))
        .collect())
}

mod links {
    use regex::Regex;
    use std::collections::HashSet;
    use std::path::PathBuf;

    #[derive(Eq, PartialEq, Debug, Clone)]
    pub enum LinkType {
        Web,
        Local,
    }

    #[derive(Debug, Clone)]
    pub struct Link {
        pub text: String,
        source: String,
        pub linktype: LinkType,
    }

    pub fn from_file(filename: &PathBuf, linktype: Option<LinkType>) -> Vec<Link> {
        lazy_static! {
            static ref RE_WEB: Regex = Regex::new(
                r#"(https+://)*[-a-zA-Z0-9@:%._/\+~#=]{1,256}\.[a-zA-Z0-9()]{1,6}\b[-a-zA-Z0-9@:%_\+.~#?&//=]*"#
            )
            .unwrap();
            static ref RE_LOCAL: Regex = Regex::new(r#"\((?:\.+/)*([a-zA-Z0-9\-_ ]*?\.md)"#).unwrap();
        }
        let contents = std::fs::read_to_string(filename).unwrap();
        let mut links = Vec::new();
        let fn_str = filename.to_string_lossy().to_string();
        let mut seen = HashSet::new();

        if linktype == None || linktype == Some(LinkType::Local) {
            for cap in RE_LOCAL.captures_iter(&contents) {
                let linktext: String = if cap[0].starts_with("(") {
                    cap[0][1..].into()
                } else {
                    cap[0].into()
                };
                if seen.contains(&linktext) {
                    continue;
                } else {
                    seen.insert(linktext.clone());
                }
                links.push(Link {
                    text: linktext.clone(),
                    linktype: LinkType::Local,
                    source: fn_str.clone(),
                });
            }
        }

        if linktype == None || linktype == Some(LinkType::Web) {
            for cap in RE_WEB.captures_iter(&contents) {
                let linktext: String = cap[0].into();
                if seen.contains(&linktext) {
                    continue;
                } else {
                    seen.insert(linktext.clone());
                }
                links.push(Link {
                    text: linktext.clone(),
                    linktype: LinkType::Web,
                    source: fn_str.clone(),
                });
            }
        }
        links
    }

    impl Link {
        pub fn is_alive(&self) -> bool {
            if self.linktype == LinkType::Local {
                self.is_valid_local_link()
            } else {
                self.is_valid_web_link()
            }
        }

        pub fn is_local(&self) -> bool {
            self.linktype == LinkType::Local
        }

        pub fn is_web(&self) -> bool {
            self.linktype == LinkType::Web
        }

        fn is_valid_local_link(&self) -> bool {
            let source = std::path::PathBuf::from(&self.source);
            let parent = source.parent().unwrap();
            let p = parent.join(&self.text).canonicalize();
            if let Err(e) = p {
                false
            } else {
                p.unwrap().exists()
            }
        }

        fn is_valid_web_link(&self) -> bool {
            match reqwest::blocking::get(&self.text) {
                Ok(resp) => resp.status() == 200,
                Err(e) => false,
            }
        }
    }
}
