//! A small library containing funtranslations api types and functions

// third party imports
use reqwest::Result;
use serde::{Deserialize, Serialize};

// functranslations api endpoints
const FUNTRANSLATION_BASE_URL: &str = "https://api.funtranslations.com/translate";
const FUNTRANSLATION_YODA_ENDPOINT: &str = "/yoda.json";
const FUNTRANSLATION_SHAKESPEARE_ENDPOINT: &str = "/shakespeare.json";

/// a funtranslation result
#[derive(Deserialize, Serialize)]
pub struct FunTranslation {
    success: Success,
    contents: Contents,
}

#[derive(Deserialize, Serialize)]
pub struct Success {
    total: i32,
}

#[derive(Deserialize, Serialize)]
pub struct Contents {
    translated: String,
    text: String,
    translation: String,
}

/// Returns a yoda translation of the given text
///
/// # Arguments
///
/// * `text` - A string slice with the text to translate
pub async fn translate_to_yoda(text: &str) -> Result<String> {
    let yoda_url = format!(
        "{}{}?text={}",
        FUNTRANSLATION_BASE_URL, FUNTRANSLATION_YODA_ENDPOINT, text
    );
    let res = reqwest::get(yoda_url.as_str())
        .await?
        .json::<FunTranslation>()
        .await?;
    Ok(res.contents.translated)
}

/// Returns a shakespeare translation of the given text
///
/// # Arguments
///
/// * `text` - A string slice with the text to translate
pub async fn translate_to_shakespeare(text: &str) -> Result<String> {
    let shakespeare_url = format!(
        "{}{}?text={}",
        FUNTRANSLATION_BASE_URL, FUNTRANSLATION_SHAKESPEARE_ENDPOINT, text
    );
    let res = reqwest::get(shakespeare_url.as_str())
        .await?
        .json::<FunTranslation>()
        .await?;
    Ok(res.contents.translated)
}
