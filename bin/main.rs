#[macro_use]
extern crate clap;

use anyhow::Result;
use clap::{App, Arg};
use glob::glob;

use mdlc;

fn main() -> Result<()> {
    let app = App::new("mdlc - Markdown Link Checker")
        .version(crate_version!())
        .arg(
            Arg::with_name("local_links")
                .short("l")
                .long("--local")
                .help("Only check LOCAL links")
                .conflicts_with("web_links"),
        )
        .arg(
            Arg::with_name("web_links")
                .short("w")
                .long("--web")
                .help("Only check web links")
                .conflicts_with("web_links"),
        )
        .arg(
            Arg::with_name("FILES")
                .multiple(true)
                .help("Files to check"),
        )
        .get_matches();
    let files = app
        .values_of("FILES")
        .map(|x| x.map(|x| x.to_string()).collect::<Vec<String>>())
        .unwrap_or(get_md_files_in_curdir()?);
    let linkfilter = if app.is_present("local_links") {
        Some(mdlc::LinkType::Local)
    } else if app.is_present("web_links") {
        Some(mdlc::LinkType::Web)
    } else {
        None
    };

    for filename in files {
        for link in mdlc::from_file(&filename) {
            if !(linkfilter == None || link.linktype == linkfilter.unwrap()) {
                continue;
            }
            if !link.is_alive() {
                println!("{}:{:?}:{}", filename, link.linktype, link.text);
            }
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
