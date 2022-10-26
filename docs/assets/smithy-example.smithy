$version: "2"
namespace org.crisidev

/// The Pokémon Service allows you to retrieve
/// information about Pokémon species.
service PokemonService {
    version: "2022-10-26"
    operations: [
        GetPokemon,
    ]
}

/// Retrieve information about a Pokémon species.
@readonly
@http(uri: "/pokemon/{name}", method: "GET")
operation GetPokemon {
    input: GetPokemonInput,
    output: GetPokemonOutput,
    errors: [PokemonNotFoundException],
}
