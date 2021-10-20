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
            pokemon.update_description().await;
            Ok(Json(pokemon))
        }
        Err(_err) => Err(Status::NotAcceptable),
    }
}

/// a list a available translation types
#[derive(Debug, Eq, PartialEq)]
enum TranslateType {
    Yoda,
    Shakespeare,
}

/// represents a pokemon
#[derive(Serialize)]
pub struct Pokemon {
    name: String,
    description: Option<String>,
    habitat: Option<String>,
    is_legendary: bool,
}

impl Pokemon {
    /// updates a pokemons description with a fun translation
    pub async fn update_description(&mut self) {
        match self.get_translation_type() {
            Some(TranslateType::Yoda) => {
                if let Ok(new_description) =
                    translate_to_yoda(self.description.as_ref().unwrap()).await
                {
                    self.description = Some(new_description);
                }
            }
            Some(TranslateType::Shakespeare) => {
                if let Ok(new_description) =
                    translate_to_shakespeare(self.description.as_ref().unwrap()).await
                {
                    self.description = Some(new_description);
                }
            }
            None => (),
        }
    }

    /// returns the `TranslateType` of the pokemon
    fn get_translation_type(&self) -> Option<TranslateType> {
        if self.description.is_some() {
            if self.is_legendary {
                return Some(TranslateType::Yoda);
            } else if let Some(habitat) = self.habitat.as_deref() {
                if habitat == "cave" {
                    return Some(TranslateType::Yoda);
                }
                return Some(TranslateType::Shakespeare);
            }
            return Some(TranslateType::Shakespeare);
        }
        None
    }
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

#[test]
fn update_get_translation_type_is_legendary() {
    let onix = Pokemon {
        name: "onix".into(),
        description: Some("As it grows, the stone portions of its body harden to become similar to a diamond, but colored black.".into()),
        habitat: Some("cave".into()),
        is_legendary: true,
    };
    assert_eq!(onix.get_translation_type(), Some(TranslateType::Yoda));
}

#[test]
fn update_get_translation_type_cave_habitat() {
    let onix = Pokemon {
        name: "onix".into(),
        description: Some("As it grows, the stone portions of its body harden to become similar to a diamond, but colored black.".into()),
        habitat: Some("cave".into()),
        is_legendary: true,
    };
    assert_eq!(onix.get_translation_type(), Some(TranslateType::Yoda));
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
        onix.get_translation_type(),
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
    assert_eq!(onix.get_translation_type(), None);
}
