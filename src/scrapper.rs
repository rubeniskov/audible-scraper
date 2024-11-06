use reqwest::Client;
use std::error::Error;
use url::Url;

use crate::{PageResult, QueryParams};

// Struct to hold search parameters for AudibleScrapper
pub struct Scrapper {
    client: Client,
    params: QueryParams,
}

impl Scrapper {
    pub fn new(client: Client, params: QueryParams) -> Self {
        Self { client, params }
    }

    pub async fn fetch(&self) -> Result<PageResult, Box<dyn Error>> {
        Ok(self.fetch_page(self.params.clone().build_url()?).await?)
    }

    // Fetch all pages until the last one
    pub async fn fetch_all(&self) -> Result<Vec<PageResult>, Box<dyn Error>> {
        let mut result = self.fetch().await?;
        let mut results = vec![result.clone()];

        // Continue fetching while there is a next page
        while let Some(next_page_url) = result.next_page_url() {
            result = self.fetch_page(next_page_url).await?; // Fetch the next page by URL
            results.push(result.clone());
        }

        Ok(results)
    }

    async fn fetch_page(&self, url: Url) -> Result<PageResult, Box<dyn Error>> {
        let res = self.client.get(url.clone()).send().await?;
        Ok(PageResult::new(url, res.text().await?))
    }
}
