# Tenor

A Rust crate for tenor.com's v2 API 

## Usage
```rs
use tenor::{search::Response Error, Tenor};

async fn search_tenor() {
    let tenor = Tenor::new(env::var("API_KEY").expect("Failed to find api key"));
    let response: Resullt<Response, Error> = tenor.search("rustlang".to_string()).await;

    dbg!("{}", &response);
    assert!(response.is_ok())
}
```

## Roadmap
endpoints
* [ ] Search Suggestions
* [ ] Autocomplete
* [ ] Register Share
* [ ] Posts

## Contributions
Welcomed.
