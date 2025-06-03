use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{ArRange, ContentFilter, DEFAULT_LIMIT, Limit, MediaFilter, SearchFilter};

#[derive(Debug)]
pub struct Parameters {
    pub client_key: String,
    pub search_filter: Option<SearchFilter>,
    pub content_filter: ContentFilter,
    pub media_filter: Option<&'static [MediaFilter]>,
    pub ar_range: ArRange,
    pub random: bool,
    pub limit: Limit,
}

impl Default for Parameters {
    fn default() -> Self {
        Self {
            client_key: "my_test_app".to_string(),
            search_filter: None,
            content_filter: ContentFilter::default(),
            media_filter: Some(MediaFilter::DEFAULT),
            ar_range: ArRange::default(),
            random: false,
            limit: DEFAULT_LIMIT,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Response {
    pub results: Vec<ResponseObject>,
    pub next: String,
}

/// Note: Even though `hascaption` and `bg_color` are listed in the table they do not exist in the
/// response so they are omited.
/// <https://developers.google.com/tenor/guides/response-objects-and-errors#response-object>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResponseObject {
    pub created: f64,
    pub hasaudio: bool,
    pub id: String,
    pub media_formats: ContentFormats,
    pub tags: Vec<String>,
    pub title: String,
    pub content_description: String,
    pub itemurl: String,
    pub flags: Vec<Value>,
    pub url: String,
}

/// Tenor's API offers the following five base formats in a variety of sizes:
///    GIF
///    MP4
///    WebM
///    Transparent WebP
///    Transparent GIF
///
/// The MP4 and WebM formats play their clip only once, with the exception of the loopedmp4, which plays the clip a few times. The GIF format plays its clip on a continuous loop. The transparent formats are for sticker content and aren't available in GIF search results.
///
/// <https://developers.google.com/tenor/guides/response-objects-and-errors#content-formats>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContentFormats {
    pub gifpreview: Option<Gifpreview>,
    pub gif: Option<Gif>,
    pub mediumgif: Option<Mediumgif>,
    pub tinygif: Option<Tinygif>,
    pub nanogif: Option<Nanogif>,
    pub mp4: Option<Mp4>,
    pub loopedmp4: Option<Loopedmp4>,
    pub tinymp4: Option<Tinymp4>,
    pub nanomp4: Option<Nanomp4>,
    pub webm: Option<Webm>,
    pub tinywebm: Option<Tinywebm>,
    pub nanowebm: Option<Nanowebm>,
    pub webp: Option<Webp>,
    pub tinygifpreview: Option<Tinygifpreview>,
    pub nanogifpreview: Option<Nanogifpreview>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Gifpreview {
    pub url: String,
    pub duration: f64,
    pub preview: String,
    #[serde(rename = "dims")]
    pub dimensions: Vec<i64>,
    pub size: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Gif {
    pub url: String,
    pub duration: f64,
    pub preview: String,
    #[serde(rename = "dims")]
    pub dimensions: Vec<i64>,
    pub size: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Mediumgif {
    pub url: String,
    pub duration: f64,
    pub preview: String,
    #[serde(rename = "dims")]
    pub dimensions: Vec<i64>,
    pub size: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tinygif {
    pub url: String,
    pub duration: f64,
    pub preview: String,
    #[serde(rename = "dims")]
    pub dimensions: Vec<i64>,
    pub size: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Nanogif {
    pub url: String,
    pub duration: f64,
    pub preview: String,
    #[serde(rename = "dims")]
    pub dimensions: Vec<i64>,
    pub size: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Mp4 {
    pub url: String,
    pub duration: f64,
    pub preview: String,
    #[serde(rename = "dims")]
    pub dimensions: Vec<i64>,
    pub size: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Loopedmp4 {
    pub url: String,
    pub duration: f64,
    pub preview: String,
    #[serde(rename = "dims")]
    pub dimensions: Vec<i64>,
    pub size: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tinymp4 {
    pub url: String,
    pub duration: f64,
    pub preview: String,
    #[serde(rename = "dims")]
    pub dimensions: Vec<i64>,
    pub size: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Nanomp4 {
    pub url: String,
    pub duration: f64,
    pub preview: String,
    #[serde(rename = "dims")]
    pub dimensions: Vec<i64>,
    pub size: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Webm {
    pub url: String,
    pub duration: f64,
    pub preview: String,
    #[serde(rename = "dims")]
    pub dimensions: Vec<i64>,
    pub size: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tinywebm {
    pub url: String,
    pub duration: f64,
    pub preview: String,
    #[serde(rename = "dims")]
    pub dimensions: Vec<i64>,
    pub size: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Nanowebm {
    pub url: String,
    pub duration: f64,
    pub preview: String,
    #[serde(rename = "dims")]
    pub dimensions: Vec<i64>,
    pub size: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Webp {
    pub url: String,
    pub duration: f64,
    pub preview: String,
    #[serde(rename = "dims")]
    pub dimensions: Vec<i64>,
    pub size: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tinygifpreview {
    pub url: String,
    pub duration: f64,
    pub preview: String,
    #[serde(rename = "dims")]
    pub dimensions: Vec<i64>,
    pub size: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Nanogifpreview {
    pub url: String,
    pub duration: f64,
    pub preview: String,
    #[serde(rename = "dims")]
    pub dimensions: Vec<i64>,
    pub size: i64,
}
