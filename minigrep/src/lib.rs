use std::fs;
use std::error::Error;
use std::env;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    
    let results = if config.case_sensitive {
        search_case_sensitive(&config.query, &contents)
    }else {
        search_case_insensitive(&config.query, &contents)
    };

    // print line that contains query
    for line in results {
        println!("{}", line); 
    }

    Ok(())
}
pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(mut args : env::Args) -> Result<Config, &'static str> {
        args.next();


        let query = match args.next() {
            Some(arg ) => arg,
            None => return Err("Didn't get a query string"),
        };

        let filename = match args.next() {
            Some(arg ) => arg,
            None => return Err("Didn't get a file name"),
        };
        
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {query, filename, case_sensitive})
    }
}

pub fn search_case_sensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line | line.contains(query)).collect()
}
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query){
            results.push(line);
        }
    }

    results
}

// test module 
#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn case_sensitive(){
        let query = "duct";
        let content = "\
            Rust:
safe, fast, productive.
Pick thre.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search_case_sensitive(query, content));
    }


    #[test]
    fn case_insensitive(){
        let query = "rUsT";
        let content = "\
Rust:
safe, fast, productive.
Pick thre.
Trust me.";

        assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(query, content));
    }
}