use serde::Serialize;
use sqlx::Row;

use crate::db::DbPool;

pub struct ClassRepository<'a> {
    db: &'a DbPool,
}

#[derive(Serialize)]
pub struct Class {
    pub id: String,
    pub matkul: String,
    pub kode_kelas: String,
    pub hari: String,
    pub jam: String,
    pub kode_dosen: String,
    pub nama_dosen: String,
}

impl<'a> ClassRepository<'a> {
    pub fn new(db_connection: &'a DbPool) -> ClassRepository {
        ClassRepository { db: db_connection }
    }

    pub async fn get_classes(&self) -> Result<Vec<Class>, sqlx::Error> {
        let mut classes: Vec<Class> = Vec::new();
        let rows = sqlx::query(
            "select c.id, m.name as matkul, c.code as kode_kelas, c.day,
            s.session_time, l.code as kode_dosen, l.fullname as nama_dosen from Class c
            inner join Session s on s.id = c.sessionId 
            inner join Matkul m on m.id = c.matkulId 
            inner join _ClassToLecturer cl on cl.A = c.id 
            inner join Lecturer l on l.id = cl.B",
        )
        .fetch_all(self.db)
        .await?;
        rows.into_iter().for_each(|row| {
            classes.push(Class {
                id: row.get("id"),
                matkul: row.get("matkul"),
                kode_kelas: row.get("kode_kelas"),
                hari: row.get("day"),
                jam: row.get("session_time"),
                kode_dosen: row.get("kode_dosen"),
                nama_dosen: row.get("nama_dosen"),
            })
        });

        Ok(classes)
    }

    pub async fn get_class_by_id(&self, class_id: &String) -> Result<Class, sqlx::Error> {
        let row = sqlx::query(
            "select c.id, m.name as matkul, c.code as kode_kelas, c.day,
        s.session_time, l.code as kode_dosen, l.fullname as nama_dosen from Class c
        inner join Session s on s.id = c.sessionId 
        inner join Matkul m on m.id = c.matkulId 
        inner join _ClassToLecturer cl on cl.A = c.id 
        inner join Lecturer l on l.id = cl.B
        where c.id = ?",
        )
        .bind(class_id)
        .fetch_one(self.db)
        .await?;

        Ok(Class {
            id: row.get("id"),
            matkul: row.get("matkul"),
            kode_kelas: row.get("kode_kelas"),
            hari: row.get("day"),
            jam: row.get("session_time"),
            kode_dosen: row.get("kode_dosen"),
            nama_dosen: row.get("nama_dosen"),
        })
    }
}
