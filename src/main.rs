extern crate regex;

mod config;
mod grep;

use config::Config;
use std::fs;
use std::process::exit;
use crate::grep::Grep;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let config = Config::new(&args).expect("failed to create config");

    let content = fs::read_to_string(&config.filename)
        .expect("cannot read file");
    let res = grep::Grep::new(&content, &config.query);
    if res.is_ok() {
        let re: Grep = res.unwrap();
        if !re.is_match() {
            println!("Query does not match given content");
            exit(0)
        }
        println!("{:?}", re.extract().unwrap());
    }
}
