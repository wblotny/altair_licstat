use serde::Deserialize;
use serde_json;
use std::error::Error;
use std::collections::HashMap;

#[derive(Deserialize)]
struct User {
    username: String,
    location: String,
}

pub fn parse_json(file_content: &str) -> Result<HashMap<String,String>, Box<dyn Error>> {
    let users: Vec<User> = serde_json::from_str(&file_content)?;

    let mut user_dir = HashMap::new();
    for user in users {
        user_dir.insert(user.username, user.location);
    }
//    dbg!(&user_dir);
    Ok(user_dir)
}
