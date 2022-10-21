//! PokémonService shared state.
use std::{collections::HashMap, sync::atomic::AtomicU64};

use pokemon_service_server_sdk::model::{FlavorText, Language};

/// Some applications may want to manage state between handlers. Imagine having a database connection pool
/// that can be shared between different handlers and operation implementations.
/// State management can be expressed in a struct where the attributes hold the shared entities.
///
/// **NOTE: It is up to the implementation of the state structure to handle concurrency by protecting**
/// **its attributes using synchronization mechanisms.**
///
/// The framework stores the `Arc<T>` inside an [`http::Extensions`] and conveniently passes it to
/// the operation's implementation, making it able to handle operations with two different async signatures:
/// * `FnOnce(InputType) -> Future<OutputType>`
/// * `FnOnce(InputType, Extension<Arc<T>>) -> Future<OutputType>`
///
/// Wrapping the service with a [`tower::Layer`] will allow to have operations' signatures with and without shared state:
///
/// ```compile_fail
/// use std::sync::Arc;
/// use aws_smithy_http_server::{AddExtensionLayer, Extension, Router};
/// use tower::ServiceBuilder;
/// use tokio::sync::RwLock;
///
/// // Shared state,
/// #[derive(Debug, State)]
/// pub struct State {
///     pub count: RwLock<u64>
/// }
///
/// // Operation implementation with shared state.
/// async fn operation_with_state(input: Input, state: Extension<Arc<State>>) -> Output {
///     let mut count = state.0.write().await;
///     *count += 1;
///     Ok(Output::new())
/// }
///
/// // Operation implementation without shared state.
/// async fn operation_without_state(input: Input) -> Output {
///     Ok(Output::new())
/// }
///
/// let app: Router = OperationRegistryBuilder::default()
///     .operation_with_state(operation_with_state)
///     .operation_without_state(operation_without_state)
///     .build()
///     .unwrap()
///     .into();
/// let shared_state = Arc::new(State::default());
/// let app = app.layer(ServiceBuilder::new().layer(AddExtensionLayer::new(shared_state)));
/// let server = hyper::Server::bind(&"0.0.0.0:13734".parse().unwrap()).serve(app.into_make_service());
/// ...
/// ```
///
/// Without the middleware layer, the framework will require operations' signatures without
/// the shared state.
///
/// [`middleware`]: [`aws_smithy_http_server::AddExtensionLayer`]
#[derive(Debug)]
pub struct State {
    pub(crate) pokemons_translations: HashMap<String, Vec<FlavorText>>,
    pub(crate) call_count: AtomicU64,
}

impl Default for State {
    fn default() -> Self {
        let mut pokemons_translations = HashMap::new();
        pokemons_translations.insert(
            String::from("pikachu"),
            vec![
                FlavorText {
                    flavor_text: "When several of these Pokémon gather, their electricity could build and cause lightning storms.".to_string(),
                    language: Language::English,
                },
                FlavorText {
                    flavor_text: "Quando vari Pokémon di questo tipo si radunano, la loro energia può causare forti tempeste.".to_string(),
                    language: Language::Italian,
                },
                FlavorText {

                    flavor_text: "Cuando varios de estos Pokémon se juntan, su energía puede causar fuertes tormentas.".to_string(),
                    language: Language::Spanish,
                },
                FlavorText {
                    flavor_text: "ほっぺたの りょうがわに ちいさい でんきぶくろを もつ。ピンチのときに ほうでんする。".to_string(),
                    language: Language::Japanese,
                },
            ]
        );
        pokemons_translations.insert(
            String::from("raichu"),
            vec![
                FlavorText {
                    flavor_text: "When electricity builds up inside its body, it becomes feisty. It also glows in the dark.".to_string(),
                    language: Language::English,
                },
                FlavorText {
                    flavor_text: "Quando l'elettricità si accumula all'interno del suo corpo, diventa irascibile. Si illumina anche al buio.".to_string(),
                    language: Language::Italian,
                },
            ],
        );
        Self {
            pokemons_translations,
            call_count: Default::default(),
        }
    }
}
