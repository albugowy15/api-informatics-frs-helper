use std::{collections::HashMap, sync::Arc};

use axum::{
    extract::{Path, Query, State},
    Json,
};
use serde_json::json;

use crate::{repository, route::AppState};

use super::{display_err, ErrorViews, JsonResponse, RouteHandler, SuccessResponse};

pub async fn classes(
    State(app_state): State<Arc<AppState>>,
    Query(params): Query<HashMap<String, String>>,
) -> RouteHandler<JsonResponse> {
    let class_repo = repository::class_repository::ClassRepository::new(&app_state.db_pool);
    let mut classes = match class_repo.get_classes().await {
        Ok(classes) => classes,
        _ => return Err(display_err(ErrorViews::Internal)),
    };

    if let Some(day_param) = params.get("hari") {
        classes.retain(|x| x.hari.to_lowercase() == *day_param.to_lowercase());
    }
    if let Some(hour_param) = params.get("jam") {
        classes.retain(|x| x.jam == *hour_param);
    }
    if let Some(course_name_param) = params.get("matkul") {
        classes.retain(|x| x.matkul.to_lowercase() == *course_name_param.to_lowercase());
    }
    if let Some(lecturer_code_param) = params.get("kode_dosen") {
        classes.retain(|x| x.kode_dosen.to_lowercase() == *lecturer_code_param.to_lowercase());
    }
    let response = SuccessResponse {
        total_results: classes.len(),
        data: classes,
    };
    Ok(Json(json!(response)))
}

pub async fn class_by_id(
    State(app_state): State<Arc<AppState>>,
    Path(id_kelas): Path<String>,
) -> RouteHandler<JsonResponse> {
    let class_repo = repository::class_repository::ClassRepository::new(&app_state.db_pool);
    let class = match class_repo.get_class_by_id(&id_kelas).await {
        Ok(class) => class,
        Err(err) => match err {
            sqlx::Error::RowNotFound => return Err(display_err(ErrorViews::NotFound("Kelas"))),
            _ => {
                return Err(display_err(ErrorViews::Internal));
            }
        },
    };
    Ok(Json(json!(class)))
}
