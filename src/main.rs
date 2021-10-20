#[macro_use]
extern crate rocket;

mod pokeapi;

// third party imports
use rocket::http::Status;
use rocket::serde::json::Json;
use serde::Serialize;

// local imports
use pokeapi::get_pokemon_species;
use pokeapi::types::pokemon::PokemonSpecies;

/// represents a pockemon
#[derive(Serialize)]
pub struct Pokemon {
    name: String,
    description: Option<String>,
    habitat: Option<String>,
    is_lengerday: bool,
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
                .map(|x| x.flavor_text.clone()),
            habitat: pokemon_species.habitat.map(|x| x.name),
            is_lengerday: pokemon_species.is_legendary,
        }
    }
}

#[get("/<name>")]
async fn pokemon(name: &str) -> Result<Json<Pokemon>, Status> {
    match get_pokemon_species(name).await {
        Ok(pokemon) => Ok(Json(pokemon.into())),
        Err(_err) => Err(Status::NotAcceptable),
    }
}

#[get("/translated/<name>")]
async fn translated(name: &str) -> &'static str {
    unimplemented!()
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/pokemon", routes![pokemon, translated])
}
