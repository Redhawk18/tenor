pub mod categories;
pub mod featured;
pub mod search;
pub mod tenor;
pub mod trending;

use std::fmt;

use codes_iso_639::part_1::LanguageCode;
use codes_iso_3166::part_1::CountryCode;
pub use tenor::Tenor;

#[cfg(test)]
mod tests;

use thiserror::Error;

pub type Limit = u8;

/// Fetch up to the specified number of results. Use this as the default if you are unsure.
pub const DEFAULT_LIMIT: Limit = 20;

/// Fetch up to the specified number of results, and the maximum value is 50 inclusive.
pub const MAX_LIMIT: Limit = 50;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Request(#[from] reqwest::Error),
    #[error(transparent)]
    Serialization(#[from] serde_json::Error),
}

/// Filter the Response Objects to only include GIFs with aspect ratios that fit within the selected range.
/// The default value is all. The accepted values are all, wide, and standard:
///
/// all: No constraints
/// wide: 0.42 <= aspect ratio <= 2.36
/// standard: 0.56 <= aspect ratio <= 1.78
///
#[derive(Debug, Default)]
pub enum ArRange {
    #[default]
    All,
    Wide,
    Standard,
}

impl ArRange {
    pub fn to_query_parameter(&self) -> String {
        format!(
            "&ar_range={}",
            match self {
                ArRange::All => "all",
                ArRange::Wide => "wide",
                ArRange::Standard => "standard",
            }
        )
    }
}

/// Specify the content safety filter level.
/// The default value is off. The accepted values are off, low, medium, and high.
#[derive(Debug, Default)]
pub enum ContentFilter {
    #[default]
    Off,
    Low,
    Medium,
    High,
}

impl ContentFilter {
    pub fn to_query_parameter(&self) -> String {
        format!(
            "&contentfilter={}",
            match self {
                ContentFilter::Off => "off",
                ContentFilter::Low => "low",
                ContentFilter::Medium => "medium",
                ContentFilter::High => "high",
            }
        )
    }
}

/// Specify the default language to interpret the search string. xx is the language's ISO 639-1 language code, while the optional YY value is the two-letter ISO 3166-1 country code.
/// You can use the country code that you provide in locale to differentiate between dialects of the given language.
/// The default value is en_US.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Locale {
    language: LanguageCode,
    country: CountryCode,
}

impl Locale {
    pub fn new(language: LanguageCode, country: CountryCode) -> Self {
        Self { language, country }
    }

    pub fn to_query_parameter(&self) -> String {
        format!("&locate={}", self.to_string())
    }
}

impl Default for Locale {
    fn default() -> Self {
        Self {
            language: LanguageCode::En,
            country: CountryCode::US,
        }
    }
}

impl fmt::Display for Locale {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}_{}",
            self.language.to_string().to_lowercase(),
            self.country.to_string().to_uppercase(),
        )
    }
}

/// Comma-separated list of GIF formats to filter the Response Objects. By default, media_filter returns all formats for each Response Object.
/// Example: media_filter=gif,tinygif,mp4,tinymp4
/// Doesn't have a default value.
#[derive(Debug)]
pub enum MediaFilter {
    Preview,
    Gif,
    MediumGif,
    TinyGif,
    NanoGif,
    Mp4,
    LoopedMp4,
    TinyMp4,
    NanoMp4,
    Webm,
    TinyWebm,
    NanoWebm,
    WebpTransparent,
    TinyWebpTransparent,
    NanoWebpTransparent,
    GifTransparent,
    TinyGifTransparent,
    NanoGifTransparent,
}

impl MediaFilter {
    pub const ALL: &'static [MediaFilter] = &[
        MediaFilter::Preview,
        MediaFilter::Gif,
        MediaFilter::MediumGif,
        MediaFilter::TinyGif,
        MediaFilter::NanoGif,
        MediaFilter::Mp4,
        MediaFilter::LoopedMp4,
        MediaFilter::TinyMp4,
        MediaFilter::NanoMp4,
        MediaFilter::Webm,
        MediaFilter::TinyWebm,
        MediaFilter::NanoWebm,
        MediaFilter::WebpTransparent,
        MediaFilter::TinyWebpTransparent,
        MediaFilter::NanoWebpTransparent,
        MediaFilter::GifTransparent,
        MediaFilter::TinyGifTransparent,
        MediaFilter::NanoGifTransparent,
    ];

    pub const DEFAULT: &'static [MediaFilter] = Self::ALL;
}

// This is just so we get `to_string()` I don't care about `Display`.
impl fmt::Display for MediaFilter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MediaFilter::Preview => write!(f, "preview"),
            MediaFilter::Gif => write!(f, "gif"),
            MediaFilter::MediumGif => write!(f, "mediumgif"),
            MediaFilter::TinyGif => write!(f, "tinygif"),
            MediaFilter::NanoGif => write!(f, "nanogif"),
            MediaFilter::Mp4 => write!(f, "mp4"),
            MediaFilter::LoopedMp4 => write!(f, "loopedmp4"),
            MediaFilter::TinyMp4 => write!(f, "tinymp4"),
            MediaFilter::NanoMp4 => write!(f, "nanomp4"),
            MediaFilter::Webm => write!(f, "webm"),
            MediaFilter::TinyWebm => write!(f, "tinywebm"),
            MediaFilter::NanoWebm => write!(f, "nanowebm"),
            MediaFilter::WebpTransparent => write!(f, "webp_transparent"),
            MediaFilter::TinyWebpTransparent => write!(f, "tinywebp_transparent"),
            MediaFilter::NanoWebpTransparent => write!(f, "nanowebp_transparent"),
            MediaFilter::GifTransparent => write!(f, "gif_transparent"),
            MediaFilter::TinyGifTransparent => write!(f, "tinygif_transparent"),
            MediaFilter::NanoGifTransparent => write!(f, "nanogif_transparent"),
        }
    }
}

/// Comma-separated list of non-GIF content types to filter the Response Objects. By default, searchfilter returns GIF content only.
/// Doesn't have a default value. The accepted values are sticker, static, and -static:
///
/// searchfilter=sticker returns both static and animated sticker content.
/// searchfilter=sticker,-static returns only animated sticker content.
/// searchfilter=sticker,static returns only static sticker content.
///
/// For GIF content, either leave searchfilter blank or don't use it.
///
#[derive(Debug, PartialEq)]
pub enum SearchFilter {
    Sticker,
    Static,
    NonStatic,
}

impl SearchFilter {
    pub fn to_query_parameter(&self) -> String {
        format!(
            "&searchfilter={}",
            match self {
                SearchFilter::Sticker => "sticker",
                SearchFilter::Static => "sticker,static",
                SearchFilter::NonStatic => "sticker,-static",
            }
        )
    }
}
