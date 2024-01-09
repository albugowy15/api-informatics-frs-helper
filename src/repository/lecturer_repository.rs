use serde::{Deserialize, Serialize};
use sqlx::Row;

use crate::db::DbPool;

pub struct LecturerRepository<'a> {
    db: &'a DbPool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Lecturer {
    pub id: String,
    pub kode: String,
    pub nama: String,
}

impl<'a> LecturerRepository<'a> {
    pub fn new(db_connection: &'a DbPool) -> LecturerRepository {
        LecturerRepository { db: db_connection }
    }

    pub async fn get_lecturers(&self) -> Result<Vec<Lecturer>, sqlx::Error> {
        let rows = sqlx::query("select id, code, fullname from Lecturer order by code asc")
            .fetch_all(self.db)
            .await?;
        let mut lecturers: Vec<Lecturer> = Vec::with_capacity(rows.len());
        rows.into_iter().for_each(|row| {
            lecturers.push(Lecturer {
                id: row.get("id"),
                kode: row.get("code"),
                nama: row.get("fullname"),
            });
        });
        Ok(lecturers)
    }

    pub async fn get_lecturer_by_id(&self, lecturer_id: &String) -> Result<Lecturer, sqlx::Error> {
        let row = sqlx::query("select id, code, fullname from Lecturer where id = ?")
            .bind(lecturer_id)
            .fetch_one(self.db)
            .await?;
        Ok(Lecturer {
            id: row.get("id"),
            kode: row.get("code"),
            nama: row.get("fullname"),
        })
    }

    pub async fn get_lecturers_by_course_id(
        &self,
        course_id: &String,
    ) -> Result<Vec<Lecturer>, sqlx::Error> {
        let rows = sqlx::query(
            "select distinct l.id, l.code, l.fullname from Lecturer l 
            inner join _ClassToLecturer cl on cl.B = l.id
            inner join Class c on c.id = cl.A
            inner join Matkul m on m.id = c.matkulId
            where m.id = ?
            order by l.code asc",
        )
        .bind(course_id)
        .fetch_all(self.db)
        .await?;
        let mut lecturers: Vec<Lecturer> = Vec::with_capacity(rows.len());
        rows.into_iter().for_each(|row| {
            lecturers.push(Lecturer {
                id: row.get("id"),
                kode: row.get("code"),
                nama: row.get("fullname"),
            })
        });
        Ok(lecturers)
    }
}
