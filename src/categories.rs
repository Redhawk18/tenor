use iso_639::part1::Language;
use serde::{Deserialize, Serialize};

use crate::ContentFilter;

#[derive(Debug, Default)]
pub struct Parameters {
    pub client_key: String,
    pub content_filter: ContentFilter,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Response {
    /// Language of the returned content.
    pub locale: Language,
    /// All the [`Tag`]s found.
    pub tags: Vec<Tag>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tag {
    /// The term, example: "excited".
    pub searchterm: String,
    /// Image api path.
    pub path: String,
    /// Full url of the gif.
    pub image: String,
    /// The name, example: "#excited".
    pub name: String,
}
