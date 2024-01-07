use serde::Serialize;
use sqlx::Row;

use crate::db::DbPool;

pub struct LecturerRepository<'a> {
    db: &'a DbPool,
}

#[derive(Serialize)]
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
        let mut lecturers: Vec<Lecturer> = Vec::new();
        let rows = sqlx::query("select id, code, fullname from Lecturer")
            .fetch_all(self.db)
            .await?;
        rows.into_iter().for_each(|row| {
            lecturers.push(Lecturer {
                id: row.get("id"),
                kode: row.get("code"),
                nama: row.get("fullname"),
            });
        });
        Ok(lecturers)
    }

    pub async fn get_lecturers_by_id(&self, lecturer_id: &String) -> Result<Lecturer, sqlx::Error> {
        let row = sqlx::query("select id, code, fullname from Lecturer where id = ?")
            .bind(lecturer_id)
            .fetch_one(self.db)
            .await?;
        let lecturer = Lecturer {
            id: row.get("id"),
            kode: row.get("code"),
            nama: row.get("fullname"),
        };

        Ok(lecturer)
    }
}
