use std::collections::HashMap;
use std::io::Error;

use duct::cmd;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Entry {
    pub id: Option<String>,
    pub name: Option<String>,
    pub user: Option<String>,
    pub folder: Option<String>,
}

// // runs rbw list - returns the stdout as list or kills the program if an error occurs
// pub fn raw() -> Result<HashMap<String, Vec<String>>, Error> {
//     let args: Vec<String> = vec![String::from("list"), String::from("--raw")];

//     // rbw list --raw
//     let rbw_list_raw = command::no_std_in("rbw", args)?;

//     // deserializes the output from json into Entry
//     let entries: Vec<Entry> = serde_json::from_str(&rbw_list_raw)?;

//     // stores the name as key and a Vector of users as the value
//     let mut name_to_users: HashMap<String, Vec<String>> = HashMap::new();

//     // .entry prevents overwriting
//     for entry in entries {
//         // unwrap the Options first
//         if let (Some(name), Some(user)) = (entry.name, entry.user) {
//             name_to_users.entry(name).or_default().push(user);
//         }
//     }

//     Ok(name_to_users)
// }

// runs rbw list - returns the stdout as list or kills the program if an error occurs
pub fn get_name_to_users() -> Result<HashMap<String, Vec<String>>, Error> {
    let raw = cmd!("rbw", "list", "--raw").read()?;

    // deserializes the output from json into Entry
    let entries: Vec<Entry> = serde_json::from_str(&raw)?;

    // stores the name as key and a Vector of users as the value
    let mut name_to_users: HashMap<String, Vec<String>> = HashMap::new();

    // .entry prevents overwriting
    for entry in entries {
        // unwrap the Options first
        if let (Some(name), Some(user)) = (entry.name, entry.user) {
            name_to_users.entry(name).or_default().push(user);
        }
    }

    Ok(name_to_users)
}
