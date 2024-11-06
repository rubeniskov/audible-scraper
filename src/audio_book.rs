use chrono::NaiveDate;
use serde::Serialize;
use url::Url;

/// Represents an audio book with relevant metadata and serialization capabilities.
///
/// # Fields
///
/// * `title` - The title of the audio book.
/// * `narrator` - The name of the person who narrates the audio book.
/// * `language` - The language in which the audio book is narrated. e.g. "Español (Castellano)".
/// * `release_date` - The release date of the audio book, if available.
/// * `sample_url` - A URL to a sample of the audio book.
///
/// # Serialization
///
/// This struct can be serialized to JSON with camelCase field names.
///
/// # Examples
///
/// ```
/// use chrono::NaiveDate;
/// use url::Url;
/// use audible_scrapper::AudioBook;
///
/// let title = String::from("The Rust Programming Language");
/// let narrator = String::from("Steve Klabnik");
/// let language = String::from("English");
/// let release_date = Some(NaiveDate::from_ymd(2019, 5, 15));
/// let sample_url = Url::parse("https://example.com/sample").unwrap();
///
/// let audio_book = AudioBook::new(title, narrator, language, release_date, sample_url);
///
/// assert_eq!(audio_book.title(), "The Rust Programming Language");
/// assert_eq!(audio_book.narrator(), "Steve Klabnik");
/// assert_eq!(audio_book.language(), "English");
/// assert_eq!(audio_book.release_date(), Some(NaiveDate::from_ymd(2019, 5, 15)));
/// assert_eq!(audio_book.sample_url().as_str(), "https://example.com/sample");
/// ```
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AudioBook {
    title: String,
    narrator: String,
    language: String,
    release_date: Option<NaiveDate>,
    sample_url: Url,
}

impl AudioBook {
    pub fn new(
        title: String,
        narrator: String,
        language: String,
        release_date: Option<NaiveDate>,
        sample_url: Url,
    ) -> Self {
        Self {
            title,
            narrator,
            language,
            release_date,
            sample_url,
        }
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn narrator(&self) -> &str {
        &self.narrator
    }

    pub fn language(&self) -> &str {
        &self.language
    }

    pub fn release_date(&self) -> Option<NaiveDate> {
        self.release_date
    }

    pub fn sample_url(&self) -> &Url {
        &self.sample_url
    }
}
