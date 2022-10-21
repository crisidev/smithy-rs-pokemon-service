use std::fs::File;
use std::future;
use std::io::BufReader;
use std::net::SocketAddr;
use std::sync::Arc;

use aws_smithy_http_server::{routing::Router, AddExtensionLayer};
use clap::Parser;
use futures_util::stream::StreamExt;
use pokemon_service::{get_pokemon_species, setup_tracing, State};
use pokemon_service_server_sdk::operation_registry::OperationRegistryBuilder;
use tokio_rustls::{
    rustls::{Certificate, PrivateKey, ServerConfig},
    TlsAcceptor,
};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Hyper server bind address.
    #[clap(short, long, action, default_value = "127.0.0.1")]
    address: String,
    /// Hyper server bind port.
    #[clap(short, long, action, default_value = "13734")]
    port: u16,
    /// Hyper server TLS certificate path. Must be a PEM file.
    #[clap(long, default_value = "certs/localhost.pem")]
    tls_cert_path: String,
    /// Hyper server TLS private key path. Must be a PEM file.
    #[clap(long, default_value = "certs/localhost-key.pem")]
    tls_key_path: String,
}

#[tokio::main]
pub async fn main() {
    let args = Args::parse();
    setup_tracing();
    let app: Router = OperationRegistryBuilder::default()
        // Build a registry containing implementations to all the operations in the service. These
        // are async functions or async closures that take as input the operation's input and
        // return the operation's output.
        .get_pokemon_species(get_pokemon_species)
        .build()
        .expect("Unable to build operation registry")
        // Convert it into a router that will route requests to the matching operation
        // implementation.
        .into();

    // Setup shared state and middlewares.
    let shared_state = Arc::new(State::default());
    let app = app.layer(AddExtensionLayer::new(shared_state));

    let addr: SocketAddr = format!("{}:{}", args.address, args.port)
        .parse()
        .expect("unable to parse the server bind address and port");

    let acceptor = acceptor(&args.tls_cert_path, &args.tls_key_path);
    let listener = tls_listener::TlsListener::new(
        acceptor,
        hyper::server::conn::AddrIncoming::bind(&addr).expect("could not bind"),
    )
    .filter(|conn| {
        if let Err(err) = conn {
            eprintln!("connection error: {:?}", err);
            future::ready(false)
        } else {
            future::ready(true)
        }
    });
    let server = hyper::Server::builder(hyper::server::accept::from_stream(listener))
        .serve(app.into_make_service());
    if let Err(err) = server.await {
        eprintln!("server error: {}", err);
    }
}

// Returns a `TlsAcceptor` that can be used to create `TlsListener`
// which then can be used with Hyper.
pub fn acceptor(cert_path: &str, key_path: &str) -> TlsAcceptor {
    let certs = load_certs(cert_path);
    let key = load_key(key_path);
    let mut server_config = ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth()
        .with_single_cert(certs, key)
        .expect("could not create server config");

    // If we don't state we are accepting "h2", clients by default don't negotiate way up to http2.
    server_config.alpn_protocols = vec!["h2".into(), "http/1.1".into()];

    TlsAcceptor::from(Arc::new(server_config))
}

fn load_certs(path: &str) -> Vec<Certificate> {
    let mut reader = BufReader::new(File::open(path).expect("could not open certificate"));
    rustls_pemfile::certs(&mut reader)
        .expect("could not parse certificate")
        .into_iter()
        .map(Certificate)
        .collect()
}

fn load_key(path: &str) -> PrivateKey {
    let mut reader = BufReader::new(File::open(path).expect("could not open private key"));
    loop {
        match rustls_pemfile::read_one(&mut reader).expect("could not parse private key") {
            Some(rustls_pemfile::Item::RSAKey(key)) => return PrivateKey(key),
            Some(rustls_pemfile::Item::PKCS8Key(key)) => return PrivateKey(key),
            Some(rustls_pemfile::Item::ECKey(key)) => return PrivateKey(key),
            None => break,
            _ => {}
        }
    }
    panic!("invalid private key")
}
