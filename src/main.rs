use crawler::crawler::Crawler;
use log::info;

fn main() {
    env_logger::init();

    info!("Crawler - Start");

    let c = Crawler::new(1, 10_000_000);
    c.set_seed(vec!["https://github.com/tomfran/LSM-Tree".to_string()]);
    c.start();
}
