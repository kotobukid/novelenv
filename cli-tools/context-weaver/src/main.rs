mod handlers;
mod models;
mod services;

use anyhow::Result;
use axum::{
    routing::{delete, get, post, put},
    Router,
};
use clap::{Parser, Subcommand};
use std::env;
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

fn get_static_dir() -> PathBuf {
    // 実行ファイルの場所を取得
    let current_exe = env::current_exe()
        .expect("Failed to get current executable path");
    
    let exe_dir = current_exe.parent()
        .expect("Failed to get executable directory");
    
    // まず同じディレクトリ内でstaticフォルダを探す（インストール済み環境）
    let installed_static = exe_dir.join("static");
    if installed_static.exists() {
        return installed_static;
    }
    
    // 開発環境のパスを試す
    if let Some(cli_tools_dir) = exe_dir
        .parent() // target
        .and_then(|p| p.parent()) // release
        .and_then(|p| p.parent()) // context-weaver
    {
        let dev_static = cli_tools_dir
            .join("context-weaver")
            .join("static");
        if dev_static.exists() {
            return dev_static;
        }
    }
    
    // フォールバック: 相対パス
    PathBuf::from("cli-tools/context-weaver/static")
}

async fn serve(port: u16, path: PathBuf) -> Result<()> {
    let scanner = FileScanner::new(path.clone());
    scanner.scan()?;

    let store = NarrativeStore::new(path.clone());
    
    let static_dir = get_static_dir();
    tracing::info!("Static directory: {}", static_dir.display());

    let app = Router::new()
        .route("/api/files", get(handlers::list_files))
        .route("/api/files/refresh", post(handlers::refresh_files))
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
        .nest_service("/", ServeDir::new(static_dir))
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
