use crawler::crawler::Crawler;
use log::info;

fn main() {
    env_logger::init();

    info!("Crawler - Start");

    let seed = ["http://www.odp.org/homepage.php"];

    let c = Crawler::new(5);
    c.set_seed(&seed.iter().map(|s| s.to_string()).collect::<Vec<String>>());
    c.start();
}
