use serde::{de::DeserializeOwned, Deserialize, Serialize};
use sqlx::Row;

use super::FromRow;
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
    fn from_row(row: &sqlx::mysql::MySqlRow) -> Self {
        Self {
            id: row.get("id"),
            matkul: row.get("matkul"),
            semester: row.get("semester"),
            sks: row.get("sks"),
        }
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
    fn from_row(row: &sqlx::mysql::MySqlRow) -> Self {
        Self {
            id: row.get("id"),
            matkul: row.get("matkul"),
            semester: row.get("semester"),
            sks: row.get("sks"),
            dosen: serde_json::from_str(row.get("dosen")).unwrap_or_default(),
        }
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
    fn from_row(row: &sqlx::mysql::MySqlRow) -> Self {
        Self {
            id: row.get("id"),
            matkul: row.get("matkul"),
            semester: row.get("semester"),
            sks: row.get("sks"),
            kelas: serde_json::from_str(row.get("kelas")).unwrap_or_default(),
        }
    }
}
