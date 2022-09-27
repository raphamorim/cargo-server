extern crate clap;
use axum::{http::StatusCode, response::IntoResponse, routing::get_service, Router};
use clap::Parser;
use std::fs;
use std::{io, net::SocketAddr};
use tower_http::cors::{AllowOrigin, CorsLayer};
use tower_http::services::ServeDir;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Path
    #[clap(default_value_t = String::from("./"))]
    path: String,

    /// Port
    #[clap(short, long, value_parser, default_value_t = 8000)]
    port: u16,

    /// Open
    #[clap(short, long, value_parser, default_value_t = false)]
    open: bool,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let port = &args.port;
    let path = &args.path;
    let open = &args.open;
    let cors = CorsLayer::new()
        .allow_origin(AllowOrigin::predicate(|_, _| true))
        .allow_credentials(true);

    let app: _ = Router::new()
        .fallback(get_service(ServeDir::new(path)).handle_error(handle_error))
        .layer(cors);

    let addr = SocketAddr::from(([127, 0, 0, 1], *port as u16));
    let ou = format!("{}{}{}", "\x1b[93m", "[ou]", "\x1b[0m");

    let files = fs::read_dir(path).unwrap();
    let mut files_str = String::new();
    for file in files {
        files_str = files_str
            + " "
            + &file
                .as_ref()
                .unwrap()
                .path()
                .into_os_string()
                .into_string()
                .ok()
                .unwrap();
    }

    println!("{} files:{}", ou, files_str);

    if !files_str.contains("./index.html") {
        println!("{} consider to add an 'index.html' file", ou);
    }

    println!("{} listening on: {}", ou, addr);

    if open == &true {
        let url: String = format!("http://{}", addr);
        match open::that(url.to_string()) {
            Ok(()) => println!("{} opened '{}' successfully on browser.", ou, url),
            Err(err) => eprintln!(
                "{} an error occurred when opening {} on browser: {}",
                ou, url, err
            ),
        }
    }

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handle_error(_err: io::Error) -> impl IntoResponse {
    (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong...")
}
