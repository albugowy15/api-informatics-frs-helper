use std::{collections::HashMap, sync::Arc};

use axum::extract::{Path, Query, State};

use crate::{
    model::class_model::Class, repository::class_repository::ClassRepository, route::AppState,
};

use super::{display_err, DataResponse, ErrorViews, IntoJson, JsonResponse, RouteHandler};

fn filter_classes(params: &HashMap<String, String>, classes: &mut Vec<Class>) {
    let day_param = params.get("hari").map(|s| s.to_lowercase());
    let hour_param = params.get("jam").map(|s| s.to_string());
    let course_param = params.get("matkul").map(|s| s.to_lowercase());
    let lecturer_code_param = params.get("kode_dosen").map(|s| s.to_lowercase());
    classes.retain(|class| {
        let matches_day = day_param
            .as_ref()
            .map_or(true, |day| class.hari.to_lowercase() == *day);
        let matches_hour = hour_param.as_ref().map_or(true, |hour| class.jam == *hour);
        let matches_course = course_param
            .as_ref()
            .map_or(true, |course| class.matkul.to_lowercase().contains(course));
        let matches_lecturer_code = lecturer_code_param.as_ref().map_or(true, |lecturer| {
            class.kode_dosen.to_lowercase() == *lecturer
        });
        matches_day && matches_hour && matches_course && matches_lecturer_code
    });
}

pub async fn classes(
    State(state): State<Arc<AppState>>,
    Query(params): Query<HashMap<String, String>>,
) -> RouteHandler<JsonResponse> {
    let class_repo = ClassRepository::new(&state.db_pool);
    let mut classes = match class_repo.get_classes().await {
        Ok(classes) => classes,
        _ => return Err(display_err(ErrorViews::Internal)),
    };
    filter_classes(&params, &mut classes);
    Ok(DataResponse::new(classes.len(), classes).into_json())
}

pub async fn class_by_id(
    State(state): State<Arc<AppState>>,
    Path(id_kelas): Path<String>,
) -> RouteHandler<JsonResponse> {
    let class_repo = ClassRepository::new(&state.db_pool);
    let class = match class_repo.get_class_by_id(&id_kelas).await {
        Ok(class) => class,
        Err(err) => match err {
            sqlx::Error::RowNotFound => return Err(display_err(ErrorViews::NotFound("Kelas"))),
            _ => {
                return Err(display_err(ErrorViews::Internal));
            }
        },
    };
    Ok(class.into_json())
}
