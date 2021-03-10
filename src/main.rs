use std::env;

use glob::glob;

#[macro_use]
extern crate lazy_static;

mod links;

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
    Ok(Args { files, linktype })
}

fn main() -> Result<()> {
    let args = parse_args()?;
    let linkfilter = args.linktype;

    for filename in args.files {
        let mut links: Vec<links::Link> = Vec::new();
        for link in links::from_file(&filename) {
            if !(linkfilter == None || link.linktype == linkfilter.unwrap()) {
                continue;
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
        .map(|x| {
            x.expect("Already tested each glob is ok")
                .to_string_lossy()
                .to_string()
        })
        .collect())
}
