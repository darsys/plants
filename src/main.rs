use axum::{
    extract::Path,
    routing::{get, post},
    Router,
    response::Json,
};
use serde_json::{Value, json};
use tokio::fs;
use crate::error::AppError;
use crate::plant::Plant;
use std::env;
use log::info;
use tower_http::trace::TraceLayer;
use tracing_subscriber::EnvFilter;

mod plant;
mod error;

#[tokio::main]
async fn main() {
    // 1. Initialize tracing + log bridging
    tracing_subscriber::fmt()
        // This allows you to use, e.g., `RUST_LOG=info` or `RUST_LOG=debug`
        // when running the app to set log levels.
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .or_else(|_| EnvFilter::try_new("plants=info,tower_http=warn"))
                .unwrap(),
        )
        .init();

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        .route("/plant", get(plantoftheday))
        .route("/plants/{plantname}", get(get_json_file))
        .route("/plant", post(plant::create_plant_from_json))
        .route("/test", get(test))
        .route("/info", get(info))
        .route("/theid/{id}", get(dotheid))
        .layer(TraceLayer::new_for_http());

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    info!("Handle root request");
    "You have found plants! Welcome!"
}

async fn dotheid(Path(_id): Path<String>) -> &'static str {
    info!("Do the id");
    "You have found plants! Welcome!"
}

async fn info() -> Result<Json<Value>, AppError> {
    let path = env::current_dir()?;
    info!("Handling info request");
    Ok(Json(json!({ "path": [ format!("{}", path.display()) ] })))
}

async fn test() -> Result<Json<Plant>, AppError> {
    let file_path = "/home/damonv/plants/plant_data/echinacea_purpureaprairiemoon.json";
    info!("Handling test request");
    let content = fs::read_to_string(&file_path).await?;
    let plant: Plant = serde_json::from_str(&content)?;
    Ok(Json(plant))
}

async fn plantoftheday() -> Result<Json<Plant>, AppError> {
    let file_path = "echinacea_purpureaprairiemoon.json";
    info!("Handling plantoftheday request");
    let content = fs::read_to_string(&file_path).await?;
    let plant: Plant = serde_json::from_str(&content)?;
    Ok(Json(plant))
}

async fn get_json_file(Path(plantname): Path<String>) -> Result<Json<Value>, AppError> {
    // Sanitize input: prevent path traversal attacks (e.g., "../")
    info!("Handling get_json_file request with id:{}", plantname);
    //if id.contains('/') || id.contains('\\') || id.contains("..") {
    //    return Err(AppError::BadRequest("Invalid input format".into()));
    //}
    // Construct path dynamically (e.g., "data/123.json")
    let file_path = format!("/home/damonv/plants/plant_data/{}.json", plantname);
    // Read the file asynchronously
    // let content = fs::read_to_string(&file_path).await?;

    // Parse string content into serde_json::Value
    // let json: Value = serde_json::from_str(&content)?;

    Ok(Json(json!({ "path": [ format!("{}", file_path) ] })))
}


