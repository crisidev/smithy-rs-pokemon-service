/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

// Files here are for running integration tests.
// These tests only have access to your crate's public API.
// See: https://doc.rust-lang.org/book/ch11-03-test-organization.html#integration-tests

use crate::helpers::{client, PokemonService};

use serial_test::serial;

mod helpers;

#[tokio::test]
#[serial]
async fn simple_integration_test() {
    let _program = PokemonService::run().await;

    let pokemon_species_output = client()
        .get_pokemon_species()
        .name("pikachu")
        .send()
        .await
        .unwrap();
    assert_eq!("pikachu", pokemon_species_output.name().unwrap());

    let pokemon_species_error = client()
        .get_pokemon_species()
        .name("some_pokémon")
        .send()
        .await
        .unwrap_err();
    assert_eq!(
        r#"PokemonNotFoundError [PokemonNotFoundException]: Requested Pokémon not available"#,
        pokemon_species_error.to_string()
    );
}
