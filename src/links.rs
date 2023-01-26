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
            r#"(?:https?://|ftp://|news://|mailto:|\bwww\.)[a-zA-Z0-9\-@;/?:&=%\$_.+!*\x27,~#]*(\([a-zA-Z0-9\-@;/?:&=%\$_.+!*\x27,~#]*\)|[a-zA-Z0-9\-@;/?:&=%\$_+*~])+"#,
        )
        .unwrap();
        static ref RE_MD: Regex = Regex::new(r#"\[.+?\](?:\(|: )(.+?.md)(?:\)*)"#).unwrap();
    }
    let contents = std::fs::read_to_string(filename).unwrap();
    let mut links = Vec::new();
    let mut seen = HashSet::new();

    for cap in RE_MD.captures_iter(&contents) {
        let linktext: String = cap[1].into();
        if matches_heuristic(&linktext) {
            continue;
        }
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
        if matches_heuristic(&linktext) {
            continue;
        }
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

fn matches_heuristic(link: &str) -> bool {
    let low = link.to_lowercase();
    if link.contains("...") {
        true
    } else if low.contains("e.g.") {
        true
    } else if low.contains("i.e.") {
        true
    } else if low.contains("b.c.") {
        true
    } else {
        false
    }
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
