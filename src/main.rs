extern crate clap;
use axum::{http::StatusCode, response::IntoResponse, routing::get_service, Router};
use clap::Parser;
use std::{io, net::SocketAddr};
use tower_http::services::ServeDir;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Filepath
    // #[clap(default_value_t = String::from(""))]
    // file: u64,

    /// Scale screen
    #[clap(short, long, value_parser, default_value_t = 8000)]
    port: u16,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let port = &args.port;
    let app: _ =
        Router::new().fallback(get_service(ServeDir::new("./")).handle_error(handle_error));

    let addr = SocketAddr::from(([127, 0, 0, 1], *port as u16));
    let ou = format!("{}{}{}", "\x1b[93m", "[ou]", "\x1b[0m");
    println!("{} listening on {}", ou, addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handle_error(_err: io::Error) -> impl IntoResponse {
    (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong...")
}
