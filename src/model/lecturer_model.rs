use axum::Json;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::{json, Value};
use sqlx::{mysql::MySqlRow, FromRow, Row};

use super::FromRows;

#[derive(Deserialize, Serialize, sqlx::FromRow)]
pub struct Lecturer {
    pub id: String,
    pub kode: String,
    pub nama: String,
}
impl From<Lecturer> for Json<Value> {
    fn from(value: Lecturer) -> Self {
        Json(json!(value))
    }
}
impl FromRows for Vec<Lecturer> {
    fn from_rows(rows: &[MySqlRow]) -> Result<Self, sqlx::Error> {
        rows.iter().map(Lecturer::from_row).collect()
    }
}

#[derive(Deserialize, Serialize)]
pub struct LecturerWithClasses<TClasses> {
    pub id: String,
    pub kode: String,
    pub nama: String,
    pub kelas: TClasses,
}
impl<T: Serialize> From<LecturerWithClasses<T>> for Json<Value> {
    fn from(value: LecturerWithClasses<T>) -> Self {
        Json(json!(value))
    }
}
impl<'a, TData: Default + DeserializeOwned> FromRow<'a, MySqlRow> for LecturerWithClasses<TData> {
    fn from_row(row: &MySqlRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            id: row.try_get("id")?,
            kode: row.try_get("kode")?,
            nama: row.try_get("nama")?,
            kelas: serde_json::from_str(row.try_get("kelas")?).map_err(|err| {
                sqlx::Error::ColumnDecode {
                    index: "kelas".into(),
                    source: Box::new(err),
                }
            })?,
        })
    }
}
impl<TData: Default + DeserializeOwned> FromRows for Vec<LecturerWithClasses<TData>> {
    fn from_rows(rows: &[MySqlRow]) -> Result<Self, sqlx::Error> {
        rows.iter().map(LecturerWithClasses::from_row).collect()
    }
}

#[derive(Deserialize, Serialize)]
pub struct LecturerWithCourses<TCourses> {
    pub id: String,
    pub kode: String,
    pub nama: String,
    pub matkul: TCourses,
}
impl<T: Serialize> From<LecturerWithCourses<T>> for Json<Value> {
    fn from(value: LecturerWithCourses<T>) -> Self {
        Json(json!(value))
    }
}
impl<'a, TData: Default + DeserializeOwned> FromRow<'a, MySqlRow> for LecturerWithCourses<TData> {
    fn from_row(row: &MySqlRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            id: row.try_get("id")?,
            kode: row.try_get("kode")?,
            nama: row.try_get("nama")?,
            matkul: serde_json::from_str(row.try_get("matkul")?).map_err(|err| {
                sqlx::Error::ColumnDecode {
                    index: "matkul".into(),
                    source: Box::new(err),
                }
            })?,
        })
    }
}
impl<TData: Default + DeserializeOwned> FromRows for Vec<LecturerWithCourses<TData>> {
    fn from_rows(rows: &[MySqlRow]) -> Result<Self, sqlx::Error> {
        rows.iter().map(LecturerWithCourses::from_row).collect()
    }
}
