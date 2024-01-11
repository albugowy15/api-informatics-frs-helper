use serde::{de::DeserializeOwned, Deserialize, Serialize};
use sqlx::{mysql::MySqlRow, Row};

use super::{FromRow, FromRows};
use crate::services::IntoJson;

#[derive(Deserialize, Serialize)]
pub struct Course {
    pub id: String,
    pub matkul: String,
    pub semester: i8,
    pub sks: i8,
}
impl IntoJson for Course {}
impl FromRow for Course {
    fn from_row(row: &MySqlRow) -> Self {
        Self {
            id: row.get("id"),
            matkul: row.get("matkul"),
            semester: row.get("semester"),
            sks: row.get("sks"),
        }
    }
}
impl FromRows for Vec<Course> {
    fn from_rows(rows: &[MySqlRow]) -> Self {
        let mut courses = Vec::with_capacity(rows.len());
        rows.iter()
            .for_each(|row| courses.push(Course::from_row(row)));
        courses
    }
}

#[derive(Serialize)]
pub struct CourseWithLecturer<TLecturer> {
    pub id: String,
    pub matkul: String,
    pub semester: i8,
    pub sks: i8,
    pub dosen: TLecturer,
}
impl<T: Serialize> IntoJson for CourseWithLecturer<T> {}
impl<TData: Default + DeserializeOwned> FromRow for CourseWithLecturer<TData> {
    fn from_row(row: &MySqlRow) -> Self {
        Self {
            id: row.get("id"),
            matkul: row.get("matkul"),
            semester: row.get("semester"),
            sks: row.get("sks"),
            dosen: serde_json::from_str(row.get("dosen")).unwrap_or_default(),
        }
    }
}
impl<TData: Default + DeserializeOwned> FromRows for Vec<CourseWithLecturer<TData>> {
    fn from_rows(rows: &[MySqlRow]) -> Self {
        let mut courses_lecturers = Vec::with_capacity(rows.len());
        rows.iter()
            .for_each(|row| courses_lecturers.push(CourseWithLecturer::from_row(row)));
        courses_lecturers
    }
}

#[derive(Serialize)]
pub struct CourseWithClass<TClass> {
    pub id: String,
    pub matkul: String,
    pub semester: i8,
    pub sks: i8,
    pub kelas: TClass,
}
impl<T: Serialize> IntoJson for CourseWithClass<T> {}
impl<TData: Default + DeserializeOwned> FromRow for CourseWithClass<TData> {
    fn from_row(row: &MySqlRow) -> Self {
        Self {
            id: row.get("id"),
            matkul: row.get("matkul"),
            semester: row.get("semester"),
            sks: row.get("sks"),
            kelas: serde_json::from_str(row.get("kelas")).unwrap_or_default(),
        }
    }
}
impl<TData: Default + DeserializeOwned> FromRows for Vec<CourseWithClass<TData>> {
    fn from_rows(rows: &[MySqlRow]) -> Self {
        let mut courses_lecturers = Vec::with_capacity(rows.len());
        rows.iter()
            .for_each(|row| courses_lecturers.push(CourseWithClass::from_row(row)));
        courses_lecturers
    }
}
