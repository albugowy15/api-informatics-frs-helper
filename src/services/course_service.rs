use std::{collections::HashMap, sync::Arc};

use axum::extract::{Path, Query, State};

use super::{DataResponse, ErrorViews, IntoJson};
use crate::{
    model::{
        class_model::CompactClass,
        course_model::{Course, CourseWithClass, CourseWithLecturer},
        lecturer_model::Lecturer,
    },
    repository::{
        class_repository::ClassRepository, course_repository::CourseRepository,
        lecturer_repository::LecturerRepository,
    },
    route::AppState,
};

use super::{display_err, JsonResponse, RouteHandler};

fn filter_courses<'a>(
    params: &'a HashMap<String, String>,
    courses: &'a mut Vec<Course>,
) -> Result<(), ErrorViews<'a>> {
    let course_name_param = params.get("nama").map(|s| s.to_lowercase());
    let semester_param = match params.get("semester") {
        Some(s) => match s.parse::<i8>() {
            Ok(num) => Some(num),
            _ => {
                return Err(ErrorViews::BadRequest(String::from(
                    "Semester wajib bertipe integer",
                )))
            }
        },
        None => None,
    };
    let sks_param = match params.get("sks") {
        Some(s) => match s.parse::<i8>() {
            Ok(num) => Some(num),
            _ => {
                return Err(ErrorViews::BadRequest(String::from(
                    "sks wajib bertipe integer",
                )))
            }
        },
        None => None,
    };
    courses.retain(|course| {
        let matches_course_name = course_name_param.as_ref().map_or(true, |course_name| {
            course.matkul.to_lowercase().contains(course_name)
        });
        let matches_semester = semester_param
            .as_ref()
            .map_or(true, |semester| course.semester == *semester);
        let matches_sks = sks_param.as_ref().map_or(true, |sks| course.sks == *sks);
        matches_course_name && matches_semester && matches_sks
    });
    Ok(())
}

pub async fn courses(
    State(state): State<Arc<AppState>>,
    Query(params): Query<HashMap<String, String>>,
) -> RouteHandler<JsonResponse> {
    let course_repo = CourseRepository::new(&state.db_pool);
    let mut courses = match course_repo.get_courses().await {
        Ok(courses) => courses,
        _ => {
            return Err(display_err(ErrorViews::Internal));
        }
    };
    if let Err(err) = filter_courses(&params, &mut courses) {
        return Err(display_err(err));
    }

    Ok(DataResponse::new(courses.len(), courses).into_json())
}

pub async fn course_by_id(
    State(state): State<Arc<AppState>>,
    Path(id_matkul): Path<String>,
) -> RouteHandler<JsonResponse> {
    let course_repo = CourseRepository::new(&state.db_pool);
    let course = match course_repo.get_course_by_id(&id_matkul).await {
        Ok(course) => course,
        Err(err) => match err {
            sqlx::Error::RowNotFound => return Err(display_err(ErrorViews::NotFound("Matkul"))),
            _ => return Err(display_err(ErrorViews::Internal)),
        },
    };
    Ok(course.into_json())
}

pub async fn courses_with_lecturers(
    State(state): State<Arc<AppState>>,
) -> RouteHandler<JsonResponse> {
    let course_repo = CourseRepository::new(&state.db_pool);
    let courses_lecturers = match course_repo.get_courses_with_lecturers().await {
        Ok(res) => res,
        _ => return Err(display_err(ErrorViews::Internal)),
    };
    Ok(DataResponse::new(courses_lecturers.len(), courses_lecturers).into_json())
}

pub async fn courses_with_classes(
    State(state): State<Arc<AppState>>,
) -> RouteHandler<JsonResponse> {
    let course_repo = CourseRepository::new(&state.db_pool);
    let courses_classes = match course_repo.get_courses_with_classes().await {
        Ok(res) => res,
        _ => return Err(display_err(ErrorViews::Internal)),
    };
    Ok(DataResponse::new(courses_classes.len(), courses_classes).into_json())
}

pub async fn course_by_id_with_classes(
    State(state): State<Arc<AppState>>,
    Path(id_matkul): Path<String>,
) -> RouteHandler<JsonResponse> {
    let course_repo = CourseRepository::new(&state.db_pool);
    let course = match course_repo.get_course_by_id(&id_matkul).await {
        Ok(res) => res,
        _ => return Err(display_err(ErrorViews::Internal)),
    };
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
    let course = match course_repo.get_course_by_id(&id_matkul).await {
        Ok(res) => res,
        _ => return Err(display_err(ErrorViews::Internal)),
    };
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
