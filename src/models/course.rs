use axum::Json;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::{json, Value};
use sqlx::{mysql::MySqlRow, FromRow, Row};

use crate::FromRows;

#[derive(Deserialize, Serialize, sqlx::FromRow)]
pub struct Course {
    pub id: String,
    pub matkul: String,
    pub semester: i8,
    pub sks: i8,
}
impl From<Course> for Json<Value> {
    fn from(value: Course) -> Self {
        Json(json!(value))
    }
}
impl FromRows for Vec<Course> {
    fn from_rows(rows: &[MySqlRow]) -> Result<Self, sqlx::Error> {
        rows.iter().map(Course::from_row).collect()
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
impl<T: Serialize> From<CourseWithLecturer<T>> for Json<Value> {
    fn from(value: CourseWithLecturer<T>) -> Self {
        Json(json!(value))
    }
}
impl<TData: Default + DeserializeOwned> FromRow<'_, MySqlRow> for CourseWithLecturer<TData> {
    fn from_row(row: &MySqlRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            id: row.try_get("id")?,
            matkul: row.try_get("matkul")?,
            semester: row.try_get("semester")?,
            sks: row.try_get("sks")?,
            dosen: serde_json::from_str(row.try_get("dosen")?).unwrap_or_default(),
        })
    }
}
impl<TData: Default + DeserializeOwned> FromRows for Vec<CourseWithLecturer<TData>> {
    fn from_rows(rows: &[MySqlRow]) -> Result<Self, sqlx::Error> {
        rows.iter().map(CourseWithLecturer::from_row).collect()
    }
}

#[derive(Debug, Serialize)]
pub struct CourseWithClass<TClass> {
    pub id: String,
    pub matkul: String,
    pub semester: i8,
    pub sks: i8,
    pub kelas: TClass,
}
impl<T: Serialize> From<CourseWithClass<T>> for Json<Value> {
    fn from(value: CourseWithClass<T>) -> Self {
        Json(json!(value))
    }
}
impl<TData: Default + DeserializeOwned> FromRow<'_, MySqlRow> for CourseWithClass<TData> {
    fn from_row(row: &MySqlRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            id: row.try_get("id")?,
            matkul: row.try_get("matkul")?,
            semester: row.try_get("semester")?,
            sks: row.try_get("sks")?,
            kelas: serde_json::from_str(row.try_get("kelas")?).unwrap_or_default(),
        })
    }
}
impl<TData: Default + DeserializeOwned> FromRows for Vec<CourseWithClass<TData>> {
    fn from_rows(rows: &[MySqlRow]) -> Result<Self, sqlx::Error> {
        rows.iter().map(CourseWithClass::from_row).collect()
    }
}
