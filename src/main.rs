/*
Binary crate responsible for running the beejlander package
*/

use async_std::task;

use beejlander::{
    gui,
    query,
    request,
    io
};
use beejlander::Config;

fn main() {
    /*
    let config = Config::new();
    println!("Running with the following configuration:");
    println!("\tInclude silver-bordered cards: {}", config.include_sb);
    println!("\tCommon price limit: {}", config.cmn_price);
    println!("\tUncommon price limit: {}", config.unc_price);
    println!("\tRare price limit: {}", config.rare_price);
    println!("\tLand price limit: {} \n", config.land_price);
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
    */
    gui::run()
}
