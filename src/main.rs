use std::env;
use std::process;
use std::error::Error;
use regex::Regex;
use std::collections::HashMap;

mod read_file;
mod parse_json;

const USER_DB_PATH: &str = "/home/voitec/Rust/users.json";

#[derive(Debug)]
struct Site<'a> {
//    name: &'a str,
    usage: u32,
    users: Vec<&'a str>,
}

impl <'a> Site <'a> {
    fn new(lic_count: u32, user_name: &str) -> Site {
        Site {
            usage : lic_count,
            users : vec![user_name],
        }
    }
    fn add(&mut self, val: u32) {
        self.usage += val;
    }
    fn push_user(&mut self, user_name: &'a str) {
        self.users.push(user_name);
    }
}

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
    let mut found_sites: HashMap<&str, Site> = HashMap::new();

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
                    let line_splitted: Vec<&str> = line.split_whitespace().collect();
                    if line_splitted.len() != 6 {
                        return Err("Wrong file format, line length should be equal 6")?;
                    }

                    let lic_count: u32 = line_splitted[0].parse()?;

                    let user_string: Vec<&str>= line_splitted[4].split("@").collect();
                    if user_string.len() != 2 {
                        return Err("Format of the user string invalid")?;
                    }
                    let user_name = user_string[0];

                    let user_site = match user_db.get(user_name) {
                        Some(site) => site,
                        None => "unknown" 
                    };
                   
                    found_sites.entry(user_site).and_modify(|e| {e.add(lic_count); e.push_user(user_name)}).or_insert(Site::new(lic_count, user_name));
                       
                }
                if line.contains("Feature: ") {
                    println!("Another feature encountered");

                    for site in &found_sites {
                        dbg!(site);
                    }

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
