extern crate clap;
use axum::{http::StatusCode, response::IntoResponse, routing::get_service, Router};
use clap::Parser;
use std::fs;
use std::{io, net::SocketAddr};
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
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let port = &args.port;
    let path = &args.path;
    let app: _ =
        Router::new().fallback(get_service(ServeDir::new(path)).handle_error(handle_error));

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

    println!("{} listening on {}", ou, addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handle_error(_err: io::Error) -> impl IntoResponse {
    (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong...")
}
