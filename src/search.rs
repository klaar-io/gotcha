use std::fs::File;
use std::io::BufReader;
use std::error::Error;
use std::path::Path;
extern crate strsim;

use strsim::jaro_winkler;

use super::Movie;

pub fn by_title(term: &str) -> Vec<Movie> {
    println!("Searching for: {}", term);
    let mut results = read_movies_from_file("./src/data/movies.json").unwrap();
    results.retain(|e| jaro_winkler(&e.title, term) > 0.84);
    results.sort_by(|a, b| jaro_winkler(&b.title, term).partial_cmp(&jaro_winkler(&a.title, term)).unwrap());
    results
}

fn read_movies_from_file<P: AsRef<Path>>(path: P) -> Result<Vec<Movie>, Box<dyn Error>> {
    // Open the file in read-only mode with buffer.
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    // Read the JSON contents of the file as an instance of `User`.
    let u = serde_json::from_reader(reader)?;

    // Return the results as Vec<`Movie`>.
    Ok(u)
}

#[cfg(test)] // Only compiles when running tests
mod tests {
    use super::Movie;

    #[test]
    fn test_search() {
        let results = vec![
            Movie { title: "Red".to_string(), year: 2006, kind: "".to_string() },
            Movie { title: "Red Baron".to_string(), year: 1997, kind: "".to_string() }];
        assert_eq!(results, by_title("Red".to_string(), 2006));
    }
}
