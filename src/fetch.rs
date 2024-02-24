use html5ever::tree_builder::TreeSink;
use reqwest::blocking::get;
use scraper::{Html, Selector};

#[derive(Debug)]
pub struct Webpage {
    pub url: String,
    pub content: String,
    pub links: Vec<String>,
}

pub fn fetch(url: &str) -> Option<Webpage> {
    let response = get(url).ok()?;
    let body = response.text().ok()?;

    let mut document = Html::parse_document(&body);

    // extract all links, the input url is added to relative ones
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

    // remove scripts, styles and links
    let scripts_sel = &Selector::parse("script").unwrap();
    let style_sel = &Selector::parse("style").unwrap();

    let r1 = document.select(scripts_sel);
    let r2 = document.select(style_sel);
    let r3 = document.select(&a_sel);

    let node_ids: Vec<_> = r1.chain(r2).chain(r3).map(|x| x.id()).collect();
    for id in node_ids {
        document.remove_from_parent(&id);
    }

    let content = document.html();

    Some(Webpage {
        url,
        content,
        links,
    })
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     pub fn test() {
//         let d = fetch("https://tomfran.github.io/posts/lsm/").unwrap();

//         println!(
//             "Url: {}\n\nContent: {}\n\nLinks: {:?}",
//             d.url, d.content, d.links
//         );
//     }
// }
