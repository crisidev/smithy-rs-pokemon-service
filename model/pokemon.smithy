$version: "1.0"

namespace org.crisidev

use aws.protocols#restJson1

/// The Pokémon Service allows you to retrieve information about Pokémon species.
@title("Pokémon Service")
@restJson1
service PokemonService {
    version: "2022-10-21",
    operations: [
        GetPokemonSpecies,
        EvolvePokemon
    ]
}

/// Retrieve information about a Pokémon species.
@readonly
@http(uri: "/pokemon-species/{name}", method: "GET")
operation GetPokemonSpecies {
    input: GetPokemonSpeciesInput,
    output: GetPokemonSpeciesOutput,
    errors: [PokemonNotFoundException],
}

/// Evolves a Pokémon to the next level.
@http(uri: "/evolve-pokemon/{name}", method: "POST")
operation EvolvePokemon {
    input: EvolvePokemonInput,
    output: EvolvePokemonOutput,
    errors: [PokemonEvolveException],
}

@input
structure GetPokemonSpeciesInput {
    /// The name for the Pokémon to search.
    @required
    @httpLabel
    name: String
}

@input
structure EvolvePokemonInput {
    /// The name for the Pokémon to evolve.
    @required
    @httpLabel
    name: String,
}

@output
structure GetPokemonSpeciesOutput {
    /// The name for this resource.
    @required
    name: String,

    /// A list of flavor text entries for this Pokémon species.
    @required
    flavorTextEntries: FlavorTextEntries
}

@output
structure EvolvePokemonOutput {
    /// The name for this resource.
    @required
    name: String,

    /// A list of flavor text entries for this Pokémon species.
    @required
    evolveChain: EvolveChain
}

list EvolveChain {
    member: Pokemon
}

// List of FlavorTextEntries
list FlavorTextEntries {
    member: FlavorText
}

structure FlavorText {
    /// The localized flavor text for an API resource in a specific language.
    @required
    flavorText: String,

    /// The language this name is in.
    @required
    language: Language,
}

/// Supported languages for FlavorText entries.
@enum([
    {
        name: "ENGLISH",
        value: "en",
        documentation: "American English.",
    },
    {
        name: "SPANISH",
        value: "es",
        documentation: "Español.",
    },
    {
        name: "ITALIAN",
        value: "it",
        documentation: "Italiano.",
    },
    {
        name: "JAPANESE",
        value: "jp",
        documentation: "日本語。",
    },
])
string Language

structure Pokemon {
    @required
    name: String,
    evolution: Pokemon,
}

// Return 404 to the client if the requested Pokémon does not exist.
@error("client")
@httpError(404)
structure PokemonNotFoundException {
    @required
    message: String,
}

// Return 404 to the client if the requested Pokémon does not exist.
@error("client")
@httpError(404)
structure PokemonEvolveException {
    @required
    message: String,
}
