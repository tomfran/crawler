use crawler::crawler::Crawler;
use log::info;

fn main() {
    env_logger::init();

    info!("Crawler - Start");

    let news_sites = [
        "https://www.nytimes.com/",
        "https://www.bbc.com/news",
        "https://edition.cnn.com/",
        "https://www.theguardian.com/international",
        "https://www.aljazeera.com/",
        "https://www.reuters.com/",
        "https://www.washingtonpost.com/",
        "https://www.wsj.com/",
        "https://www.npr.org/",
        "https://apnews.com/",
        "https://abcnews.go.com/",
        "https://www.foxnews.com/",
        "https://www.usatoday.com/",
        "https://www.euronews.com/",
        "https://www.huffpost.com/",
        "https://www.telegraph.co.uk/",
        "https://www.economist.com/",
        "https://www.ft.com/",
        "https://time.com/",
        "https://www.nbcnews.com/",
    ];

    let c = Crawler::new(10);
    c.set_seed(
        &news_sites
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>(),
    );
    c.start();
}
