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

/// updates a pokemons description with a fun translation
///
///
/// # Arguments
///
/// * `pokemon` - A pokemont to update
async fn update_description(pokemon: &mut Pokemon) {
    if let Some(ref mut description) = pokemon.description {
        if pokemon.is_legendary {
            if let Ok(new_description) = translate_to_yoda(description.as_ref()).await {
                *description = new_description;
                return;
            }
        } else if let Some(habitat) = pokemon.habitat.as_deref() {
            if habitat == "cave" {
                if let Ok(new_description) = translate_to_yoda(description.as_ref()).await {
                    *description = new_description;
                    return;
                }
            }
        }
        if let Ok(new_description) = translate_to_shakespeare(description.as_ref()).await {
            *description = new_description;
        }
    }
}
