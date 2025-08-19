use std::env;

use dotenv::dotenv;
use tokio;

use crate::*;

#[tokio::test]
async fn categories() {
    let _ = dotenv();
    let tenor = tenor::Tenor::new(
        env::var("API_KEY").expect("Failed to find env file"),
        Locale::default(),
    );
    let response = tenor.categories_featured().await;

    dbg!("{}", &response);
    assert!(response.is_ok())
}

#[tokio::test]
async fn featured() {
    let _ = dotenv();
    let tenor = tenor::Tenor::new(
        env::var("API_KEY").expect("Failed to find env file"),
        Locale::default(),
    );
    let response = tenor.featured().await;

    dbg!("{}", &response);
    assert!(response.is_ok())
}

#[tokio::test]
async fn search() {
    let _ = dotenv();
    let tenor = tenor::Tenor::new(
        env::var("API_KEY").expect("Failed to find env file"),
        Locale::default(),
    );
    let response = tenor.search("excited".to_string()).await;

    dbg!("{}", &response);
    assert!(response.is_ok())
}

#[tokio::test]
async fn search_parameters() {
    let _ = dotenv();
    let tenor = tenor::Tenor::new(
        env::var("API_KEY").expect("Failed to find env file"),
        Locale::new(Language::Japanese, CountryCode::JP),
    );

    let parms = search::Parameters {
        content_filter: ContentFilter::Medium,
        media_filter: Some(&[MediaFilter::Gif, MediaFilter::Mp4]),
        ar_range: ArRange::Standard,
        random: true,
        limit: 2,
        ..Default::default()
    };
    let response = tenor
        .search_with_parameters("excited".to_string(), parms)
        .await;

    dbg!("{}", &response);
    assert!(response.is_ok())
}

#[tokio::test]
async fn trending() {
    let _ = dotenv();

    let tenor = tenor::Tenor::new(
        env::var("API_KEY").expect("Failed to find env file"),
        Locale::default(),
    );
    let response = tenor.trending_terms().await;

    dbg!("{}", &response);
    assert!(response.is_ok())
}
