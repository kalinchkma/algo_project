use std::error::Error;
use std::{env, fs};
use std::process;

pub fn main_fn() {

    let config = Config::new(env::args()).unwrap_or_else( |err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    }); 

    println!("Searching for {}", config.query);

    println!("In file {}", config.filename);

    if let Err(e) = run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

struct Config {
    query: String,
    filename: String,
    case_sensitive: bool
}

impl Config {

    fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string")
        };
        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a filename")
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {query, filename, case_sensitive})
    }

}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {

    // let mut results = Vec::new();
    
    // for line in contents.lines() {
    
    //     if line.contains(query) {
    //         results.push(line)
    //     }
    // }

    // return results;

    
    // With iterator
    contents.lines()
    .filter(|line| line.contains(query)).collect()
}

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    
    let query = query.to_lowercase();
    
    let mut results = Vec::new();
    
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line)
        }
    }
    
    results
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn one_sensitive() {
   
        let query = "duct";
   
        let contents = "\n
Rust:
safe, fast, productive.
Pick three
Duct tape";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents))
    }

    #[test]
    fn case_insensitive() {
   
        let query = "RuSt";
   
        let contents = "\n
Rust:
safe, fast, productive.
Pick three
Trust me.";
   
        assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(query, contents))
    }
}
