mod handlers;
mod models;
mod services;

use anyhow::Result;
use axum::{
    routing::{delete, get, post, put},
    Router,
};
use clap::{Parser, Subcommand};
use std::path::PathBuf;
use tower_http::cors::CorsLayer;
use tower_http::services::ServeDir;
use uuid::Uuid;

use services::{FileScanner, NarrativeStore};

#[derive(Parser)]
#[command(name = "weaver")]
#[command(about = "Context Weaver - Narrative context management tool")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Start the web UI server
    Serve {
        #[arg(short, long, default_value = "3000")]
        port: u16,

        #[arg(short = 'P', long, default_value = ".")]
        path: PathBuf,
    },

    /// Resolve and output a narrative context
    Resolve {
        /// The narrative ID to resolve
        id: Uuid,

        #[arg(short = 'P', long, default_value = ".")]
        path: PathBuf,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();

    match cli.command {
        Commands::Serve { port, path } => {
            serve(port, path).await?;
        }
        Commands::Resolve { id, path } => {
            resolve(id, path).await?;
        }
    }

    Ok(())
}

async fn serve(port: u16, path: PathBuf) -> Result<()> {
    let scanner = FileScanner::new(path.clone());
    scanner.scan()?;

    let store = NarrativeStore::new(path.clone());

    let app = Router::new()
        .route("/api/files", get(handlers::list_files))
        .with_state(scanner.clone())
        .route("/api/narratives", get(handlers::list_narratives))
        .route("/api/narratives", post(handlers::create_narrative))
        .route("/api/narratives/:id", get(handlers::get_narrative))
        .route("/api/narratives/:id", put(handlers::update_narrative))
        .route("/api/narratives/:id", delete(handlers::delete_narrative))
        .with_state(store.clone())
        .route(
            "/api/narratives/:id/resolve",
            get(handlers::resolve_narrative),
        )
        .with_state((store, scanner))
        .nest_service("/", ServeDir::new("cli-tools/context-weaver/static"))
        .layer(CorsLayer::permissive());

    let addr = format!("0.0.0.0:{port}");
    tracing::info!("Server running at http://{}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn resolve(id: Uuid, path: PathBuf) -> Result<()> {
    let scanner = FileScanner::new(path.clone());
    scanner.scan()?;

    let store = NarrativeStore::new(path.clone());

    match store.get(&id) {
        Some(narrative) => {
            let content = scanner.resolve_includes(&narrative)?;
            print!("{content}");
            Ok(())
        }
        None => anyhow::bail!("Narrative with ID {} not found", id),
    }
}
