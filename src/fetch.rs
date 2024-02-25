use std::io::Seek;

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
    pub digest: u128,
    pub links: Vec<String>,
}

pub struct Fetcher {
    scripts_sel: Selector,
    style_sel: Selector,
    a_sel: Selector,
    unicode_digit_tags_newline_re: Regex,
    multiple_space_re: Regex,
}

impl Fetcher {
    fn new() -> Fetcher {
        Fetcher {
            scripts_sel: Selector::parse("script").unwrap(),
            style_sel: Selector::parse("style").unwrap(),
            a_sel: Selector::parse("a").unwrap(),
            unicode_digit_tags_newline_re: Regex::new(r"<[^>]*>|[^\x00-\x7F]+|\d+|\n").unwrap(),
            multiple_space_re: Regex::new(r"\s+").unwrap(),
        }
    }

    pub fn fetch(&self, url: &str) -> Option<Webpage> {
        let response = get(url).ok()?;
        let body = response.text().ok()?;

        let document = Html::parse_document(&body);

        let url = url.to_string();

        let links = document
            .select(&self.a_sel)
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
        let digest = self.compute_digest(document);

        Some(Webpage {
            url,
            raw_content,
            digest,
            links,
        })
    }

    fn compute_digest(&self, mut document: Html) -> u128 {
        // remove scripts, styles and links
        let r1 = document.select(&self.scripts_sel);
        let r2 = document.select(&self.style_sel);
        let r3 = document.select(&self.a_sel);

        let node_ids: Vec<_> = r1.chain(r2).chain(r3).map(|x| x.id()).collect();
        for id in node_ids {
            document.remove_from_parent(&id);
        }

        // remove all tags, unicodes, numbers and newlines
        let mut strip = self
            .unicode_digit_tags_newline_re
            .replace_all(document.html().as_str(), " ")
            .to_string();

        // remove consecutive spaces
        strip = self.multiple_space_re.replace_all(&strip, " ").to_string();

        simhash(strip, SIMHASH_SHINGLE_SIZE)
    }
}

#[cfg(test)]
mod tests {
    use crate::simhash::hamming_distance;

    use super::*;

    #[test]
    pub fn test() {
        let f = Fetcher::new();

        // same article on two websites
        let w1 = f
            .fetch("https://itnext.io/log-structured-merge-tree-a79241c959e3")
            .unwrap();
        let w2 = f.fetch("https://tomfran.github.io/posts/lsm/").unwrap();

        assert!(hamming_distance(w1.digest, w2.digest) < 5);
    }
}
