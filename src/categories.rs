use serde::{Deserialize, Serialize};

use crate::ContentFilter;

#[derive(Debug, Default)]
pub struct Parameters {
    pub client_key: String,
    pub r#type: Type,
    pub content_filter: ContentFilter,
}

/// <https://developers.google.com/tenor/guides/endpoints#supported-types-categories>
#[derive(Debug, Default)]
pub enum Type {
    #[default]
    Featured,
    Trending,
}

impl Type {
    pub fn to_query_parameter(&self) -> String {
        format!(
            "&type={}",
            match self {
                Type::Featured => "featured",
                Type::Trending => "trending",
            }
        )
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Response {
    pub locale: String,
    pub tags: Vec<Tag>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tag {
    pub searchterm: String,
    pub path: String,
    pub image: String,
    pub name: String,
}
