//! utility types

// thrid party imports
use serde::{Deserialize, Serialize};

/// APIResource
///
///
#[derive(Deserialize, Serialize)]
pub struct APIResource {
    pub url: String,
}

/// Description
///
///
#[derive(Deserialize, Serialize)]
pub struct Description {
    pub description: String,
    pub language: NamedAPIResource,
}

/// FlavorText
///
///
#[derive(Deserialize, Serialize)]
pub struct FlavorText {
    pub flavor_text: String,
    pub language: NamedAPIResource,
    pub version: NamedAPIResource,
}

/// Name
///
///
#[derive(Deserialize, Serialize)]
pub struct Name {
    pub name: String,
    pub language: NamedAPIResource,
}

/// NamedAPIResource
///
///
#[derive(Deserialize, Serialize)]
pub struct NamedAPIResource {
    pub name: String,
    pub url: String,
}
