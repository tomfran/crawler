use html5ever::tree_builder::TreeSink;
use regex::Regex;
use reqwest::blocking::get;
use scraper::{Html, Selector};

use crate::simhash::simhash;

const SIMHASH_SHINGLE_SIZE: usize = 3;

#[derive(Debug)]
pub struct Webpage {
    pub url: String,
    pub raw_content: String,
    pub fingerprint: u128,
    pub links: Vec<String>,
}

pub fn fetch(url: &str) -> Option<Webpage> {
    let response = get(url).ok()?;
    let body = response.text().ok()?;

    let document = Html::parse_document(&body);

    let a_sel = Selector::parse("a").unwrap();
    let url = url.to_string();

    let links = document
        .select(&a_sel)
        .filter_map(|e| e.value().attr("href"))
        .map(str::to_string)
        .map(|s| {
            if s.starts_with("http") {
                s
            } else {
                url.clone() + &s
            }
        })
        .collect();

    let raw_content = document.html();
    let fingerprint = compute_digest(document);

    Some(Webpage {
        url,
        raw_content,
        fingerprint,
        links,
    })
}

fn compute_digest(mut document: Html) -> u128 {
    // remove scripts, styles and links
    let scripts_sel = &Selector::parse("script").unwrap();
    let style_sel = &Selector::parse("style").unwrap();
    let a_sel = &Selector::parse("a").unwrap();

    let r1 = document.select(scripts_sel);
    let r2 = document.select(style_sel);
    let r3 = document.select(a_sel);

    let node_ids: Vec<_> = r1.chain(r2).chain(r3).map(|x| x.id()).collect();
    for id in node_ids {
        document.remove_from_parent(&id);
    }

    // remove all tags and non-asci chars
    let re = Regex::new(r"<[^>]*>|[^\x00-\x7F]+").unwrap();
    let strip = re.replace_all(document.html().as_str(), "").to_string();

    simhash(strip, SIMHASH_SHINGLE_SIZE)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test() {
        assert!(fetch("https://tomfran.github.io/about/").is_some());
    }
}
