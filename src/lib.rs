/*
Library crate for the beejlander package
*/
use async_std::task;
use std::env;

pub mod gui;
pub mod query;
pub mod request;
pub mod io;

pub struct Config {
    pub include_sb: bool,
    pub cmn_price: f32,
    pub unc_price: f32,
    pub rare_price: f32,
    pub land_price: f32
}
impl Config {
    pub fn new(preset: Option<Config>) -> Config {
        match preset {
            Some(c) => c,
            None => {
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
    }

    pub fn parse(include_sb: bool, prices: Vec<&str>) -> ConfigParse {
        let mut config_parse = ConfigParse {
            config: None,
            errors: Vec::new()
        };
        let mut succeed = true;
        let mut prices_f: Vec<f32> = Vec::new();
        for price in prices {
            match price.parse::<f32>() {
                Ok(f) => {
                    prices_f.push(f);
                    config_parse.errors.push(false);
                }
                Err(_e) => {
                    config_parse.errors.push(true);
                    succeed = false;
                }
            }
        }
        if succeed {
            config_parse.config = Some(Config{
                include_sb,
                cmn_price: prices_f[0],
                unc_price: prices_f[1],
                rare_price: prices_f[2],
                land_price: prices_f[3]
            })
        }
        config_parse
    }
}

pub struct ConfigParse {
    pub config: Option<Config>,
    pub errors: Vec<bool>
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

pub fn run_fetch(config: Config) {
    println!("Generating queries");
    let queries = query::generate_queries(&config);
    println!("Fetching cards");
    let list = task::block_on(request::fetch_cards(queries)).unwrap();
    println!("Saving cards to 'cards.txt'");
    let result = io::save_to_file(&list);
    match result {
        Ok(s) => println!("{}", s),
        Err(e) => println!("{}", e)
    };
}
