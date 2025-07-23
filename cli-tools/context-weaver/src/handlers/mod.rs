use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use uuid::Uuid;

use crate::{
    models::{FileInfo, NarrativeData},
    services::{FileScanner, NarrativeStore},
};

pub async fn list_files(State(scanner): State<FileScanner>) -> impl IntoResponse {
    let files: Vec<FileInfo> = scanner
        .get_file_map()
        .iter()
        .map(|entry| entry.value().clone())
        .collect();

    Json(files)
}

pub async fn create_narrative(
    State(store): State<NarrativeStore>,
    Json(narrative): Json<NarrativeData>,
) -> Result<impl IntoResponse, StatusCode> {
    match store.create(narrative) {
        Ok(created) => Ok(Json(created)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn get_narrative(
    State(store): State<NarrativeStore>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, StatusCode> {
    match store.get(&id) {
        Some(narrative) => Ok(Json(narrative)),
        None => Err(StatusCode::NOT_FOUND),
    }
}

pub async fn update_narrative(
    State(store): State<NarrativeStore>,
    Path(id): Path<Uuid>,
    Json(narrative): Json<NarrativeData>,
) -> Result<impl IntoResponse, StatusCode> {
    match store.update(&id, narrative) {
        Ok(updated) => Ok(Json(updated)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn delete_narrative(
    State(store): State<NarrativeStore>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, StatusCode> {
    match store.delete(&id) {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn list_narratives(State(store): State<NarrativeStore>) -> impl IntoResponse {
    Json(store.list())
}

pub async fn resolve_narrative(
    State((store, scanner)): State<(NarrativeStore, FileScanner)>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, StatusCode> {
    let narrative = match store.get(&id) {
        Some(n) => n,
        None => return Err(StatusCode::NOT_FOUND),
    };

    match scanner.resolve_includes(&narrative) {
        Ok(content) => Ok(content),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
