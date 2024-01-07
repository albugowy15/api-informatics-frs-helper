use serde::Serialize;
use sqlx::Row;

use crate::db::DbPool;

pub struct CourseRepository<'a> {
    db: &'a DbPool,
}

#[derive(Serialize)]
pub struct Course {
    pub id: String,
    pub nama: String,
    pub semester: i8,
    pub sks: i8,
}

impl<'a> CourseRepository<'a> {
    pub fn new(db_connection: &'a DbPool) -> CourseRepository {
        CourseRepository { db: db_connection }
    }

    pub async fn get_courses(&self) -> Result<Vec<Course>, sqlx::Error> {
        let mut courses = Vec::new();
        let rows = sqlx::query("select m.id, m.name, m.semester, m.sks from Matkul m")
            .fetch_all(self.db)
            .await?;
        rows.into_iter().for_each(|row| {
            courses.push(Course {
                id: row.get("id"),
                nama: row.get("name"),
                semester: row.get("semester"),
                sks: row.get("sks"),
            })
        });
        Ok(courses)
    }

    pub async fn get_course_by_id(&self, course_id: &String) -> Result<Course, sqlx::Error> {
        let row =
            sqlx::query("select m.id, m.name, m.semester, m.sks from Matkul m where m.id = ?")
                .bind(course_id)
                .fetch_one(self.db)
                .await?;
        let course = Course {
            id: row.get("id"),
            nama: row.get("name"),
            semester: row.get("semester"),
            sks: row.get("sks"),
        };
        Ok(course)
    }
}
