use reqwest;
use tracing::{debug, error};

use crate::{Error, Locale, categories, featured, search, trending};

/// Immutable type holding the api key along with region and language codes.
#[derive(Debug, Clone)]
pub struct Tenor {
    api_key: String,
    locale: Locale,
}

impl Tenor {
    /// Creates a new instance.
    #[must_use]
    pub fn new(api_key: String, locale: Locale) -> Self {
        Self { api_key, locale }
    }

    /// Returns a vector of tagged categories that are featured.
    #[must_use]
    pub async fn categories_featured(&self) -> Result<categories::Response, Error> {
        let url = format!(
            "https://tenor.googleapis.com/v2/categories?key={}{}{}",
            self.api_key,
            self.locale.to_query_parameter(),
            "&type=featured",
        );

        self.categories_request(url).await
    }

    /// Returns a vector of tagged categories that are trending.
    #[must_use]
    pub async fn categories_trending(&self) -> Result<categories::Response, Error> {
        let url = format!(
            "https://tenor.googleapis.com/v2/categories?key={}{}{}",
            self.api_key,
            self.locale.to_query_parameter(),
            "&type=trending",
        );

        self.categories_request(url).await
    }

    /// Returns a vector of tagged categories that are featured with extra input parameters.
    #[must_use]
    pub async fn categories_featured_with_parameters(
        &self,
        parms: categories::Parameters,
    ) -> Result<categories::Response, Error> {
        let mut url = format!(
            "https://tenor.googleapis.com/v2/categories?key={}{}{}",
            self.api_key,
            self.locale.to_query_parameter(),
            "&type=featured",
        );

        url.push_str(&format!("&client_key={}", &parms.client_key));
        url.push_str(&parms.content_filter.to_query_parameter());

        self.categories_request(url).await
    }

    /// Returns a vector of tagged categories that are trending with extra input parameters.
    #[must_use]
    pub async fn categories_trending_with_parameters(
        &self,
        parms: categories::Parameters,
    ) -> Result<categories::Response, Error> {
        let mut url = format!(
            "https://tenor.googleapis.com/v2/categories?key={}{}{}",
            self.api_key,
            self.locale.to_query_parameter(),
            "&type=trending",
        );

        url.push_str(&format!("&client_key={}", &parms.client_key));
        url.push_str(&parms.content_filter.to_query_parameter());

        self.categories_request(url).await
    }

    async fn categories_request(&self, url: String) -> Result<categories::Response, Error> {
        let response = reqwest::get(url).await;

        match response.is_ok() {
            true => debug!("fetch categories successful."),
            false => error!("fetch categories failed."),
        }

        let response_body = response?.text().await?;
        let obj: categories::Response = serde_json::from_str(&response_body)?;
        Ok(obj)
    }

    /// Returns the featured stickers of the hour, have recalled every hour and cache contents.
    #[must_use]
    pub async fn featured(&self) -> Result<search::Response, Error> {
        let url = format!(
            "https://tenor.googleapis.com/v2/featured?key={}{}",
            self.api_key,
            self.locale.to_query_parameter(),
        );
        self.featured_request(url).await
    }

    // Continue from where feature left off, use the `next` field as the input for `position`.
    #[must_use]
    pub async fn featured_with_position(
        &self,
        position: String,
    ) -> Result<search::Response, Error> {
        let url = format!(
            "https://tenor.googleapis.com/v2/featured?key={}{}&pos={}",
            self.api_key,
            self.locale.to_query_parameter(),
            position,
        );
        self.featured_request(url).await
    }

    /// Return the featured stickers with extra parameters of the hour, have recalled every hour and cache contents.
    #[must_use]
    pub async fn featured_with_parameters(
        &self,
        parms: featured::Parameters,
    ) -> Result<search::Response, Error> {
        let mut url = format!(
            "https://tenor.googleapis.com/v2/featured?key={}{}",
            self.api_key,
            self.locale.to_query_parameter(),
        );

        if let Some(filter) = parms.search_filter {
            url.push_str(&filter.to_query_parameter());
        }

        if let Some(filters) = parms.media_filter {
            let mut base = String::from("&media_filter=");

            for media_type in filters {
                base.push_str(&media_type.to_string());
                base.push(',');
            }
            url.push_str(&base);
        }

        url.push_str(&parms.ar_range.to_query_parameter());
        url.push_str(&parms.content_filter.to_query_parameter());
        url.push_str(&format!("&limit={}", parms.limit));
        if let Some(pos) = parms.position {
            url.push_str(&format!("&pos={}", pos));
        }

        self.featured_request(url).await
    }

