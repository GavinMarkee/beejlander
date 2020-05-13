/*
Library crate for the beejlander package
*/

pub struct Config {
    include_sb: bool,
    cmn_price: f32,
    unc_price: f32,
    rare_price: f32,
    land_price: f32
}

pub struct Card {
    name: String,
    cmc: String,
    type_line : String
}

pub struct CardList {
    count: u8,
    card: Card
}

mod query_helper {
    pub struct Query{
        rare: String,
        other: String
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

    fn start_query(include_sb: bool) -> &str {
        if include_sb:
            format!("q=-t{c}conspiracy+-t{c}contraption+{s}", c = pes(":"), s = sb(include_sb))
        else:
            format!("q=-t{c}conspiracy+-t{c}contraption", c = pes(":"))
    }

    fn end_query() -> &str {
        "&format=text"
    }

    fn sb(include: bool) -> &str {
        // Include silver bordered cards
        if include {
            ""
        }
        else {
            format!("-border{}silver", pes(":"))
        }
    }

    fn cmn(price: f32) -> &str {
        format!("-t{c}land+r{c}c+usd<{e}{p}", c = pes(":"), e = pes("="), p = price)
    }

    fn unc(price: f32) -> &str {
        format!("-t{c}land+r{c}u+usd<{e}{p}", c = pes(":"), e = pes("="), p = price)
    }

    fn land(price: f32) -> &str {
        format!("t{}land+r<r+usd<{}{}", pes(":"), pes("="), price)
    }

    fn rare(price: f32) -> &str {
        format!("r>u+usd<{}{}", pes("="), price)
    }

    fn common_query(include_sb: bool, cmn_price: f32, unc_price: f32, land_price: f32) -> &str {
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

    fn rare_query(include_sb: bool, price: f32) -> &str {
        format!("{b}+{r}+{e}", b = start_query(include_sb), r = rare(price), e =  end_query())
    }

    pub fn generate_queries(config: &Config) -> &Query {
        &Query {
            rare: rare_query(config.include_sb, config.rare_price),
            other: common_query(config.include_sb, config.cmn_price, config.unc_price, config.land_price)
        }
    }
}

mod async_methods {

}
