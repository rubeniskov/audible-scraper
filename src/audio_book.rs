use chrono::NaiveDate;
use serde::Serialize;
use url::Url;

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
        mp3_url: Url,
    ) -> Self {
        Self {
            title,
            narrator,
            language,
            release_date,
            sample_url: mp3_url,
        }
    }
}
