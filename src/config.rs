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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_config_len_less_than_3() {
        assert_eq!(false, Config::new(&[String::from(""),String::from("lyrics.txt")]).is_ok())
    }
    #[test]
    fn test_config_ok() {
        let config = Config::new(&[String::from(""),String::from("lyrics.txt"), String::from(".*")]);
        
        let is_ok = config.is_ok();
        let config = config.unwrap();
        let filename = config.filename;
        let query = config.query;
        
        assert_eq!(true, is_ok);
        assert_eq!("lyrics.txt", filename);
        assert_eq!(".*", query);
    }
}