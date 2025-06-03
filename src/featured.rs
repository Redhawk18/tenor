use crate::{ArRange, ContentFilter, Limit, MediaFilter, SearchFilter};

#[derive(Debug)]
pub struct Parameters {
    pub client_key: String,
    pub search_filter: Option<SearchFilter>,
    pub media_filter: Option<&'static [MediaFilter]>,
    pub ar_range: ArRange,
    pub content_filter: ContentFilter,
    pub limit: Limit,
    // pub position
}
