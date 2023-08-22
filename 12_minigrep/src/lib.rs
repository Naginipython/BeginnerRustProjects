//Seems I can just create a lib.rs and start throwing things inside, adding "pub" to grant access. I will need to remember the named used
//for the project, I have been renaming the files using 'mv'. The name is in Cargo.toml, though
use std::fs;
use std::error::Error;
use std::env;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    //"build" instead of "new" because devs don't expect new to fail
    pub fn build(
        mut args: impl Iterator<Item = String>,
    ) -> Result<Config, &'static str> {
        // OLD WAY
        // if args.len() < 3 {
        //     return Err("Not enough arguments");
        // }
        // //using clone isn't ideal, but since this is a smaller program, it's fine. [Fixed in NEW WAY]
        // let query = args[1].clone();
        // let file_path = args[2].clone();

        //NEW WAY
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };

        let ignore_case = env::var("IGNORED_CASE").is_ok();

        Ok(Config { query, file_path, ignore_case })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}

//'a is a lifetime value. I don't fully understand this yet. Supposedly,
//we tell Rust that the data returned by the "search" function will live as long as the data passed into the "search" function in the "content" argument
//Essentially, content is getting sliced and referenced, so content needs to still exist beyond this, maybe
//re-read 10.3
pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    //OLD WAY
    // let mut results = Vec::new();

    // for line in content.lines() {
    //     if line.contains(query) {
    //         results.push(line);
    //     }
    // }
    
    // results

    //NEW WAY
    content
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in content.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }
    
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let content = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";
        assert_eq!(vec!["safe, fast, productive."], search(query, content));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let content = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(query, content));
    }
}