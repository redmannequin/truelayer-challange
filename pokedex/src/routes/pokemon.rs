//! Contains all of the `/pokemon` endpoint routes and types

// third party imports
use funtranslations_api::{translate_to_shakespeare, translate_to_yoda};
use pokeapi::get_pokemon_species;
use pokeapi::types::pokemon::PokemonSpecies;
use rocket::http::Status;
use rocket::serde::json::Json;
use serde::Serialize;

#[get("/<name>")]
pub async fn pokemon(name: &str) -> Result<Json<Pokemon>, Status> {
    match get_pokemon_species(name).await {
        Ok(pokemon) => Ok(Json(pokemon.into())),
        Err(_err) => Err(Status::NotAcceptable),
    }
}

#[get("/translated/<name>")]
pub async fn translated(name: &str) -> Result<Json<Pokemon>, Status> {
    match get_pokemon_species(name).await {
        Ok(pokemon) => {
            let mut pokemon: Pokemon = pokemon.into();
            update_description(&mut pokemon).await;
            Ok(Json(pokemon))
        }
        Err(_err) => Err(Status::NotAcceptable),
    }
}

/// represents a pokemon
#[derive(Serialize)]
pub struct Pokemon {
    name: String,
    description: Option<String>,
    habitat: Option<String>,
    is_legendary: bool,
}

impl From<PokemonSpecies> for Pokemon {
    fn from(pokemon_species: PokemonSpecies) -> Pokemon {
        Pokemon {
            name: pokemon_species.name,
            description: pokemon_species
                .flavor_text_entries
                .iter()
                .filter(|x| x.language.name == "en")
                .next()
                .map(|x| {
                    x.flavor_text
                        .clone()
                        .split_whitespace()
                        .collect::<Vec<_>>()
                        .join(" ")
                }),
            habitat: pokemon_species.habitat.map(|x| x.name),
            is_legendary: pokemon_species.is_legendary,
        }
    }
}

/// a list a available translation types
#[derive(Debug, Eq, PartialEq)]
enum TranslateType {
    Yoda,
    Shakespeare,
}

/// updates a pokemons description with a fun translation
///
///
/// # Arguments
///
/// * `pokemon` - A pokemont to update
async fn update_description(pokemon: &mut Pokemon) {
    match get_translation_type(pokemon) {
        Some(TranslateType::Yoda) => {
            if let Ok(new_description) =
                translate_to_yoda(pokemon.description.as_ref().unwrap()).await
            {
                pokemon.description = Some(new_description);
            }
        }
        Some(TranslateType::Shakespeare) => {
            if let Ok(new_description) =
                translate_to_shakespeare(pokemon.description.as_ref().unwrap()).await
            {
                pokemon.description = Some(new_description);
            }
        }
        None => (),
    }
}

/// gets the translation type for a given pokemon
///
/// # Arguments
///
/// * `pokemon` - the pokemont in question
fn get_translation_type(pokemon: &Pokemon) -> Option<TranslateType> {
    if pokemon.description.is_some() {
        if pokemon.is_legendary {
            return Some(TranslateType::Yoda);
        } else if let Some(habitat) = pokemon.habitat.as_deref() {
            if habitat == "cave" {
                return Some(TranslateType::Yoda);
            }
            return Some(TranslateType::Shakespeare);
        }
        return Some(TranslateType::Shakespeare);
    }
    None
}

#[test]
fn update_get_translation_type_is_legendary() {
    let onix = Pokemon {
        name: "onix".into(),
        description: Some("As it grows, the stone portions of its body harden to become similar to a diamond, but colored black.".into()),
        habitat: Some("cave".into()),
        is_legendary: true,
    };
    assert_eq!(get_translation_type(&onix), Some(TranslateType::Yoda));
}

#[test]
fn update_get_translation_type_cave_habitat() {
    let onix = Pokemon {
        name: "onix".into(),
        description: Some("As it grows, the stone portions of its body harden to become similar to a diamond, but colored black.".into()),
        habitat: Some("cave".into()),
        is_legendary: true,
    };
    assert_eq!(get_translation_type(&onix), Some(TranslateType::Yoda));
}

#[test]
fn update_get_translation_type_shakespeare() {
    let onix = Pokemon {
        name: "onix".into(),
        description: Some("As it grows, the stone portions of its body harden to become similar to a diamond, but colored black.".into()),
        habitat: Some("water".into()),
        is_legendary: false,
    };
    assert_eq!(
        get_translation_type(&onix),
        Some(TranslateType::Shakespeare)
    );
}

#[test]
fn update_get_translation_type_none() {
    let onix = Pokemon {
        name: "onix".into(),
        description: None,
        habitat: Some("water".into()),
        is_legendary: false,
    };
    assert_eq!(get_translation_type(&onix), None);
}
