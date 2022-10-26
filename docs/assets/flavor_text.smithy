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
])
string Language
