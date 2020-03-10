use reqwest::{Error, blocking};

/**
 * Fetches the content from the provided Url
 */
pub fn fetch_text_sync(url: &str) -> Result<String, Error> {
    let resp = blocking::get(url)?.text()?;                  // Get the response from the provided url as text asynchronously
    Ok(resp)                                                            // In case of Success return the response-text
}
