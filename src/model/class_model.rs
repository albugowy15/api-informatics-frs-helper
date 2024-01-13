use axum::Json;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sqlx::{mysql::MySqlRow, FromRow};

use super::FromRows;

#[derive(Deserialize, Serialize, sqlx::FromRow)]
pub struct Class {
    pub id: String,
    pub matkul: String,
    pub kode_kelas: String,
    pub hari: String,
    pub jam: String,
    pub kode_dosen: String,
    pub nama_dosen: String,
}
impl From<Class> for Json<Value> {
    fn from(class: Class) -> Self {
        Json(json!(class))
    }
}
impl FromRows for Vec<Class> {
    fn from_rows(rows: &[MySqlRow]) -> Result<Self, sqlx::Error> {
        rows.iter().map(Class::from_row).collect()
    }
}

#[derive(Deserialize, Serialize, sqlx::FromRow)]
pub struct CompactClass {
    pub id: String,
    pub kode_kelas: String,
    pub hari: String,
    pub jam: String,
    pub kode_dosen: String,
}
impl From<CompactClass> for Json<Value> {
    fn from(class: CompactClass) -> Self {
        Json(json!(class))
    }
}
impl FromRows for Vec<CompactClass> {
    fn from_rows(rows: &[MySqlRow]) -> Result<Self, sqlx::Error> {
        rows.iter().map(CompactClass::from_row).collect()
    }
}

#[derive(Deserialize, Serialize, sqlx::FromRow)]
pub struct ClassWithSubjectName {
    id: String,
    matkul: String,
    kode_kelas: String,
    hari: String,
    jam: String,
}
impl From<ClassWithSubjectName> for Json<Value> {
    fn from(class: ClassWithSubjectName) -> Self {
        Json(json!(class))
    }
}
impl FromRows for Vec<ClassWithSubjectName> {
    fn from_rows(rows: &[MySqlRow]) -> Result<Self, sqlx::Error> {
        rows.iter().map(ClassWithSubjectName::from_row).collect()
    }
}
