use serde::{de::DeserializeOwned, Deserialize, Serialize};
use sqlx::Row;

use super::FromRow;
use crate::services::IntoJson;

#[derive(Deserialize, Serialize)]
pub struct Lecturer {
    pub id: String,
    pub kode: String,
    pub nama: String,
}
impl IntoJson for Lecturer {}
impl FromRow for Lecturer {
    fn from_row(row: &sqlx::mysql::MySqlRow) -> Self {
        Self {
            id: row.get("id"),
            kode: row.get("kode"),
            nama: row.get("nama"),
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct LecturerWithClasses<TClasses> {
    pub id: String,
    pub kode: String,
    pub nama: String,
    pub kelas: TClasses,
}
impl<T: Serialize> IntoJson for LecturerWithClasses<T> {}
impl<TData: Default + DeserializeOwned> FromRow for LecturerWithClasses<TData> {
    fn from_row(row: &sqlx::mysql::MySqlRow) -> Self {
        Self {
            id: row.get("id"),
            kode: row.get("kode"),
            nama: row.get("nama"),
            kelas: serde_json::from_str(row.get("kelas")).unwrap_or_default(),
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct LecturerWithCourses<TCourses> {
    pub id: String,
    pub kode: String,
    pub nama: String,
    pub matkul: TCourses,
}
impl<T: Serialize> IntoJson for LecturerWithCourses<T> {}
impl<TData: Default + DeserializeOwned> FromRow for LecturerWithCourses<TData> {
    fn from_row(row: &sqlx::mysql::MySqlRow) -> Self {
        Self {
            id: row.get("id"),
            kode: row.get("kode"),
            nama: row.get("nama"),
            matkul: serde_json::from_str(row.get("matkul")).unwrap_or_default(),
        }
    }
}
