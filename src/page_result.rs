use scraper::{Html, Selector};
use serde::Serialize;
use url::Url;

use crate::{extract_date, AudioBook};

/// Represents the result of a page fetch operation, containing information about the current page,
/// navigation to next and previous pages, and the content of the page.
///
/// # Fields
/// - `page`: The current page number.
/// - `has_next`: Indicates if there is a next page.
/// - `has_prev`: Indicates if there is a previous page.
/// - `next_page_url`: The URL of the next page, if available.
/// - `prev_page_url`: The URL of the previous page, if available.
/// - `url`: The URL of the current page.
/// - `body`: The HTML content of the current page (not serialized).
///
/// # Methods
/// - `new(url: Url, body: String) -> Self`: Constructs a new `PageResult` from the given URL and HTML body.
/// - `collect(&self) -> Result<Vec<AudioBook>, Box<dyn std::error::Error>>`: Extracts audiobook details from the page content.
/// - `has_next(&self) -> bool`: Returns `true` if there is a next page.
/// - `has_prev(&self) -> bool`: Returns `true` if there is a previous page.
/// - `next_page_url(&self) -> Option<Url>`: Returns the URL of the next page, if available.
/// - `prev_page_url(&self) -> Option<Url>`: Returns the URL of the previous page, if available.
use html_escape::decode_html_entities;
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PageResult {
    page: u32,
    has_next: bool,
    has_prev: bool,
    next_page_url: Option<Url>,
    prev_page_url: Option<Url>,
    url: Url,
    #[serde(skip)]
    body: String,
}

impl PageResult {
    /// Constructs a new `PageResult` from the given URL and HTML body.
    pub fn new(url: Url, body: String) -> Self {
        let document = Html::parse_document(&body);

        // Define selectors for current page, next, and previous buttons
        let current_page_selector = Selector::parse("span.pageNumberElement").unwrap();
        let next_button_selector = Selector::parse(".nextButton a").unwrap();
        let prev_button_selector = Selector::parse(".previousButton a").unwrap();

        // Extract current page number from the <span class="pageNumberElement">
        let page = document
            .select(&current_page_selector)
            .filter_map(|el| el.text().next())
            .filter_map(|text| text.parse::<u32>().ok())
            .next()
            .unwrap_or(1);

        // Determine next page URL based on the presence and status of the "Next" button
        let next_page_url = document
            .select(&next_button_selector)
            .next()
            .and_then(|el| {
                // Check if the button is enabled (aria-disabled should be absent)
                if el.value().attr("aria-disabled").is_none() {
                    el.value().attr("href").and_then(|href| url.join(href).ok())
                } else {
                    None
                }
            });
        let has_next = next_page_url.is_some();

        // Determine previous page URL based on the presence and status of the "Previous" button
        let prev_page_url = document
            .select(&prev_button_selector)
            .next()
            .and_then(|el| {
                // Check if the button is enabled (aria-disabled should be absent)
                if el.value().attr("aria-disabled").is_none() {
                    el.value().attr("href").and_then(|href| url.join(href).ok())
                } else {
                    None
                }
            });
        let has_prev = prev_page_url.is_some();

        Self {
            page,
            has_next,
            has_prev,
            next_page_url,
            prev_page_url,
            url,
            body,
        }
    }

