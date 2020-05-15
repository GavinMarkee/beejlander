/*
Library crate for the beejlander package
*/
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
