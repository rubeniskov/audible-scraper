use html_escape::decode_html_entities;
use scraper::{Html, Selector};
use serde::Serialize;
use url::Url;

use crate::AudioBook;

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

use chrono::NaiveDate;
use regex::Regex;

fn extract_date(text: &str) -> Result<NaiveDate, Box<dyn std::error::Error>> {
    // Define a regex pattern to capture any date in dd-mm-yy format
    let re = Regex::new(r"(\d{2}-\d{2}-\d{2})")?;

    // Find the date in the text
    let date_str = re
        .captures(text)
        .and_then(|caps| caps.get(1).map(|m| m.as_str()))
        .ok_or("Date not found")?;

    // Parse the date string to a NaiveDate object using the format dd-mm-yy
    let date = NaiveDate::parse_from_str(date_str, "%d-%m-%y")?;

    Ok(date)
}

impl PageResult {
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

    pub fn collect(&self) -> Result<Vec<AudioBook>, Box<dyn std::error::Error>> {
        let document = Html::parse_document(&self.body);

        // Define selectors for each piece of data
        let item_selector = Selector::parse("li.productListItem")?;
        let button_selector = Selector::parse("button[data-mp3]")?;
        let narrator_selector = Selector::parse("li.narratorLabel span.bc-text a")?;
        let language_selector = Selector::parse("li.languageLabel span.bc-text")?;
        let release_date_selector = Selector::parse("li.releaseDateLabel span.bc-text")?;

        // Collect audiobook details
        let mut audiobooks = Vec::new();
        for item in document.select(&item_selector) {
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
}
