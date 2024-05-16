use clap::Parser;
use crawler::crawler::Crawler;
use log::{info, LevelFilter};

#[derive(Parser, Debug)]
#[command(about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = 3)]
    workers: usize,

    #[arg(short, long, default_value_t = 100_000)]
    urls_max_number: usize,

    #[arg(short, long, value_parser, num_args = 1.., value_delimiter = ' ')]
    seed: Vec<String>,
}

fn main() {
    let args = Args::parse();

    env_logger::Builder::new()
        .filter_level(LevelFilter::Info)
        .init();

    info!(
        "Crawler - {} workers, sieve size {}, seed {:?}",
        args.workers, args.urls_max_number, args.seed
    );

    let c = Crawler::new(args.workers, args.urls_max_number);
    c.set_seed(&args.seed);
    c.start();
}
