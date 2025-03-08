use std::env;
use std::process;
use std::fs;
use std::error::Error;
use regex::Regex;
use std::collections::HashMap;

mod read_file;
mod parse_json;

const USER_DB_PATH: &str = "/home/voitec/Rust/users.json";

struct Cli {
    feature: String,
    server: String,
    path: String,
}

impl Cli{
    fn build(mut args: impl Iterator<Item = String>) -> Result<Cli, &'static str> {
        args.next();
        let feature = match args.next() {
            Some(feature) => feature,
            None => return Err("feature not defined"),
        };
        let server = match args.next() {
            Some(server) => server,
            None => return Err("server not defined"),
        };
        let path = match args.next() {
            Some(path) => path,
            None => return Err("path not defined"),
        };
        if let Some(_) = args.next() {
            return Err("two many parameters given, exactly 3 must be defined");
        }
        Ok(Cli { feature, server, path, })   
    }
}

fn run(cli: &Cli, user_db: &HashMap<String, String>, usage_file_content: &str) -> Result<(), Box<dyn Error>> {
    let mut serv_found: bool = false;
    let mut feature_found: bool = false;
    let feature_string = format!("Feature: {}", &cli.feature);

    for line in usage_file_content.lines() {
        if serv_found == false {
            if line == format!("LM-X License Server on 6200@{}:", &cli.server) {
                println!("Requested server found: {}", cli.server);
                serv_found = true;
                continue; 
            }
        }
        if serv_found == true {
            if feature_found == false {
                if line.contains(&feature_string) {
                    println!("Requested feature found: {}", cli.feature);
                    feature_found = true;
                    continue; 
                }
            }
            if feature_found == true {
                let re = Regex::new(r"^[0-9]+ license\(s\) used by");
                if re?.is_match(&line) {
                    println!("Matched line: {}", line);
                    
                    let line_splitted = line.split_whitespace();
                    for chunk in line_splitted {
                        println!("{chunk}")
                    }

                }
                if line.contains("Feature: ") {
                    println!("Another feature encountered");
                    return Ok(());
                }
            }
            if line.contains("LM-X License Server on") {
                println!("Another server encountered");
                return Ok(());
            }
        }
    }

    if serv_found == false {
        return Err("Requested server not found")?;
    }
    if feature_found == false {
        return Err("Requested feature not found")?;
    }
    Ok(())
} 

fn main() {
    let cli = Cli::build(env::args()).unwrap_or_else(|error| {
        println!("Error while collecting input parameters: {error}");
        process::exit(1);
    });

    let user_db_json = match read_file::read_file(USER_DB_PATH) {
        Ok(user_db_json) => user_db_json,
        Err(e) => {
            eprintln!("Error while retrieving User DB: {e}");
            process::exit(1);
        }
    };
    
    let user_db = match parse_json::parse_json(&user_db_json) {
        Ok(user_db) => user_db,
        Err(e) => {
            eprintln!("Error while parsing json: {e}");
            process::exit(1);
        }
    };

    let usage_file_content = match read_file::read_file(&cli.path) {
        Ok(usage_file_content) => usage_file_content,
        Err(e) => {
            eprintln!("Error while retrieving usage file content: {e}");
            process::exit(1);
        }
    };

    if let Err(e) = run(&cli, &user_db, &usage_file_content) {
        eprintln!("Unable to parse the file: {e}");
        process::exit(1);
    }
}
