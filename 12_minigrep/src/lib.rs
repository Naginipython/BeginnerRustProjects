//Seems I can just create a lib.rs and start throwing things inside, adding "pub" to grant access. I will need to remember the named used
//for the project, I have been renaming the files using 'mv'. The name is in Cargo.toml, though
use std::fs;
use std::error::Error;

pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    //"build" instead of "new" because devs don't expect new to fail
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        //using clone isn't ideal, but since this is a smaller program, it's fine.
        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    println!("With text:\n{contents}");

    Ok(())
}