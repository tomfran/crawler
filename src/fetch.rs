use html5ever::tree_builder::TreeSink;
use regex::Regex;
use reqwest::blocking::{Client, ClientBuilder};
use scraper::{Html, Selector};

use crate::simhash::simhash;

const SIMHASH_SHINGLE_SIZE: usize = 3;
static APP_USER_AGENT: &str = "https://github.com/tomfran/crawler";

#[derive(Debug)]
pub struct Webpage {
    pub url: String,
    pub raw_content: String,
    pub digest: u128,
    pub links: Vec<String>,
}

pub struct Fetcher {
    client: Client,
    scripts_sel: Selector,
    style_sel: Selector,
    a_sel: Selector,
    unicode_digit_tags_newline_re: Regex,
    multiple_space_re: Regex,
}

impl Default for Fetcher {
    fn default() -> Self {
        println!("{}", APP_USER_AGENT);
        Fetcher {
            client: ClientBuilder::new()
                .user_agent(APP_USER_AGENT)
                .build()
                .unwrap(),
            scripts_sel: Selector::parse("script").unwrap(),
            style_sel: Selector::parse("style").unwrap(),
            a_sel: Selector::parse("a").unwrap(),
            unicode_digit_tags_newline_re: Regex::new(r"<[^>]*>|[^\x00-\x7F]+|\d+|\n").unwrap(),
            multiple_space_re: Regex::new(r"\s+").unwrap(),
        }
    }
}

impl Fetcher {
    pub fn fetch(&self, url: &str) -> Option<Webpage> {
        let response = self.client.get(url).send().ok()?;
        let body = response.text().ok()?;

        let document = Html::parse_document(&body);

        let url = url.to_string();

        let mut links: Vec<_> = document
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
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();

        links.dedup();

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

        simhash(&strip, SIMHASH_SHINGLE_SIZE)
    }
}
