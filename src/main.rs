mod core;

use crate::core::{Config, Search};

fn main() {
    let config: Config = Config::new(std::env::args().collect::<Vec<String>>());

    println!("");

    println!("{}", Search::query(config).results());
}
