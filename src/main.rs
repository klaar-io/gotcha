#[derive(Debug,PartialEq)]
struct Entity {
    title: String,
    year: i16,
    kind: String,
}

fn main() {
    println!("{}", hello());
    let results = search("Red Baron".to_string(), 2006);

    for i in &results {
        println!("{} ({}) {}", i.title, i.year, i.kind);
    }
}

fn hello() -> String {
    "This is gotcha!".to_string()
}

fn search(_term: String, _year: i16) -> Vec<Entity> {
    let mut results: Vec<Entity> = vec![
        Entity { title: "Red".to_string(), year: 2006, kind: "Movie".to_string() },
        Entity { title: "Red Baron".to_string(), year: 1997, kind: "TV Show".to_string() }
    ];
    results.retain(|e| e.title == _term);
    results
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
