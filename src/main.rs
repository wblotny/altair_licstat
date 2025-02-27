use std::env;
use std::process;

struct Config {
    feature: String,
    path: String,
}

impl Config {
    fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {

        args.next();

        let feature = match args.next() {
            Some(feature) => feature,
            None => return Err("feature not defined"),
        };

        let path = match args.next() {
            Some(path) => path,
            None => return Err("path not defined"),
        };

        Ok(Config { feature, path })   
    }
}

fn main() {
    let cfg = Config::build(env::args()).unwrap_or_else(|error| {
        println!("Error while collecting input parameters: {error}");
        process::exit(1);
    });

    println!("{}", cfg.feature);
    println!("{}", cfg.path);
}
