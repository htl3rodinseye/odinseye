use reqwest;

/**
 * Fetches the content from the provided Url
 */
pub async fn fetch_text(url: &str) -> Result<String, reqwest::Error> {
    let resp = reqwest::get(url).await?.text().await?;                  // Get the response from the provided url as text asynchronously
    Ok(resp)                                                            // In case of Success return the response-text
}
