extern crate clap;

use std::env;

use axum::{
    http::Request,
    middleware::{self, Next},
    response::Response,
};
use axum::{http::StatusCode, response::IntoResponse, routing::get_service, Router};
use clap::Parser;
use http::HeaderValue;
use std::fs;
use std::{io, net::SocketAddr};
use tower_http::cors::{AllowOrigin, CorsLayer};
use tower_http::services::ServeDir;

const PREFIX: &str = "\x1b[93m[server]\x1b[0m";
const OPTSET_OUTPUT: &str = "OUTPUT";
const OPTSET_DEBUGGING: &str = "DEBUGGING";
const OPTSET_BEHAVIOUR: &str = "BEHAVIOUR";

#[derive(Parser, Debug)]
#[clap(author, name = "server", bin_name = "cargo", version, about)]
struct Cli {
    #[clap(subcommand)]
    command: Action,
}

#[derive(Debug, clap::Subcommand)]
enum Action {
    Server(Server),
}

#[derive(Debug, Parser)]
struct Server {
    /// Path
    #[clap(
        long,
        required = false,
        value_parser,
        default_value = "",
        help_heading = OPTSET_BEHAVIOUR,
    )]
    pub path: String,

    /// Version
    #[clap(
        short = 'v',
        long = "version",
        value_parser,
        default_value_t = false,
        help_heading = OPTSET_DEBUGGING,
    )]
    pub version: bool,

    /// Port
    #[clap(
        short = 'p',
        long = "port", 
        help_heading = OPTSET_BEHAVIOUR,
        value_parser,
        default_value_t = 8000)
    ]
    pub port: u16,

    /// Open
    #[clap(short = 'o', long = "open", value_parser, default_value_t = false)]
    pub open: bool,

    /// Quiet
    #[clap(
        short = 'q',
        long = "quiet",
        value_parser,
        default_value_t = false,
        help_heading = OPTSET_OUTPUT,
    )]
    pub quiet: bool,
}

async fn propagate_custom_headers<B>(req: Request<B>, next: Next<B>) -> Result<Response, Response> {
    let mut response = next.run(req).await;

    // Support SharedArrayBuffer
    // https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/SharedArrayBuffer#security_requirements
    response.headers_mut().insert(
        "Cross-Origin-Opener-Policy",
        HeaderValue::from_static("same-origin"),
    );

    response.headers_mut().insert(
        "Cross-Origin-Embedder-Policy",
        HeaderValue::from_static("require-corp"),
    );

    Ok(response)
}

#[tokio::main]
async fn main() {
    let mut server_path: String = env::current_dir()
        .unwrap()
        .into_os_string()
        .into_string()
        .unwrap();

    let args = Cli::parse();

    let command = match args.command {
        Action::Server(command) => command,
    };

    let port = &command.port;
    let path = &command.path;
    let quiet = &command.quiet;
    let open = &command.open;
    let version = &command.version;

    if *version {
        println!("0.2.0");
        return;
    }

    let cors = CorsLayer::new()
        .allow_credentials(true)
        .allow_headers(vec![
            http::header::CONTENT_TYPE,
            http::header::ORIGIN,
            http::header::ACCEPT,
            http::header::ACCESS_CONTROL_REQUEST_HEADERS,
            http::header::ACCESS_CONTROL_REQUEST_METHOD,
            http::header::ACCESS_CONTROL_ALLOW_HEADERS,
            http::header::AUTHORIZATION,
        ])
        .allow_origin(AllowOrigin::predicate(|_, _| true));

    if !path.is_empty() {
        server_path = path.to_string();
    }

    let app: _ = Router::new()
        .fallback(get_service(ServeDir::new(&server_path)).handle_error(handle_error))
        .layer(middleware::from_fn(propagate_custom_headers))
        .layer(cors);

    let addr = SocketAddr::from(([127, 0, 0, 1], *port as u16));

    let files = fs::read_dir(&server_path).unwrap();
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

    if !*quiet {
        println!("{} path: {}", PREFIX, server_path);

        if !files_str.contains("index.html") {
            println!("{} hint: consider to add an 'index.html' file", PREFIX);
        }

        println!("{} listening on: {}", PREFIX, addr);
    }

    if open == &true {
        let url: String = format!("http://{}", addr);
        match open::that(&url) {
            Ok(()) => {
                if !*quiet {
                    println!("{} opened '{}' successfully on browser.", PREFIX, url)
                }
            }
            Err(err) => {
                if !*quiet {
                    eprintln!(
                        "{} an error occurred when opening {} on browser: {}",
                        PREFIX, url, err
                    )
                }
            }
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
