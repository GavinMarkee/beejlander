/*
Query module for the beejlander package
*/

// Public Structs

pub struct Query{
    pub rare: String,
    pub other: String
}

// Public Functions

pub fn generate_queries(config: &super::Config) -> Query {
    Query {
        rare: rare_query(config.include_sb, config.rare_price),
        other: common_query(config.include_sb, config.cmn_price, config.unc_price, config.land_price)
    }
}

// Private Functions

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