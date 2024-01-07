use std::{collections::HashMap, sync::Arc};

use axum::extract::{Query, State};

use crate::{repository, route::AppState};

pub async fn class_handler(
    State(app_state): State<Arc<AppState>>,
    Query(_params): Query<HashMap<String, String>>,
) -> &'static str {
    let _class_repo = repository::class_repository::ClassRepository::new(&app_state.db_pool);
    "Class handler"
}
