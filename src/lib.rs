use regex::Regex;

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

pub fn search(content: String, query: String) -> Result<bool, regex::Error> {
    Ok(Regex::new(query.as_str())?.is_match(content.as_str()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_match_regex() {
        let content = String::from("@amirreza@");
        let query = String::from("@.*@");
        let res = search(content, query);
        assert_eq!(true, res.is_ok());
        assert_eq!(true, res.unwrap());
    }
    #[test]
    fn test_regex_creation_failed() {
        let content = String::from("@amirreza@");
        let query = String::from("(");
        let res = search(content, query);
        assert_eq!(true, res.is_err());
    }
}