use crawler::crawler::Crawler;
use log::{info, LevelFilter};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let ascii_art = r#"
   +-----------------------------------------------------------
   :". /  /  /                      .           
   :.-". /  /     ,-. ,-. ,-. . , , |  ,-. ,-.     
   : _.-". /      |   |   ,-| |/|/  |  |-' |        
   :"  _.-".      `-' '   `-^ ' '   `' `-' '   
   :-""     ".  
   :                 
 ^.-.^
'^\+/^`
'/`"'\`      
"#;

    println!("{}", ascii_art);

    if args.len() < 4 {
        println!(
            "Usage: {} <number_of_workers> <sieve_size> <seed_website1> [<seed_website2> ...]",
            args[0]
        );
        return;
    }

    let number_of_workers: usize = args[1].parse().expect("Invalid number of workers");
    let sieve_size: usize = args[2].parse().expect("Invalid sieve size");
    let seed_websites: Vec<String> = args[3..].to_vec();

    env_logger::Builder::new()
        .filter_level(LevelFilter::Info) // Set the log level to Info
        .init();

    info!(
        "Crawler - {} workers, sieve size {}, seed {:?}",
        number_of_workers, sieve_size, seed_websites
    );

    let c = Crawler::new(number_of_workers, sieve_size);
    c.set_seed(&seed_websites);
    c.start();
}
