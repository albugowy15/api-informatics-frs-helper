use std::{collections::HashMap, sync::Arc};

use axum::extract::{Path, Query, State};

use crate::{
    repositories::class::ClassRepository, AppError, AppState, DataResponse, JsonResponse,
    RouteHandler,
};

pub async fn classes(
    State(state): State<Arc<AppState>>,
    Query(params): Query<HashMap<String, String>>,
) -> RouteHandler<JsonResponse> {
    let class_repo = ClassRepository::new(&state.db_pool);
    let classes = class_repo
        .get_classes_with_filter(&params)
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(DataResponse::new(classes.len(), classes).into())
}

pub async fn class_by_id(
    State(state): State<Arc<AppState>>,
    Path(id_kelas): Path<String>,
) -> RouteHandler<JsonResponse> {
    let class_repo = ClassRepository::new(&state.db_pool);
    let class = class_repo
        .get_class_by_id(&id_kelas)
        .await
        .map_err(|err| match err {
            sqlx::Error::RowNotFound => AppError::NotFound("Kelas".to_string()),
            _ => AppError::Internal,
        })?;
    Ok(class.into())
}
