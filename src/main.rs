use std::env;

use glob::glob;

#[macro_use]
extern crate lazy_static;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

struct Args {
    files: Vec<String>,
    linktype: Option<links::LinkType>,
}

fn parse_args() -> Result<Args> {
    let mut files = Vec::new();
    let mut linktype: Option<links::LinkType> = None;
    for a in env::args().skip(1) {
        if !a.starts_with("-") {
            files.push(a);
        } else {
            if a == "-l" || a == "--local" {
                linktype = Some(links::LinkType::Local);
            } else if a == "-w" || a == "--web" {
                linktype = Some(links::LinkType::Web);
            } else {
                println!("Unrecognised Flag {}", a);
            }
        }
    }
    if files.is_empty() {
        files = get_md_files_in_curdir()?
    }
    Ok(Args{files, linktype})
}

fn main() -> Result<()> {
    let args = parse_args()?;
    let linkfilter = args.linktype;

    for filename in args.files {
        let mut links: Vec<links::Link> = Vec::new();
        for link in links::from_file(&filename) {
            if !(linkfilter == None || link.linktype == linkfilter.unwrap()) {
                continue
            }
            if !link.is_alive() {
                links.push(link);
            }
        }

        for link in links {
            println!("{}:{:?}:{}", filename, link.linktype, link.text);
        }
    }
    Ok(())
}

/// Glob for markdown files under the current working directory
fn get_md_files_in_curdir() -> Result<Vec<String>> {
    Ok(glob("**/*.md")?
        .filter(|x| x.is_ok())
        .map(|x| x.expect("Already tested each glob is ok").to_string_lossy().to_string())
        .collect())
}

mod links {
    use regex::Regex;
    use std::collections::HashSet;

    #[derive(Eq, PartialEq, Debug, Clone, Copy)]
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

    pub fn from_file(filename: &str) -> Vec<Link> {
        lazy_static! {
            static ref RE_WEB: Regex = Regex::new(
                r#"(https+://)*[-a-zA-Z0-9@:%._/\+~#=]{1,256}\.[a-zA-Z0-9()]{1,6}\b[-a-zA-Z0-9@:%_\+.~#?&//=]*"#
            )
            .unwrap();
            static ref RE_LOCAL: Regex = Regex::new(r#"\((?:\.+/)*([a-zA-Z0-9\-_ ]*?\.md)"#).unwrap();
        }
        let contents = std::fs::read_to_string(filename).unwrap();
        let mut links = Vec::new();
        let mut seen = HashSet::new();

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
                source: filename.to_string(),
            });
        }

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
                source: filename.to_string(),
            });
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

        fn is_valid_local_link(&self) -> bool {
            let source = std::path::PathBuf::from(&self.source);
            let parent = source.parent().unwrap();
            let p = parent.join(&self.text).canonicalize();
            if let Err(_e) = p {
                false
            } else {
                p.unwrap().exists()
            }
        }

        fn is_valid_web_link(&self) -> bool {
            match reqwest::blocking::get(&self.text) {
                Ok(resp) => resp.status() == 200,
                Err(_e) => false,
            }
        }
    }
}
