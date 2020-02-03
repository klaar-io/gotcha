#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate strsim;
extern crate dialoguer;

use dialoguer::{theme::CustomPromptCharacterTheme, Input};
use strsim::jaro_winkler;

mod search;

#[derive(Debug, PartialEq)]
struct Entity {
    title: String,
    year: i16,
    kind: String,
}

#[derive(Deserialize, Debug, PartialEq, PartialOrd)]
pub struct Movie {
    title: String,
    us_gross: u32,
    worldwide_gross: u32,
    us_dvd_sales: u32,
    production_budget: u32,
    release_date: String,
    mpaa_rating: String,
    running_time_min: u16,
    distributor: String,
    source: String,
    major_genre: String,
    creative_type: String,
    director: String,
    rotten_tomatoes_rating: u8,
    imdb_rating: f64,
    imdb_votes: u32,
}

fn main() {
    hello();
}

fn hello() {
    let theme = CustomPromptCharacterTheme::new(':');
    let input: String = Input::with_theme(&theme)
        .with_prompt("Search for")
        .interact()
        .unwrap();
    let results: Vec<Movie> = search::by_title(&input);
    let inp = input.as_str();

    for (pos, i) in results.iter().enumerate() {
        println!("{}. {} ({}) (score: {})", pos, i.title, i.release_date, jaro_winkler(&i.title, inp));
    }
    let sel: usize = Input::with_theme(&theme)
        .with_prompt("choose movie (enter number)")
        .interact()
        .unwrap();
    println!("You have selected: {:#?}", results[sel]);
}

#[cfg(test)] // Only compiles when running tests
mod tests {
    use super::search; // Import root greet function
    use super::Entity;

    #[test]
    fn test_search() {
        let results = vec![
            Entity { title: "Red".to_string(), year: 2006, kind: "".to_string() },
            Entity { title: "Red Baron".to_string(), year: 1997, kind: "".to_string() }];
        assert_eq!(results, search("Red".to_string(), 2006));
    }
}
