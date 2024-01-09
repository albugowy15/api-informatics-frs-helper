use std::{collections::HashMap, sync::Arc};

use axum::{
    extract::{Path, Query, State},
    Json,
};
use serde_json::json;

use crate::{repository, route::AppState};

use super::{display_err, ErrorViews, JsonResponse, RouteHandler, SuccessResponse};

pub async fn lecturer_handler(
    State(state): State<Arc<AppState>>,
    Query(params): Query<HashMap<String, String>>,
) -> RouteHandler<JsonResponse> {
    let lecturer_repo = repository::lecturer_repository::LecturerRepository::new(&state.db_pool);
    let mut lecturers = match lecturer_repo.get_lecturers().await {
        Ok(lecturers) => lecturers,
        _ => {
            return Err(display_err(ErrorViews::Internal));
        }
    };
    if let Some(fullname_param) = params.get("nama") {
        lecturers.retain(|lecturer| {
            lecturer
                .nama
                .to_lowercase()
                .contains(&fullname_param.to_lowercase())
        });
    }
    if let Some(code_param) = params.get("kode") {
        lecturers.retain(|lecturer| lecturer.kode.to_lowercase() == *code_param.to_lowercase());
    }
    let response = SuccessResponse {
        total_results: lecturers.len(),
        data: lecturers,
    };
    Ok(Json(json!(response)))
}

pub async fn lecturer_with_id_handler(
    State(state): State<Arc<AppState>>,
    Path(id_dosen): Path<String>,
) -> RouteHandler<JsonResponse> {
    let lecturer_repo = repository::lecturer_repository::LecturerRepository::new(&state.db_pool);
    let lecturer = match lecturer_repo.get_lecturer_by_id(&id_dosen).await {
        Ok(lecturer) => lecturer,
        Err(err) => match err {
            sqlx::Error::RowNotFound => {
                return Err(display_err(ErrorViews::NotFound("Dosen")));
            }
            _ => return Err(display_err(ErrorViews::Internal)),
        },
    };
    Ok(Json(json!(lecturer)))
}
