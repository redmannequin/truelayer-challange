//! Pokemon types
//!
//! API definitions can be found at https://pokeapi.co/docs/v2#pokemon-section

// third party imports
use serde::Deserialize;

// local imports
use super::utility::{APIResource, Description, FlavorText, Name, NamedAPIResource};

/// PokemonSpecies
///
/// https://pokeapi.co/docs/v2#pokemon-species
#[derive(Deserialize)]
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
#[derive(Deserialize)]
pub struct Genus {
    pub genus: String,
    pub language: NamedAPIResource,
}

/// PalParkEncounterArea
///
///
#[derive(Deserialize)]
pub struct PalParkEncounterArea {
    pub base_score: i32,
    pub rate: i32,
    pub area: NamedAPIResource,
}

/// PokemonAbility
///
///
#[derive(Deserialize)]
pub struct PokemonAbility {
    pub is_hidden: bool,
    pub slot: i32,
    pub ability: NamedAPIResource,
}

/// PokemonSpeciesDexEntry
///
///
#[derive(Deserialize)]
pub struct PokemonSpeciesDexEntry {
    pub entry_number: i32,
    pub pokedex: NamedAPIResource,
}

/// PokemonSpeciesVariety
///
///
#[derive(Deserialize)]
pub struct PokemonSpeciesVariety {
    pub is_default: bool,
    pub pokemon: NamedAPIResource,
}

#[test]
fn test_deserialize_pokemon_species() {
    let bulbasaur = r#"
    {
        "base_happiness": 70,
        "capture_rate": 45,
        "color": {
          "name": "green",
          "url": "https://pokeapi.co/api/v2/pokemon-color/5/"
        },
        "egg_groups": [
          {
            "name": "monster",
            "url": "https://pokeapi.co/api/v2/egg-group/1/"
          },
          {
            "name": "plant",
            "url": "https://pokeapi.co/api/v2/egg-group/7/"
          }
        ],
        "evolution_chain": {
          "url": "https://pokeapi.co/api/v2/evolution-chain/1/"
        },
        "evolves_from_species": null,
        "flavor_text_entries": [
          {
            "flavor_text": "A strange seed was\nplanted on its\nback at birth.\fThe plant sprouts\nand grows with\nthis POKéMON.",
            "language": {
              "name": "en",
              "url": "https://pokeapi.co/api/v2/language/9/"
            },
            "version": {
              "name": "red",
              "url": "https://pokeapi.co/api/v2/version/1/"
            }
          },
          {
            "flavor_text": "A strange seed was\nplanted on its\nback at birth.\fThe plant sprouts\nand grows with\nthis POKéMON.",
            "language": {
              "name": "en",
              "url": "https://pokeapi.co/api/v2/language/9/"
            },
            "version": {
              "name": "blue",
              "url": "https://pokeapi.co/api/v2/version/2/"
            }
          }
        ],
        "form_descriptions": [],
        "forms_switchable": false,
        "gender_rate": 1,
        "genera": [
          {
            "genus": "たねポケモン",
            "language": {
              "name": "ja-Hrkt",
              "url": "https://pokeapi.co/api/v2/language/1/"
            }
          },
          {
            "genus": "씨앗포켓몬",
            "language": {
              "name": "ko",
              "url": "https://pokeapi.co/api/v2/language/3/"
            }
          },
          {
            "genus": "種子寶可夢",
            "language": {
              "name": "zh-Hant",
              "url": "https://pokeapi.co/api/v2/language/4/"
            }
          },
          {
            "genus": "Pokémon Graine",
            "language": {
              "name": "fr",
              "url": "https://pokeapi.co/api/v2/language/5/"
            }
          },
          {
            "genus": "Samen",
            "language": {
              "name": "de",
              "url": "https://pokeapi.co/api/v2/language/6/"
            }
          }
        ],
        "generation": {
          "name": "generation-i",
          "url": "https://pokeapi.co/api/v2/generation/1/"
        },
        "growth_rate": {
          "name": "medium-slow",
          "url": "https://pokeapi.co/api/v2/growth-rate/4/"
        },
        "habitat": {
          "name": "grassland",
          "url": "https://pokeapi.co/api/v2/pokemon-habitat/3/"
        },
        "has_gender_differences": false,
        "hatch_counter": 20,
        "id": 1,
        "is_baby": false,
        "is_legendary": false,
        "is_mythical": false,
        "name": "bulbasaur",
        "names": [
          {
            "language": {
              "name": "ja-Hrkt",
              "url": "https://pokeapi.co/api/v2/language/1/"
            },
            "name": "フシギダネ"
          },
          {
            "language": {
              "name": "roomaji",
              "url": "https://pokeapi.co/api/v2/language/2/"
            },
            "name": "Fushigidane"
          },
          {
            "language": {
              "name": "ko",
              "url": "https://pokeapi.co/api/v2/language/3/"
            },
            "name": "이상해씨"
          }
        ],
        "order": 1,
        "pal_park_encounters": [
          {
            "area": {
              "name": "field",
              "url": "https://pokeapi.co/api/v2/pal-park-area/2/"
            },
            "base_score": 50,
            "rate": 30
          }
        ],
        "pokedex_numbers": [
          {
            "entry_number": 1,
            "pokedex": {
              "name": "national",
              "url": "https://pokeapi.co/api/v2/pokedex/1/"
            }
          },
          {
            "entry_number": 1,
            "pokedex": {
              "name": "kanto",
              "url": "https://pokeapi.co/api/v2/pokedex/2/"
            }
          },
          {
            "entry_number": 226,
            "pokedex": {
              "name": "original-johto",
              "url": "https://pokeapi.co/api/v2/pokedex/3/"
            }
          }
        ],
        "shape": {
          "name": "quadruped",
          "url": "https://pokeapi.co/api/v2/pokemon-shape/8/"
        },
        "varieties": [
          {
            "is_default": true,
            "pokemon": {
              "name": "bulbasaur",
              "url": "https://pokeapi.co/api/v2/pokemon/1/"
            }
          }
        ]
      }
    "#;

    let pokemon = serde_json::from_str::<PokemonSpecies>(bulbasaur);
    assert!(pokemon.is_ok());
}
