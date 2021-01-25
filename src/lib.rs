use std::fs;
use std::fmt;
use std::error;
use std::time::Instant;
use std::env;


pub struct Config {
    query: String,
    filename: String,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();
        // Performance Tweak to use iterators 
        // Clone is too slow
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Query string is missing")
        };

        let filename = match args.next() {
            Some(arg) => arg, 
            None => return Err("Filename is missing")
        };

        Ok(Config{
            query,
            filename,
        })
    }
}

impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        
        write!(f, "Filename: {} Query: {}", self.filename, self.query)
    }
}



pub fn run(config: Config) -> Result<(), Box<dyn error::Error>>{
    let now = Instant::now();

    let contents = fs::read_to_string(&config.filename)?;
    let size = fs::metadata(&config.filename)?.len();
    let lines = search(&config.query, &contents);
    let len = lines.len();
    println!("{}", lines.join("\n")); // Performance Tweak to avoid the stdout flush

    println!("\nFilename: {:?}, Search Term: {:?} Filesize: {:?} Kb\nMatched Lines: {:?}, Excecution Time: {:?} ms\n", 
             config.filename, config.query ,size / 1000, len, now.elapsed().as_millis());
    Ok(())
}

fn search<'a>(query:&str, contents: &'a str) -> Vec<&'a str> {
    // Performance Tweak to use sequentials iterators    
    contents
        .lines()
        .filter(|x| x.contains(query))
        .collect()
    
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust
Safe, fast, productive
pick three.";

        assert_eq!(vec!["Safe, fast, productive"], search(query, contents))
    }
}