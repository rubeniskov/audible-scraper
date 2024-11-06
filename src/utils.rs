use chrono::NaiveDate;
use regex::Regex;

/// Extracts a date in the format dd-mm-yy from a given text string.
///
/// # Arguments
///
/// * `text` - A string slice that holds the text containing the date.
///
/// # Returns
///
/// * `Result<NaiveDate, Box<dyn std::error::Error>>` - A result containing the extracted date or an error.
///
/// # Examples
///
/// ```
/// use chrono::NaiveDate;
/// use audible_scraper::extract_date;
/// let text = "The event is scheduled for 12-05-21.";
/// let date = extract_date(text).unwrap();
/// assert_eq!(date, NaiveDate::from_ymd_opt(2021, 5, 12).unwrap());
/// ```
pub fn extract_date(text: &str) -> Result<NaiveDate, Box<dyn std::error::Error>> {
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
