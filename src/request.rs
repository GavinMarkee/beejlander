/*
Request and Async module for the beejlander package
*/

use regex::Regex;
use std::collections::HashMap;
use std::error::Error;
use super::Card;
use super::query::Query;

struct Params {
    base: String,
    query: String,
    re: Regex
}

pub async fn fetch_cards(queries: Query) -> Result<HashMap<String, Card>, Box<dyn Error>> {
    let mut list = HashMap::new();
    let mut params = Params{
        base: String::from("https://api.scryfall.com/cards/random"),
        query: format!("{}", queries.rare),
        re: Regex::new(r"(?i)(\w+\s{1})?(l{1})and(\s{1}.+)?").unwrap()
    };

    let rare_card = fetch_card(&params).await?;
    list.insert(format!("{}", rare_card.name), rare_card);

    params.query = format!("{}", queries.other);
    fetch_nonrares(&params, &mut list).await?;

    Ok(list)
}

async fn fetch_nonrares(params: &Params, list: &mut HashMap<String, Card>) -> Result<(), Box<dyn Error>> {
    let mut total_count: u8 = 0;
    let mut duplicates: u8 = 0;

    while total_count < 99 {
        let card = fetch_card(&params).await;
        let card = match card {
            Ok(c) => c,
            Err(_e) => panic!("Scryfall timed out while fetching cards"),
        };
        let name = format!("{}", card.name);
        if list.contains_key(&name) && duplicates < 5 {
            let prev_card = list.get_mut(&name).unwrap();
            prev_card.count += 1;
            duplicates += 1;
        }
        else if list.contains_key(&name) && duplicates >= 5 {
            continue;
        }
        else {
            list.insert(name, card);
        }
        total_count += 1;
        if (total_count + 1) % 10 == 0 {
            println!("{}/100", total_count + 1);
        }
    }

    Ok(())
}

async fn fetch_card(params: &Params) -> Result<Card, Box<dyn Error>> {
    let url = format!("{}?{}", params.base, params.query);
    let response = surf::get(&url)
        .recv_string()
        .await;
    let response = match response {
        Ok(s) => s,
        Err(_e) => panic!("Scryfall timed out while fetching cards"),
    };

    let mut lines = response.lines();
    let title_line_vec: Vec<&str> = lines.next()
        .unwrap()
        .split(" ")
        .collect();
    let type_line = lines.next().unwrap().to_string();
    let tile_line_len = title_line_vec.len();

    let mut name = String::from("");
    let mut cmc = String::from("");
    if tile_line_len == 1 {
        name.push_str(title_line_vec.get(0).unwrap())
    }
    else if params.re.is_match(&type_line) {
        for name_part in 0..tile_line_len {
            if name_part != 0 {
                name.push(' ');
            }
            name.push_str(title_line_vec.get(name_part).unwrap());
        }
    }
    else {
        for name_part in 0..(tile_line_len - 1) {
            if name_part != 0 {
                name.push(' ');
            }
            name.push_str(title_line_vec.get(name_part).unwrap());
        }
        cmc.push_str(title_line_vec.get(tile_line_len - 1).unwrap());
    }
    let card = Card {
        name,
        cmc,
        type_line,
        count: 1
    };
    Ok(card)
}