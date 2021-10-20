use pokedex::rocket;
use rocket::http::Status;
use rocket::local::Client;

#[test]
fn pokedex_pokemon_root_endpoint() {
    let client = Client::new(rocket()).expect("valid rocket instance");
    let mut response = client.get("/pokemon/mewtwo").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.body_string(), Some(r#"{"name":"mewtwo","description":"It was created by a scientist after years of horrific gene splicing and DNA engineering experiments.","habitat":"rare","is_legendary":true}"#.into()));
}

#[test]
fn pokedex_pokemon_translated_endpoint_yoda_is_legendary() {
    let client = Client::new(rocket()).expect("valid rocket instance");
    let mut response = client.get("/pokemon/translated/mewtwo").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.body_string(), Some(r#"{"name":"mewtwo","description":"Created by a scientist after years of horrific gene splicing and dna engineering experiments,  it was.","habitat":"rare","is_legendary":true}"#.into()));
}

fn pokedex_pokemon_translated_endpoint_yoda_cave_habitat() {
    let client = Client::new(rocket()).expect("valid rocket instance");
    let mut response = client.get("/pokemon/translated/onix").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.body_string(), Some(r#"{"name":"onix","description":"As it grows,To become similar to a diamond,  the stone portions of its body harden,But colored black.","habitat":"cave","is_legendary":false}"#.into()));
}

#[test]
fn pokedex_pokemon_translated_endpoint_shakespeare() {
    let client = Client::new(rocket()).expect("valid rocket instance");
    let mut response = client.get("/pokemon/translated/bulbasaur").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.body_string(), Some(r#"{"name":"bulbasaur","description":"A strange seed wast planted on its back at birth. The plant sprouts and grows with this pokémon.","habitat":"grassland","is_legendary":false}"#.into()));
}
