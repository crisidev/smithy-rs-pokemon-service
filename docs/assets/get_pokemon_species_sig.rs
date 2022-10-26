use pokemon_service::State;
use pokemon_service_server_sdk::{input, output, error};
use aws_smithy_http_server::Extension;

pub async fn get_pokemon_species(
    input: input::GetPokemonSpeciesInput,
    state: Extension<Arc<State>>,
) -> Result<
    output::GetPokemonSpeciesOutput,
    error::GetPokemonSpeciesError
> {
    let pokemon = state.0.pokemons_translations.get(&input.name)?;
    Ok(output::GetPokemonSpeciesOutput {
        name: input.name,
        flavor_text_entries: pokemon.to_vec(),
    })
}
