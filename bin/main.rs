use anyhow::Result;
use glob::glob;
use rayon::prelude::*;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "mdlc", about = "Markdown Link Checker")]
struct Opt {
    /// Files to check (defaults to **/*.md)
    files: Vec<String>,
    /// Only check local links (links to files)
    #[structopt(conflicts_with("web"), short = "l", long = "local")]
    local: bool,
    /// Only check web links
    #[structopt(conflicts_with("local"), short = "w", long = "web")]
    web: bool,
}

fn main() -> Result<()> {
    let args = Opt::from_args();
    let files = if !args.files.is_empty() {
        args.files
    } else {
        get_md_files_in_curdir()?
    };
    let linkfilter = if args.local {
        Some(mdlc::LinkType::Local)
    } else if args.web {
        Some(mdlc::LinkType::Web)
    } else {
        None
    };

    files.par_iter().for_each(|filename| {
        for link in mdlc::from_file(filename) {
            if !(linkfilter == None || link.linktype == linkfilter.unwrap()) {
                continue;
            }
            if !link.is_alive() {
                println!("{}:{:?}:{}", filename, link.linktype, link.text);
            }
        }
    });
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