    /// Extracts audiobook details from the page content.
    /// Returns a vector of `AudioBook` instances.
    /// # Errors
    /// Returns an error if any required data is missing or cannot be parsed.
    pub fn collect(&self) -> Result<Vec<AudioBook>, Box<dyn std::error::Error>> {
        let document = Html::parse_document(&self.body);

        // Define selectors for each piece of data
        let item_selector = Selector::parse("li.productListItem")?;
        let button_selector = Selector::parse("button[data-mp3]")?;
        let narrator_selector = Selector::parse("li.narratorLabel span.bc-text a")?;
        let language_selector = Selector::parse("li.languageLabel span.bc-text")?;
        let release_date_selector = Selector::parse("li.releaseDateLabel span.bc-text")?;
        let items = document.select(&item_selector);
        // Collect audiobook details
        let mut audiobooks = Vec::new();

        if items.clone().count() == 0 {
            return Err("No items found".into());
        }

        for item in items {
            if let Some(button) = item.select(&button_selector).next() {
                let mp3_url = Url::parse(
                    button
                        .value()
                        .attr("data-mp3")
                        .ok_or("Unexpected missing data-mp3")?,
                )?;

                let title = item
                    .attr("aria-label")
                    .map(|label| decode_html_entities(label).to_string())
                    .ok_or("Title not found")?;

                let narrator = item
                    .select(&narrator_selector)
                    .next()
                    .map(|el| el.inner_html())
                    .ok_or("Narrator not found")?;

                let language = item
                    .select(&language_selector)
                    .next()
                    .map(|el| el.inner_html().trim().to_string())
                    .ok_or("Language not found")?;

                let release_date = item
                    .select(&release_date_selector)
                    .next()
                    .map(|el| extract_date(el.inner_html().as_str()))
                    .transpose()?;

                audiobooks.push(AudioBook::new(
                    title,
                    narrator,
                    language,
                    release_date,
                    mp3_url,
                ));
            }
        }

        Ok(audiobooks)
    }

    pub fn has_next(&self) -> bool {
        self.has_next
    }

    pub fn has_prev(&self) -> bool {
        self.has_prev
    }

    pub fn next_page_url(&self) -> Option<Url> {
        self.next_page_url.clone()
    }

    pub fn prev_page_url(&self) -> Option<Url> {
        self.prev_page_url.clone()
    }

    pub fn url(&self) -> &Url {
        &self.url
    }

    pub fn page(&self) -> u32 {
        self.page
    }
}

/// Extracts audiobook details from the page content.
/// Returns a vector of `AudioBook` instances.
/// # Errors
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_page_result_new() {
        let url = Url::parse("https://example.com").unwrap();
        let body = fs::read_to_string("test_data/result_first_page.html").unwrap();
        let page_result = PageResult::new(url.clone(), body);

        assert_eq!(page_result.page(), 1);
        assert_eq!(page_result.url(), &url);
        assert!(page_result.has_next());
        assert!(!page_result.has_prev());
    }

    #[test]
    fn test_page_result_collect() {
        let url = Url::parse("https://example.com").unwrap();
        let body = fs::read_to_string("test_data/result_first_page.html").unwrap();
        let page_result = PageResult::new(url, body);

        let audiobooks = page_result.collect().unwrap();
        assert!(!audiobooks.is_empty());

        let first_audiobook = &audiobooks[0];
        assert_eq!(first_audiobook.title(), "1793 (Spanish Edition)");
        assert_eq!(first_audiobook.narrator(), "Jordi Salas");
        assert_eq!(first_audiobook.language(), "Español (Castellano)");
        assert!(first_audiobook.release_date().is_some());
        assert_eq!(
            first_audiobook.sample_url().as_str(),
            "https://samples.audible.com/bk/rhsp/002067/bk_rhsp_002067_sample.mp3"
        );
    }

    #[test]
    fn test_page_result_first() {
        let url = Url::parse("https://example.com").unwrap();
        let body = fs::read_to_string("test_data/result_first_page.html").unwrap();
        let page_result = PageResult::new(url, body);

        assert!(page_result.has_next());
        assert!(page_result.next_page_url().is_some());
        assert!(!page_result.has_prev());
        assert!(page_result.prev_page_url().is_none());
        assert_eq!(page_result.page(), 1);
    }

    #[test]
    fn test_page_result_last() {
        let url = Url::parse("https://example.com").unwrap();
        let body = fs::read_to_string("test_data/result_last_page.html").unwrap();
        let page_result = PageResult::new(url, body);

        assert!(!page_result.has_next());
        assert!(page_result.next_page_url().is_none());
        assert!(page_result.has_prev());
        assert!(page_result.prev_page_url().is_some());
        assert_eq!(page_result.page(), 3);
    }
}
