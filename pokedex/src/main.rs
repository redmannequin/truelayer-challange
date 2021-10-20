#[macro_use]
extern crate rocket;

mod routes;

// local imports
use routes::pokemon;

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/pokemon", routes![pokemon::pokemon, pokemon::translated])
}
