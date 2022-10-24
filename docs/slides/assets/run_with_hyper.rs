use aws_smithy_http_server::AddExtensionLayer
use aws_smithy_http_server::instrumentation::plugin::InstrumentExt;
use pokemon_service::{get_pokemon_species, State};
use pokemon_service_sdk::service::PokemonService;

#[tokio::main]
pub async fn main() {
    // Register the shared state.
    let shared_state = Arc::new(State::default());
    // Build the application router.
    let app = PokemonService::builder()
        .get_pokemon_species(get_pokemon_species)
        .instrument()
        .build()
        .layer(&AddExtensionLayer::new(shared_state));

    // Run Hyper forever-ish..
    let server = hyper::Server::bind(&bind).serve(app.into_make_service());
    if let Err(err) = server.await {
        eprintln!("server error: {}", err);
    }
}
