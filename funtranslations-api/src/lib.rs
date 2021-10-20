//! A small library containing funtranslations api types and functions

// third party imports
use reqwest::Result;
use serde::{Deserialize, Serialize};

// functranslations api endpoints
const FUNTRANSLATION_BASE_URL: &str = "https://api.funtranslations.com/translate";
const FUNTRANSLATION_YODA_ENDPOINT: &str = "/yoda.json";
const FUNTRANSLATION_SHAKESPEARE_ENDPOINT: &str = "/shakespeare.json";

/// a funtranslation result
#[derive(Deserialize, Serialize, Eq, PartialEq, Debug)]
pub struct FunTranslation {
    success: Success,
    contents: Contents,
}

#[derive(Deserialize, Serialize, Eq, PartialEq, Debug)]
pub struct Success {
    total: i32,
}

#[derive(Deserialize, Serialize, Eq, PartialEq, Debug)]
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

#[test]
fn test_deserialize_fun_translation() {
    let text = r#"{
        "success": {
          "total": 1
        },
        "contents": {
          "translated": "Thee did giveth mr. Tim a hearty meal,  but unfortunately what he did doth englut did maketh him kicketh the bucket.",
          "text": "You gave Mr. Tim a hearty meal, but unfortunately what he ate made him die.",
          "translation": "shakespeare"
        }
      }"#;
    let translated_text = serde_json::from_str::<FunTranslation>(text);
    assert!(translated_text.is_ok());
    assert_eq!(translated_text.unwrap(), FunTranslation{
        success: Success {
            total: 1
        },
        contents: Contents {
            translated: "Thee did giveth mr. Tim a hearty meal,  but unfortunately what he did doth englut did maketh him kicketh the bucket.".into(),
            text: "You gave Mr. Tim a hearty meal, but unfortunately what he ate made him die.".into(),
            translation: "shakespeare".into()
        }
    });
}
