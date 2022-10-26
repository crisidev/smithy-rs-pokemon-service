@input
structure GetPokemonSpeciesInput {
    /// The name for the Pokémon to search.
    @required
    @httpLabel
    name: String
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

// List of FlavorTextEntries
list FlavorTextEntries {
    member: FlavorText
}
