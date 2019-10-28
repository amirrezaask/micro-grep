extern crate regex;

mod config;
mod re;

use config::Config;
use std::fs;
use std::process::exit;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let config = Config::new(&args).expect("failed to create config");

    let content = fs::read_to_string(&config.filename)
        .expect("cannot read file");
    let res = re::Re::new(&content, &config.query);
    if res.is_ok() {
        let re = res.unwrap();
        if !re.is_match() {
            println!("Query does not match given content");
            exit(0)
        }
        println!("{:?}", re.extract().unwrap());
    }
}
