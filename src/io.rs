/*
File IO module for the beejlander package
*/

use std::collections::HashMap;
use std::fs;
use std::io::Error;
use super::Card;

pub fn save_to_file(list: &HashMap<String, Card>) -> Result<String, Error> {
    let mut file_string = String::from("");
    for (_key, value) in list {
        file_string.push_str(&format!("{}x {}\n", value.count, value.name));
    }
    fs::write("./cards.txt", file_string).expect("Error - Unable to save file");
    Ok(format!("File saved"))
}