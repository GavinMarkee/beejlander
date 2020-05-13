use std::fs;
use std::error::Error;
use std::collections::HashMap;
use regex::Regex;

struct CardList {
    count: u8,
    card: Card
}

struct Card {
    name: String,
    cmc: String,
    type_line : String
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("Putting your cards into 'cards.txt'...");
    let base_url = "https://api.scryfall.com/cards/random";
    let re = Regex::new(r"(?i)(\w+\s{1})?(l{1})and(\s{1}.+)?").unwrap();
    let mut total_count: u8 = 0;
    let mut duplicates: u8 = 0;
    let mut list = HashMap::new();

    let rare_card: Card = grab_rare(&base_url, &re).await?;
    let rare_name = rare_card.name.clone();
    let rare_list = CardList {
        count: 1,
        card: rare_card
    };
    list.insert(rare_name, rare_list);
    total_count += 1;

    while total_count < 100 {
        let other_card = grab_99(&base_url, &re).await?;
        let other_name = other_card.name.clone();
        if list.contains_key(&other_name) && duplicates < 5 {
            let prev_card = list.get(&other_name).unwrap();
            let new_card_count = prev_card.count + 1;
            let new_card = Card {
                name: prev_card.card.name.clone(),
                cmc: prev_card.card.cmc.clone(),
                type_line: prev_card.card.type_line.clone()
            };
            let new_list = CardList {
                count: new_card_count,
                card: new_card
            };
            list.insert(other_name, new_list);
            duplicates += 1;
        }
        else if list.contains_key(&other_name) && duplicates >= 5 {
            continue;
        }
        else {
            let other_list = CardList {
                count: 1,
                card: other_card
            };
            list.insert(other_name, other_list);
        }
        total_count += 1;
    }

    let mut file_string = String::from("");
    for (_key, value) in list {
        file_string.push_str(&format!("{}x ", value.count)[..]);
        file_string.push_str(&format!("{}\n", value.card.name)[..]);
    }
    fs::write("./cards.txt", file_string).expect("Unable to write file");

    Ok(())
}

async fn grab_rare(base_url: &str, re: &Regex) -> Result<Card, Box<dyn Error>> {
    let query_string = "q=-t%3Aconspiracy+-t%3Acontraption+r%3Ar+usd<.25&format=text";
    let card = fetch_to_card(base_url, query_string, re).await?;
    Ok(card)
}

async fn grab_99(base_url: &str, re: &Regex) -> Result<Card, Box<dyn Error>> {
    let query_string = "q=-t%3Aconspiracy+-t%3Acontraption+%28%28-t%3Aland+r<rare+usd<.11%29+or+%28t%3Aland+usd<.2%29%29&format=text";
    let card = fetch_to_card(base_url, query_string, re).await?;
    Ok(card)
}

async fn fetch_to_card(base_url: &str, query: &str, re: &Regex) -> Result<Card, Box<dyn Error>> {
    let response = reqwest::get(&format!("{}?{}", base_url, query))
        .await?
        .text()
        .await?;
    let mut lines = response.lines();
    let name_cmc = lines.next().unwrap();
    let title_vec: Vec<&str> = name_cmc.split(" ").collect();
    let vec_len = title_vec.len();
    let mut name = String::from("");
    let mut cmc = String::from("");
    let type_line = lines.next().unwrap().to_string();
    if vec_len == 1 {
        name.push_str(title_vec.get(0).unwrap())
    }
    else if re.is_match(&type_line[..]) {
        for name_part in 0..vec_len {
            if name_part != 0 {
                name.push(' ');
            }
            name.push_str(title_vec.get(name_part).unwrap());
        }
    }
    else {
        for name_part in 0..(vec_len - 1) {
            if name_part != 0 {
                name.push(' ');
            }
            name.push_str(title_vec.get(name_part).unwrap());
        }
        cmc.push_str(title_vec.get(vec_len - 1).unwrap());
    }
    let card = Card {
        name,
        cmc,
        type_line
    };
    Ok(card)
}
