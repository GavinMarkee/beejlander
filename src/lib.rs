/*
Library crate for the beejlander package
*/
use std::env;

pub struct Config {
    pub include_sb: bool,
    pub cmn_price: f32,
    pub unc_price: f32,
    pub rare_price: f32,
    pub land_price: f32
}
impl Config {
    pub fn new() -> Config {
        let include_sb = fetch_bool("SILVER");
        let cmn_price = fetch_float("COMMON", 0.10);
        let unc_price = fetch_float("UNCOMMON", 0.10);
        let rare_price = fetch_float("RARE", 0.25);
        let land_price = fetch_float("LAND", 0.20);
        Config {
            include_sb,
            cmn_price,
            unc_price,
            rare_price,
            land_price
        }
    }
}

fn fetch_bool(var: &str) -> bool {
    match env::var(var) {
        Ok(r) => match r.parse::<bool>() {
            Ok(b) => b,
            Err(_e) => true
        },
        Err(_e) => true
    }
}

fn fetch_float(var: &str, default: f32) -> f32 {
    match env::var(var) {
        Ok(r) => match r.parse::<f32>() {
            Ok(f) => f,
            Err(_e) => default
        },
        Err(_e) => default
    }
}

pub struct Card {
    name: String,
    cmc: String,
    type_line : String,
    count: u8
}

pub mod query_gen {
    pub struct Query{
        pub rare: String,
        pub other: String
    }

    pub fn generate_queries(config: &super::Config) -> Query {
        Query {
            rare: rare_query(config.include_sb, config.rare_price),
            other: common_query(config.include_sb, config.cmn_price, config.unc_price, config.land_price)
        }
    }

    fn pes(symbol: &str) -> &str {
        // Percent encodes specific symbols
        match symbol {
            "(" => "%28",
            ")" => "%29",
            "+" => "%2B",
            ":" => "%3A",
            "=" => "%3D",
            _ => "",
        }
    }

    fn start_query(include_sb: bool) -> String {
        if include_sb {
            format!("q=-t{c}conspiracy+-t{c}contraption+{s}", c = pes(":"), s = sb(include_sb))
        }
        else {
            format!("q=-t{c}conspiracy+-t{c}contraption", c = pes(":"))
        }
    }

    fn end_query() -> String {
        String::from("&format=text")
    }

    fn sb(include: bool) -> String {
        // Include silver bordered cards
        if include {
            String::from("")
        }
        else {
            format!("-border{}silver", pes(":"))
        }
    }

    fn cmn(price: f32) -> String {
        format!("-t{c}land+r{c}c+usd<{e}{p}", c = pes(":"), e = pes("="), p = price)
    }

    fn unc(price: f32) -> String {
        format!("-t{c}land+r{c}u+usd<{e}{p}", c = pes(":"), e = pes("="), p = price)
    }

    fn land(price: f32) -> String {
        format!("t{}land+r<r+usd<{}{}", pes(":"), pes("="), price)
    }

    fn rare(price: f32) -> String {
        format!("r>u+usd<{}{}", pes("="), price)
    }

    fn common_query(include_sb: bool, cmn_price: f32, unc_price: f32, land_price: f32) -> String {
        format!(
            "{b}+{l}{l}{c}{r}+or+{l}{u}{r}+or+{l}{d}{r}{r}+{e}",
            b = start_query(include_sb),
            l = pes("("),
            c = cmn(cmn_price),
            r = pes(")"),
            u = unc(unc_price),
            d = land(land_price),
            e = end_query()
        )
    }

    fn rare_query(include_sb: bool, price: f32) -> String {
        format!("{b}+{r}+{e}", b = start_query(include_sb), r = rare(price), e =  end_query())
    }
}

pub mod async_methods {
    use regex::Regex;
    use std::collections::HashMap;
    use std::error::Error;
    use super::Card;
    use super::query_gen::Query;

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
}

pub mod file_io {
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
}
