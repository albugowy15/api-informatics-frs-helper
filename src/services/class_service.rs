use std::{collections::HashMap, sync::Arc};

use axum::extract::{Path, Query, State};

use crate::{
    model::response_model::{DataResponse, ErrorViews},
    repository::class_repository::ClassRepository,
    route::AppState,
};

use super::{JsonResponse, RouteHandler};

pub async fn classes(
    State(state): State<Arc<AppState>>,
    Query(params): Query<HashMap<String, String>>,
) -> RouteHandler<JsonResponse> {
    let class_repo = ClassRepository::new(&state.db_pool);
    let classes = class_repo
        .get_classes_with_filter(&params)
        .await
        .map_err(|_| ErrorViews::Internal)?;
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
            sqlx::Error::RowNotFound => ErrorViews::NotFound("Kelas"),
            _ => ErrorViews::Internal,
        })?;
    Ok(class.into())
}