    async fn featured_request(&self, url: String) -> Result<search::Response, Error> {
        let response = reqwest::get(url).await;

        match response.is_ok() {
            true => debug!("fetch featured successful."),
            false => error!("fetch featured failed."),
        }

        let response_body = response?.text().await?;
        let obj: search::Response = serde_json::from_str(&response_body)?;
        Ok(obj)
    }

    /// Searchs Tenor with the given query.
    #[must_use]
    pub async fn search(&self, query: String) -> Result<search::Response, Error> {
        let url = format!(
            "https://tenor.googleapis.com/v2/search?q={}&key={}{}",
            query,
            self.api_key,
            self.locale.to_query_parameter(),
        );

        self.search_request(url).await
    }

    /// Continue searching Tenor with the given query, starting from where the last request ended.
    #[must_use]
    pub async fn search_with_position(
        &self,
        query: String,
        position: String,
    ) -> Result<search::Response, Error> {
        let url = format!(
            "https://tenor.googleapis.com/v2/search?q={}&key={}{}&pos={}",
            query,
            self.api_key,
            self.locale.to_query_parameter(),
            position,
        );

        self.search_request(url).await
    }

    /// Searchs Tenor with the given query with extra parameters.
    #[must_use]
    pub async fn search_with_parameters(
        &self,
        query: String,
        parms: search::Parameters,
    ) -> Result<search::Response, Error> {
        let mut url = format!(
            "https://tenor.googleapis.com/v2/search?q={}&key={}{}&client_key={}",
            query,
            self.api_key,
            self.locale.to_query_parameter(),
            parms.client_key,
        );

        if let Some(filter) = parms.search_filter {
            url.push_str(&filter.to_query_parameter());
        }

        url.push_str(&parms.content_filter.to_query_parameter());

        if let Some(filters) = parms.media_filter {
            let mut base = String::from("&media_filter=");

            for media_type in filters {
                base.push_str(&media_type.to_string());
                base.push(',');
            }
            url.push_str(&base);
        }

        url.push_str(&parms.ar_range.to_query_parameter());
        url.push_str(&format!("&random={}", parms.random));
        url.push_str(&format!("&limit={}", parms.limit));
        if let Some(pos) = parms.position {
            url.push_str(&format!("&pos={}", pos));
        }

        self.search_request(url).await
    }

    async fn search_request(&self, url: String) -> Result<search::Response, Error> {
        let response = reqwest::get(&url).await;

        match response.is_ok() {
            true => debug!("search query successful.",),
            false => error!("search query failed {:#?}.", &response),
        }

        let response_body = response?.text().await?;
        let obj: search::Response = serde_json::from_str(&response_body)?;

        Ok(obj)
    }

    /// Returns the hourly tending search terms.
    #[must_use]
    pub async fn trending_terms(&self) -> Result<trending::Response, Error> {
        let url = format!(
            "https://tenor.googleapis.com/v2/trending_terms?key={}{}",
            self.api_key,
            self.locale.to_query_parameter(),
        );

        self.trending_request(url).await
    }

    /// Returns the hourly tending search terms with extra parameters.
    #[must_use]
    pub async fn trending_terms_with_parameters(
        &self,
        parms: trending::Parameters,
    ) -> Result<trending::Response, Error> {
        let url = format!(
            "https://tenor.googleapis.com/v2/trending_terms?key={}{}&client_key={}&limit={}",
            self.api_key,
            self.locale.to_query_parameter(),
            parms.client_key,
            parms.limit,
        );

        self.trending_request(url).await
    }

    async fn trending_request(&self, url: String) -> Result<trending::Response, Error> {
        let response = reqwest::get(url).await;

        match response.is_ok() {
            true => debug!("fetch trending successful."),
            false => error!("fetch trending failed."),
        }

        let response_body = response?.text().await?;
        let obj: trending::Response = serde_json::from_str(&response_body)?;
        Ok(obj)
    }
}
