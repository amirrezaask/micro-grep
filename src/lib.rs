use std::fs;

pub struct Config {
    pub filename: String,
    pub query: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("need filename and query")
        }
        Ok(Config{
            filename: args[1].clone(),
            query: args[2].clone(),
        })
    }
}

pub fn search(content: String, query: String) -> Result<String, &'static str> {

}