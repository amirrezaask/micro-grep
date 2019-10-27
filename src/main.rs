
mod lib;
use lib::Config;
use std::fs;

fn main() {
    let args: Ven<String> = std::env::args().collect();

    let config = Config::new(&args).expect("failed to create config");

    let content = fs::read_to_string(&config.filename)
        .expect("cannot read file");
    lib::search(content, config.query).expect("search failed");
}
