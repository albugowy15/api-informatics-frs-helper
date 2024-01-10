use serde::{Deserialize, Serialize};
use sqlx::Row;

use crate::db::DbPool;

use super::{class_repository::ClassWithSubjectName, course_repository::Course};

pub struct LecturerRepository<'a> {
    db: &'a DbPool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Lecturer {
    pub id: String,
    pub kode: String,
    pub nama: String,
}

#[derive(Deserialize, Serialize)]
pub struct LecturerWithClasses<TClasses> {
    pub id: String,
    pub kode: String,
    pub nama: String,
    pub kelas: TClasses,
}

#[derive(Deserialize, Serialize)]
pub struct LecturerWithCourses<TCourses> {
    pub id: String,
    pub kode: String,
    pub nama: String,
    pub matkul: TCourses,
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

    pub async fn get_lecturers_with_courses(
        &self,
    ) -> Result<Vec<LecturerWithCourses<Vec<Course>>>, sqlx::Error> {
        let rows = sqlx::query(
            "select l.id, l.code, l.fullname,
                    concat('[',
                        group_concat(
                            distinct json_object('id', m.id, 'matkul', m.name, 'semester', m.semester, 'sks', m.sks)
                        ),
                    ']') as matkul
                    from Lecturer l
                    inner join _ClassToLecturer cl on cl.B = l.id
                    inner join Class c on c.id = cl.A
                    inner join Matkul m on m.id = c.matkulId
                    group by l.id
                    order by l.code asc"
            ).fetch_all(self.db).await?;
        let mut lecturers_courses = Vec::with_capacity(rows.len());
        rows.into_iter().for_each(|row| {
            lecturers_courses.push(LecturerWithCourses::<Vec<Course>> {
                id: row.get("id"),
                kode: row.get("code"),
                nama: row.get("fullname"),
                matkul: serde_json::from_str(row.get("matkul")).unwrap_or_default(),
            })
        });
        Ok(lecturers_courses)
    }

    pub async fn get_lecturers_with_classes(
        &self,
    ) -> Result<Vec<LecturerWithCourses<Vec<ClassWithSubjectName>>>, sqlx::Error> {
        let rows = sqlx::query(
            "select l.id, l.code, l.fullname,
                    concat('[',
                        group_concat(
                            distinct json_object('id', c.id, 'matkul', m.name, 'kode_kelas', c.code, 'hari', c.day, 'jam', s.session_time)
                        ),
                    ']') as kelas
                    from Lecturer l
                    inner join _ClassToLecturer cl on cl.B = l.id
                    inner join Class c on c.id = cl.A
                    inner join Session s on s.id = c.sessionId
                    inner join Matkul m on m.id = c.matkulId
                    group by l.id
                    order by l.code asc"
            ).fetch_all(self.db).await?;
        let mut lecturers_classes = Vec::with_capacity(rows.len());
        rows.into_iter().for_each(|row| {
            lecturers_classes.push(LecturerWithCourses::<Vec<ClassWithSubjectName>> {
                id: row.get("id"),
                kode: row.get("code"),
                nama: row.get("fullname"),
                matkul: serde_json::from_str(row.get("kelas")).unwrap_or_default(),
            })
        });
        Ok(lecturers_classes)
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
