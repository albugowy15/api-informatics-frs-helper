use std::{collections::HashMap, sync::Arc};

use axum::extract::{Path, Query, State};

use super::{DataResponse, ErrorViews, IntoJson};
use crate::{
    model::{
        class_model::CompactClass,
        course_model::{CourseWithClass, CourseWithLecturer},
        lecturer_model::Lecturer,
    },
    repository::{
        class_repository::ClassRepository, course_repository::CourseRepository,
        lecturer_repository::LecturerRepository,
    },
    route::AppState,
};

use super::{JsonResponse, RouteHandler};

pub async fn courses(
    State(state): State<Arc<AppState>>,
    Query(params): Query<HashMap<String, String>>,
) -> RouteHandler<JsonResponse> {
    if params
        .get("sks")
        .map_or(false, |s| s.parse::<i8>().is_err())
    {
        return Err(ErrorViews::BadRequest("sks wajib bertipe integer".into()).into());
    }

    if params
        .get("semester")
        .map_or(false, |s| s.parse::<i8>().is_err())
    {
        return Err(ErrorViews::BadRequest("semester wajib bertipe integer".into()).into());
    }

    let course_repo = CourseRepository::new(&state.db_pool);
    let courses = course_repo
        .get_courses_with_filter(&params)
        .await
        .map_err(|_| ErrorViews::Internal)?;

    Ok(DataResponse::new(courses.len(), courses).into_json())
}

pub async fn course_by_id(
    State(state): State<Arc<AppState>>,
    Path(id_matkul): Path<String>,
) -> RouteHandler<JsonResponse> {
    let course_repo = CourseRepository::new(&state.db_pool);
    let course = course_repo
        .get_course_by_id(&id_matkul)
        .await
        .map_err(|err| match err {
            sqlx::Error::RowNotFound => ErrorViews::NotFound("Matkul"),
            _ => ErrorViews::Internal,
        })?;
    Ok(course.into_json())
}

pub async fn courses_with_lecturers(
    State(state): State<Arc<AppState>>,
) -> RouteHandler<JsonResponse> {
    let course_repo = CourseRepository::new(&state.db_pool);
    let courses_lecturers = course_repo
        .get_courses_with_lecturers()
        .await
        .map_err(|_| ErrorViews::Internal)?;
    Ok(DataResponse::new(courses_lecturers.len(), courses_lecturers).into_json())
}

pub async fn courses_with_classes(
    State(state): State<Arc<AppState>>,
) -> RouteHandler<JsonResponse> {
    let course_repo = CourseRepository::new(&state.db_pool);
    let courses_classes = course_repo
        .get_courses_with_classes()
        .await
        .map_err(|_| ErrorViews::Internal)?;
    Ok(DataResponse::new(courses_classes.len(), courses_classes).into_json())
}

pub async fn course_by_id_with_classes(
    State(state): State<Arc<AppState>>,
    Path(id_matkul): Path<String>,
) -> RouteHandler<JsonResponse> {
    let course_repo = CourseRepository::new(&state.db_pool);
    let course = course_repo
        .get_course_by_id(&id_matkul)
        .await
        .map_err(|_| ErrorViews::Internal)?;
    let class_repo = ClassRepository::new(&state.db_pool);
    let classes = class_repo
        .get_classes_by_course_id(&course.id)
        .await
        .unwrap_or_default();
    let response = CourseWithClass::<Vec<CompactClass>> {
        id: course.id,
        matkul: course.matkul,
        semester: course.semester,
        sks: course.sks,
        kelas: classes,
    };
    Ok(response.into_json())
}

pub async fn course_by_id_with_lecturers(
    State(state): State<Arc<AppState>>,
    Path(id_matkul): Path<String>,
) -> RouteHandler<JsonResponse> {
    let course_repo = CourseRepository::new(&state.db_pool);
    let course = course_repo
        .get_course_by_id(&id_matkul)
        .await
        .map_err(|_| ErrorViews::Internal)?;
    let lecturer_repo = LecturerRepository::new(&state.db_pool);
    let lecturers = lecturer_repo
        .get_lecturers_by_course_id(&course.id)
        .await
        .unwrap_or_default();
    let response = CourseWithLecturer::<Vec<Lecturer>> {
        id: course.id,
        matkul: course.matkul,
        semester: course.semester,
        sks: course.sks,
        dosen: lecturers,
    };
    Ok(response.into_json())
}
