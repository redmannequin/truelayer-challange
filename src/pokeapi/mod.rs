//! containes pokeapi realates types and functions
pub mod types;

// third party imports
use reqwest::Result;

// local imports
use types::pokemon::PokemonSpecies;

const POKEAPI_POKEMON_SPECIES_URL: &str = "https://pokeapi.co/api/v2/pokemon-species";

/// Returns a pokemon-species given a pokemon name
///
/// # Arguments
///
/// * `name` - A string slice with the name of a pokemon
pub async fn get_pokemon_species(name: &str) -> Result<PokemonSpecies> {
    let pokemon_species_url = format!("{}/{}", POKEAPI_POKEMON_SPECIES_URL, name);
    reqwest::get(pokemon_species_url.as_str())
        .await?
        .json::<PokemonSpecies>()
        .await
}
