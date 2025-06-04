use reqwest;
use tracing::{debug, error};

use crate::{Error, Locale, categories, featured, search, trending};

/// Immutable type holding the api key along with region and language codes.
pub struct Tenor {
    api_key: String,
    locale: Locale,
}

impl Tenor {
    /// Creates a new instance.
    pub fn new(api_key: String, locale: Locale) -> Self {
        Self { api_key, locale }
    }

    /// Returns a vector of tagged categories.
    pub async fn categories(&self) -> Result<categories::Response, Error> {
        let url = format!(
            "https://tenor.googleapis.com/v2/categories?key={}{}",
            self.api_key,
            self.locale.to_query_parameter(),
        );

        self.categories_request(url).await
    }

    /// Returns a vector of tagged categories with extra input parameters.
    pub async fn categories_with_parameters(
        &self,
        parms: categories::Parameters,
    ) -> Result<categories::Response, Error> {
        let mut url = format!(
            "https://tenor.googleapis.com/v2/categories?key={}{}",
            self.api_key,
            self.locale.to_query_parameter(),
        );

        url.push_str(&format!("&client_key={}", &parms.client_key));
        url.push_str(&parms.r#type.to_query_parameter());
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

    /// Returns the featured gifs of the hour, have recalled every hour and cache contents.
    pub async fn featured(&self) -> Result<search::Response, Error> {
        let url = format!(
            "https://tenor.googleapis.com/v2/featured?key={}{}",
            self.api_key,
            self.locale.to_query_parameter(),
        );
        self.featured_request(url).await
    }

    // Continue from where feature left off, use the `next` field as the input for `position`.
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

    /// Return the featured gifs with extra parameters of the hour, have recalled every hour and cache contents.
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

    /// Returns the hourly tending content.
    pub async fn trending(&self) -> Result<trending::Response, Error> {
        let url = format!(
            "https://tenor.googleapis.com/v2/trending_terms?key={}{}",
            self.api_key,
            self.locale.to_query_parameter(),
        );

        println!("url {}", &url);

        self.trending_request(url).await
    }

    /// Returns the hourly tending content with extra parameters.
    pub async fn trending_with_parameters(
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
