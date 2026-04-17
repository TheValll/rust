use std::{env, process, error::Error};
use qcd::search;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err|{
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = run(config){
        eprint!("Application error: {}", e);
        process::exit(1);
    }

}

fn run(config: Config)-> Result<(), Box<dyn Error>>{
    let results = search(&config.query);

    for line in results{
        println!("{}", line);
    }

    Ok(())
}

struct Config {
    query: String,
}

impl Config{
    fn build(args: &[String])-> Result<Config, &'static str>{
        if args.len() < 1{
            return Err("not enough arguments");
        }

        let query = args[1].clone();

        Ok(Config { query })
    }
}