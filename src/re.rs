use regex::Regex;
use std::collections::HashMap;

pub struct Re<'a> {
    content: &'a String,
    re: Regex,
}

impl<'a> Re<'a> {
    pub fn new(content: &'a String, query: &str) -> Result<Re<'a>, regex::Error> {
        Ok(Re{
            content: content,
            re: Regex::new(query)?,
        })
    }
    pub fn is_match(&self) -> bool {
        self.re.is_match(self.content.as_str())
    }
    pub fn extract(&self) -> Option<regex::Captures> {
        self.re.captures(self.content)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_re(){
        let content = String::from("@amirreza@");
        assert_eq!(true, Re::new(&content, "@.*@").is_ok());
        assert_eq!(true, Re::new(&content, "(").is_err());
    }

    #[test]
    fn test_match_regex() {
        let content = String::from("@amirreza@");
        let re = Re::new(&content, "@.*@").unwrap();
        assert_eq!(true, re.is_match());
    }
}