use crawler::crawler::Crawler;
use log::info;

fn main() {
    env_logger::init();

    info!("Crawler - Start");

    let c = Crawler::new(1, 1000);
    c.set_seed(vec!["https://en.wikipedia.org/wiki/Main_Page".to_string()]);
    c.start();
}
