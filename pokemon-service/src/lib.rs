//! Pokémon Service
//!
//! This crate implements the Pokémon Service.
#![warn(missing_docs, missing_debug_implementations, rust_2018_idioms)]
use std::sync::Arc;

use aws_smithy_http_server::Extension;
use pokemon_service_server_sdk::{error, input, output};
use tracing_subscriber::{prelude::*, EnvFilter};

mod state;
pub mod tls;

pub use state::State;

/// Setup `tracing::subscriber` to read the log level from RUST_LOG environment variable.
pub fn setup_tracing() {
    let format = tracing_subscriber::fmt::layer()
        .with_ansi(true)
        .with_line_number(true)
        .with_level(true);
    let filter = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new("info"))
        .unwrap();
    tracing_subscriber::registry()
        .with(format)
        .with(filter)
        .init();
}

/// Retrieves information about a Pokémon species.
pub async fn evolve_pokemon(
    input: input::EvolvePokemonInput,
    state: Extension<Arc<State>>,
) -> Result<output::EvolvePokemonOutput, error::EvolvePokemonError> {
    Ok(output::EvolvePokemonOutput::builder()
        .name("")
        .build()
        .unwrap())
}

/// Retrieves information about a Pokémon species.
pub async fn get_pokemon_species(
    input: input::GetPokemonSpeciesInput,
    state: Extension<Arc<State>>,
) -> Result<output::GetPokemonSpeciesOutput, error::GetPokemonSpeciesError> {
    state
        .0
        .call_count
        .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    // We only support retrieving information about Pikachu.
    let pokemon = state.0.pokemons_translations.get(&input.name);
    match pokemon {
        Some(pokemon) => {
            tracing::debug!("Requested Pokémon is {}", input.name);
            let output = output::GetPokemonSpeciesOutput {
                name: String::from("pikachu"),
                flavor_text_entries: pokemon.to_vec(),
            };
            Ok(output)
        }
        None => {
            tracing::error!("Requested Pokémon {} not available", input.name);
            Err(error::PokemonNotFoundException {
                message: String::from("Requested Pokémon not available"),
            }
            .into())
        }
    }
}
