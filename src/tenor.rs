use codes_iso_3166::part_1::CountryCode;
use reqwest;
use secrecy::{ExposeSecret, SecretString};
use tracing::{debug, error};

use crate::{Error, Locale, categories, featured, search, trending};

/// Immutable type holding the api key along with region and language codes.
pub struct Tenor {
    api_key: SecretString,
    country: CountryCode,
    locate: Locale,
}

impl Tenor {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key: SecretString::new(api_key.into()),
            // todo :p
            country: CountryCode::US,
            locate: Locale::default(),
        }
    }

    pub async fn categories(&self) -> Result<categories::Response, Error> {
        let url = format!(
            "https://tenor.googleapis.com/v2/categories?key={}&country={}{}",
            self.api_key.expose_secret(),
            self.country,
            self.locate.to_query_parameter(),
        );

        self.categories_request(url).await
    }

    pub async fn categories_with_parameters(
        &self,
        parms: categories::Parameters,
    ) -> Result<categories::Response, Error> {
        let mut url = format!(
            "https://tenor.googleapis.com/v2/categories?key={}&country={}{}",
            self.api_key.expose_secret(),
            self.country,
            self.locate.to_query_parameter(),
        );

        url.push_str(&format!("&client_key={}", &parms.client_key));
        url.push_str(&parms.r#type.to_query_parameter());
        url.push_str(&parms.content_filter.to_query_parameter());

        self.categories_request(url).await
    }

    pub async fn categories_request(&self, url: String) -> Result<categories::Response, Error> {
        let response = reqwest::get(url).await;

        match response.is_ok() {
            true => debug!("fetch categories successful."),
            false => error!("fetch categories failed."),
        }

        let response_body = response?.text().await?;
        let obj: categories::Response = serde_json::from_str(&response_body)?;
        Ok(obj)
    }

    pub async fn featured(&self) -> Result<search::Response, Error> {
        let url = format!(
            "https://tenor.googleapis.com/v2/featured?key={}&country={}{}",
            self.api_key.expose_secret(),
            self.country,
            self.locate.to_query_parameter(),
        );
        self.featured_request(url).await
    }

    pub async fn featured_with_parameters(
        &self,
        parms: featured::Parameters,
    ) -> Result<search::Response, Error> {
        let mut url = format!(
            "https://tenor.googleapis.com/v2/featured?key={}&country={}{}",
            self.api_key.expose_secret(),
            self.country,
            self.locate.to_query_parameter(),
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
        // url.push_str(&format!("&pos={}", ));

        self.featured_request(url).await
    }

    pub async fn featured_request(&self, url: String) -> Result<search::Response, Error> {
        let response = reqwest::get(url).await;

        match response.is_ok() {
            true => debug!("fetch featured successful."),
            false => error!("fetch featured failed."),
        }

        let response_body = response?.text().await?;
        let obj: search::Response = serde_json::from_str(&response_body)?;
        Ok(obj)
    }

    pub async fn search(&self, query: String) -> Result<search::Response, Error> {
        let url = format!(
            "https://tenor.googleapis.com/v2/search?q={}&key={}&country={}{}",
            query,
            self.api_key.expose_secret(),
            self.country,
            self.locate.to_query_parameter(),
        );

        self.search_request(url).await
    }

    pub async fn search_with_parameters(
        &self,
        query: String,
        parms: search::Parameters,
    ) -> Result<search::Response, Error> {
        let mut url = format!(
            "https://tenor.googleapis.com/v2/search?q={}&key={}&country={}{}&client_key={}",
            query,
            self.api_key.expose_secret(),
            self.country,
            self.locate.to_query_parameter(),
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

        self.search_request(url).await
    }

    async fn search_request(&self, url: String) -> Result<search::Response, Error> {
        let response = reqwest::get(url).await;

        match response.is_ok() {
            true => debug!("search query successful.",),
            false => error!("search query failed {:#?}.", &response),
        }

        let response_body = response?.text().await?;
        let obj: search::Response = serde_json::from_str(&response_body)?;

        Ok(obj)
    }

    pub async fn trending(&self) -> Result<trending::Response, Error> {
        let url = format!(
            "https://tenor.googleapis.com/v2/trending_terms?key={}&country={}{}",
            self.api_key.expose_secret(),
            self.country,
            self.locate.to_query_parameter(),
        );

        println!("url {}", &url);

        self.trending_request(url).await
    }

    pub async fn trending_with_parameters(
        &self,
        parms: trending::Parameters,
    ) -> Result<trending::Response, Error> {
        let url = format!(
            "https://tenor.googleapis.com/v2/trending_terms?key={}&country={}{}&client_key={}&limit={}",
            self.api_key.expose_secret(),
            self.country,
            self.locate.to_query_parameter(),
            parms.client_key,
            parms.limit,
        );

        self.trending_request(url).await
    }

    pub async fn trending_request(&self, url: String) -> Result<trending::Response, Error> {
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
