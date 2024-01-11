use serde::{de::DeserializeOwned, Deserialize, Serialize};
use sqlx::{mysql::MySqlRow, Row};

use super::{FromRow, FromRows};
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
impl FromRows for Vec<Lecturer> {
    fn from_rows(rows: &[MySqlRow]) -> Self {
        let mut lecturers = Vec::with_capacity(rows.len());
        rows.iter().for_each(|row| {
            lecturers.push(Lecturer::from_row(row));
        });
        lecturers
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
impl<TData: Default + DeserializeOwned> FromRows for Vec<LecturerWithClasses<TData>> {
    fn from_rows(rows: &[MySqlRow]) -> Self {
        let mut lecturers = Vec::with_capacity(rows.len());
        rows.iter().for_each(|row| {
            lecturers.push(LecturerWithClasses::from_row(row));
        });
        lecturers
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
impl<TData: Default + DeserializeOwned> FromRows for Vec<LecturerWithCourses<TData>> {
    fn from_rows(rows: &[MySqlRow]) -> Self {
        let mut lecturers = Vec::with_capacity(rows.len());
        rows.iter().for_each(|row| {
            lecturers.push(LecturerWithCourses::from_row(row));
        });
        lecturers
    }
}
