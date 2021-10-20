//! Pokemon types
//!
//! API definitions can be found at https://pokeapi.co/docs/v2#pokemon-section

// third party imports
use serde::{Deserialize, Serialize};

// local imports
use super::utility::{APIResource, Description, FlavorText, Name, NamedAPIResource};

/// PokemonSpecies
///
/// https://pokeapi.co/docs/v2#pokemon-species
#[derive(Deserialize, Serialize)]
pub struct PokemonSpecies {
    pub id: i32,
    pub name: String,
    pub order: i32,
    pub gender_rate: i32,
    pub capture_rate: i32,
    pub base_happiness: i32,
    pub is_baby: bool,
    pub is_legendary: bool,
    pub is_mythical: bool,
    pub hatch_counter: i32,
    pub has_gender_differences: bool,
    pub forms_switchable: bool,
    pub growth_rate: NamedAPIResource,
    pub pokedex_numbers: Vec<PokemonSpeciesDexEntry>,
    pub egg_groups: Vec<NamedAPIResource>,
    pub color: NamedAPIResource,
    pub shape: NamedAPIResource,
    pub evolves_from_species: Option<NamedAPIResource>,
    pub evolution_chain: APIResource,
    pub habitat: Option<NamedAPIResource>,
    pub generation: NamedAPIResource,
    pub names: Vec<Name>,
    pub pal_park_encounters: Vec<PalParkEncounterArea>,
    pub flavor_text_entries: Vec<FlavorText>,
    pub form_descriptions: Vec<Description>,
    pub genera: Vec<Genus>,
    pub varieties: Vec<PokemonSpeciesVariety>,
}

/// Genus
///
///
#[derive(Deserialize, Serialize)]
pub struct Genus {
    pub genus: String,
    pub language: NamedAPIResource,
}

/// PalParkEncounterArea
///
///
#[derive(Deserialize, Serialize)]
pub struct PalParkEncounterArea {
    pub base_score: i32,
    pub rate: i32,
    pub area: NamedAPIResource,
}

/// PokemonAbility
///
///
#[derive(Deserialize, Serialize)]
pub struct PokemonAbility {
    pub is_hidden: bool,
    pub slot: i32,
    pub ability: NamedAPIResource,
}

/// PokemonSpeciesDexEntry
///
///
#[derive(Deserialize, Serialize)]
pub struct PokemonSpeciesDexEntry {
    pub entry_number: i32,
    pub pokedex: NamedAPIResource,
}

/// PokemonSpeciesVariety
///
///
#[derive(Deserialize, Serialize)]
pub struct PokemonSpeciesVariety {
    pub is_default: bool,
    pub pokemon: NamedAPIResource,
}
