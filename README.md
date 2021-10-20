# TrueLayer Challange - Pokedex

A pokedex service that allows you to query for pokemon information and includes fun endpoints

## Endpoints

A list of the available endpoints

| endpoint                     | HTTP method |                        description                             |
|------------------------------|-------------|----------------------------------------------------------------|
| `/pokemon/<name>`            | `GET`       | returns a `Pokemon` with the given name                        |
| `/pokemon/translated/<name>` | `GET`       | returns a `Pokemon` with the given name with a fun description |


## Types

Type definitions of the service endpoints

### Pokemon

| field        | type            |                 description                   |
|--------------|-----------------|-----------------------------------------------|
| name         | `str`           | the name of the pokemon                       |
| description  | `str` or `null` | a small description of the pokemon            |
| habitat      | `str` or `null` | the habitat of the pokemon                    |
| is_legendary | `bool`          | a flag to signify of the pokemon is legendary |


## Runnning  

The service can be run on your system sandalone or via docker

### Standalone 

institution for building and running `pokedex` on your local computer

#### requirements

- `rust`
- `openssl`

#### steps
1. `clone` the the `repo` onto your computer
1. `cd` into the root folder of the `repo`
1. run `cargo build --release --bin pokedex` to build the service
1. run `cargo run --release --bin pokedex` to run the service 

### In Docker Container

instructions for running `pokedex` using docker

#### requirements

- `docker`
- `docker-compose`

#### steps

1. `clone` the the `repo` onto your computer
1. `cd` into the root folder of the `repo`
1. run `docker-compose up`