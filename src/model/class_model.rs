use serde::{Deserialize, Serialize};
use sqlx::{mysql::MySqlRow, Row};

use crate::services::IntoJson;

use super::{FromRow, FromRows};

#[derive(Deserialize, Serialize)]
pub struct Class {
    pub id: String,
    pub matkul: String,
    pub kode_kelas: String,
    pub hari: String,
    pub jam: String,
    pub kode_dosen: String,
    pub nama_dosen: String,
}
impl IntoJson for Class {}
impl FromRow for Class {
    fn from_row(row: &MySqlRow) -> Self {
        Self {
            id: row.get("id"),
            matkul: row.get("matkul"),
            kode_kelas: row.get("kode_kelas"),
            hari: row.get("hari"),
            jam: row.get("jam"),
            kode_dosen: row.get("kode_dosen"),
            nama_dosen: row.get("nama_dosen"),
        }
    }
}
impl FromRows for Vec<Class> {
    fn from_rows(rows: &[MySqlRow]) -> Self {
        let mut classes = Vec::with_capacity(rows.len());
        rows.iter().for_each(|row| {
            classes.push(Class::from_row(row));
        });
        classes
    }
}

#[derive(Deserialize, Serialize)]
pub struct CompactClass {
    pub id: String,
    pub kode_kelas: String,
    pub hari: String,
    pub jam: String,
    pub kode_dosen: String,
}
impl IntoJson for CompactClass {}
impl FromRow for CompactClass {
    fn from_row(row: &MySqlRow) -> Self {
        Self {
            id: row.get("id"),
            kode_kelas: row.get("kode_kelas"),
            hari: row.get("hari"),
            jam: row.get("jam"),
            kode_dosen: row.get("kode_dosen"),
        }
    }
}
impl FromRows for Vec<CompactClass> {
    fn from_rows(rows: &[MySqlRow]) -> Self {
        let mut classes = Vec::with_capacity(rows.len());
        rows.iter().for_each(|row| {
            classes.push(CompactClass::from_row(row));
        });
        classes
    }
}

#[derive(Deserialize, Serialize)]
pub struct ClassWithSubjectName {
    id: String,
    matkul: String,
    kode_kelas: String,
    hari: String,
    jam: String,
}
impl IntoJson for ClassWithSubjectName {}
impl FromRow for ClassWithSubjectName {
    fn from_row(row: &MySqlRow) -> Self {
        Self {
            id: row.get("id"),
            kode_kelas: row.get("kode_kelas"),
            hari: row.get("hari"),
            jam: row.get("jam"),
            matkul: row.get("matkul"),
        }
    }
}
impl FromRows for Vec<ClassWithSubjectName> {
    fn from_rows(rows: &[MySqlRow]) -> Self {
        let mut classes = Vec::with_capacity(rows.len());
        rows.iter().for_each(|row| {
            classes.push(ClassWithSubjectName::from_row(row));
        });
        classes
    }
}
