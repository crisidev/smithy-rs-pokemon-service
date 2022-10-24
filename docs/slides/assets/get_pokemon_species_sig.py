from libpokemon_service_server_sdk import App
from libpokemon_service_server_sdk.error import PokemonNotFoundException
from libpokemon_service_server_sdk.input import GetPokemonSpeciesInput
from libpokemon_service_server_sdk.output import GetPokemonSpeciesOutput

app = App()
# Register the context.
app.context(Context())

# Get the translation of a Pokémon specie or an error.
@app.get_pokemon_species
def get_pokemon_species(
    input: GetPokemonSpeciesInput, context: Context
) -> GetPokemonSpeciesOutput:
    raise PokemonNotFoundException(message=input.name)

app.run()
