use reqwest::{header, Client};
use std::error::Error;

use crate::{QueryParams, Scrapper};

#[derive(Default)]
pub struct Builder {
    params: QueryParams,
    client: Option<Client>,
}

impl Builder {
    pub fn new(params: QueryParams) -> Self {
        Self {
            params,
            ..Self::default()
        }
    }

    pub fn params(&mut self, params: QueryParams) -> &mut Self {
        self.params = params;
        self
    }

    pub fn client(&mut self, client: Client) -> &mut Self {
        self.client = Some(client);
        self
    }

    pub fn build(&self) -> Result<Scrapper, Box<dyn Error>> {
        let client = self.client.clone().unwrap_or(Client::builder()
        .default_headers({
            let mut headers = header::HeaderMap::new();
            headers.insert(header::ACCEPT, header::HeaderValue::from_static(
                "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.7"
            ));
            headers.insert(header::ACCEPT_LANGUAGE, header::HeaderValue::from_static("es-ES,es;q=0.9,en;q=0.8"));
            headers.insert(header::CACHE_CONTROL, header::HeaderValue::from_static("max-age=0"));
            headers.insert(header::USER_AGENT, header::HeaderValue::from_static(
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/93.0.4577.82 Safari/537.36"
            ));
            headers.insert(header::UPGRADE_INSECURE_REQUESTS, header::HeaderValue::from_static("1"));
            headers
        }).build()?);

        Ok(Scrapper::new(client, self.params.clone()))
    }
}
