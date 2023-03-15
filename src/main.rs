extern crate clap;

use axum::routing::get;
use axum::Json;
use axum::{extract::Path, http::StatusCode, response::IntoResponse, routing::get_service, Router};
use axum::{
    extract::State,
    http::Request,
    middleware::{self, Next},
    response::Response,
};
use clap::Parser;
use http::HeaderValue;
use std::env;
use std::fs;
use std::sync::Arc;
use std::{io, net::SocketAddr};
use tower_http::cors::{AllowOrigin, CorsLayer};
use tower_http::services::ServeDir;

const VERSION: &str = "0.3.1";
const PREFIX: &str = "\x1b[93m[server]\x1b[0m";
const OPTSET_OUTPUT: &str = "OUTPUT";
const OPTSET_DEBUGGING: &str = "DEBUGGING";
const OPTSET_BEHAVIOUR: &str = "BEHAVIOUR";

async fn handle_error(_err: io::Error) -> impl IntoResponse {
    (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong...")
}

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

    /// Route
    #[clap(
        long = "route",
        value_parser,
        default_value = "",
        help_heading = OPTSET_BEHAVIOUR,
    )]
    pub route: String,

    /// Json
    #[clap(
        long = "json",
        value_parser,
        default_value = "",
        help_heading = OPTSET_BEHAVIOUR,
    )]
    pub json: String,

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

async fn json_handler_with_param(
    // TODO: Improve into param iter
    Path(param): Path<String>,
    // TODO: Move to hashmap
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    // Example
    // String::from("{\"data\":{\"userId\":\"$1\",\"givenName\":\"Raphael\",\"country\":\"br\"}}");
    let input = state.json_data.replace("{!0}", &param);
    let json_value: serde_json::Value = serde_json::from_str(&input).unwrap_or_default();
    (StatusCode::OK, Json(json_value))
}

async fn json_handler(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let json_value: serde_json::Value = serde_json::from_str(&state.json_data).unwrap_or_default();
    (StatusCode::OK, Json(json_value))
}

struct AppState {
    json_data: String,
}

#[tokio::main]
async fn main() {
    let mut server_path: String = env::current_dir()
        .unwrap()
        .into_os_string()
        .into_string()
        .unwrap();

    let args = Cli::parse();

    let Action::Server(command) = args.command;

    let port = &command.port;
    let path = &command.path;
    let quiet = &command.quiet;
    let open = &command.open;
    let version = &command.version;
    let route = &command.route;
    let json = &command.json;

    if *version {
        println!("{VERSION}");
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

    let addr = SocketAddr::from(([127, 0, 0, 1], *port));

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
        println!("{PREFIX} path: {server_path}");

        if !files_str.contains("index.html") {
            println!("{PREFIX} hint: consider to add an 'index.html' file");
        }

        println!("{PREFIX} listening on: {addr}");
    }

    if open == &true {
        let url: String = format!("http://{addr}");
        match open::that(&url) {
            Ok(()) => {
                if !*quiet {
                    println!("{PREFIX} opened '{url}' successfully on browser.")
                }
            }
            Err(err) => {
                if !*quiet {
                    eprintln!("{PREFIX} an error occurred when opening {url} on browser: {err}")
                }
            }
        }
    }

    if route.is_empty() && json.is_empty() {
        let app = Router::new()
            .fallback(get_service(ServeDir::new(&server_path)).handle_error(handle_error))
            .layer(middleware::from_fn(propagate_custom_headers))
            .layer(cors);

        axum::Server::bind(&addr)
            .serve(app.into_make_service())
            .with_graceful_shutdown(shutdown_signal())
            .await
            .unwrap();
    } else {
        let shared_state = Arc::new(AppState {
            json_data: json.to_string(),
        });
        let app = if json.contains("{!0}") {
            Router::new()
                .route(route, get(json_handler_with_param))
                .layer(cors)
                .with_state(shared_state)
        } else {
            Router::new()
                .route(route, get(json_handler))
                .layer(cors)
                .with_state(shared_state)
        };

        axum::Server::bind(&addr)
            .serve(app.into_make_service())
            .with_graceful_shutdown(shutdown_signal())
            .await
            .unwrap();
    };
}

async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("Expect shutdown CTRL+C handler");
    };

    #[cfg(unix)] /* conditional compilation depending on target family = unix */ let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("Expected shutdown signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}
