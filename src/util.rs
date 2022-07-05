use std::error::Error;
use serde_json::Value;

const API_LINK: &str = "https://www.googleapis.com/books/v1/volumes?q=";

pub async fn retrieve_image(cover: &str) -> Result<String, Box<dyn Error>> {
    let url = [API_LINK, cover.replace(" ", "+").as_str()].join(""); 

    let resp = reqwest::get(url)
        .await?
        .text()
        .await?;

    let root: Value = serde_json::from_str(&resp)?;
    let image: Option<&str> = root.get("items")
        .and_then(|value| value.get(0))
        .and_then(|value| value.get("volumeInfo"))
        .and_then(|value| value.get("imageLinks"))
        .and_then(|value| value.get("thumbnail"))
        .and_then(|value| value.as_str());

    Ok(image.unwrap().to_string().clone())
}