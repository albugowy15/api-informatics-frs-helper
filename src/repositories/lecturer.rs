use std::collections::HashMap;

use sqlx::FromRow;

use crate::{
    db::DbPool,
    models::{
        class::ClassWithSubjectName,
        course::Course,
        lecturer::{Lecturer, LecturerWithClasses, LecturerWithCourses},
    },
    FromRows,
};

pub struct LecturerRepository<'a> {
    db: &'a DbPool,
}

impl<'a> LecturerRepository<'a> {
    pub fn new(db_connection: &'a DbPool) -> LecturerRepository<'a> {
        LecturerRepository { db: db_connection }
    }

    fn filter(params: &HashMap<String, String>, lecturer: &Lecturer) -> bool {
        let fullname_param = params.get("nama").map(|s| s.to_lowercase());
        let code_param = params.get("kode").map(|s| s.to_lowercase());
        let matches_fullname = fullname_param.as_ref().map_or(true, |fullname| {
            lecturer.nama.to_lowercase().contains(fullname)
        });
        let matches_code = code_param
            .as_ref()
            .map_or(true, |code| lecturer.kode.to_lowercase() == *code);
        matches_fullname && matches_code
    }

    pub async fn get_lecturers(&self) -> Result<Vec<Lecturer>, sqlx::Error> {
        let rows = sqlx::query(
            "select id, code as kode, fullname as nama from Lecturer order by code asc",
        )
        .fetch_all(self.db)
        .await?;
        Vec::from_rows(&rows)
    }

    pub async fn get_lecturers_with_filter(
        &self,
        params: &HashMap<String, String>,
    ) -> Result<Vec<Lecturer>, sqlx::Error> {
        let rows = sqlx::query(
            "select id, code as kode, fullname as nama from Lecturer order by code asc",
        )
        .fetch_all(self.db)
        .await?;
        let lecturers = rows
            .into_iter()
            .filter_map(|row| {
                let lecturer = Lecturer::from_row(&row).ok()?;
                if Self::filter(params, &lecturer) {
                    Some(lecturer)
                } else {
                    None
                }
            })
            .collect();
        Ok(lecturers)
    }

    pub async fn get_lecturers_with_courses(
        &self,
    ) -> Result<Vec<LecturerWithCourses<Vec<Course>>>, sqlx::Error> {
        let rows = sqlx::query(
            "select l.id, l.code as kode, l.fullname as nama,
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
        Vec::from_rows(&rows)
    }

    pub async fn get_lecturers_with_classes(
        &self,
    ) -> Result<Vec<LecturerWithClasses<Vec<ClassWithSubjectName>>>, sqlx::Error> {
        let rows = sqlx::query(
            "select l.id, l.code as kode, l.fullname as nama,
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
        Vec::from_rows(&rows)
    }

    pub async fn get_lecturer_by_id(&self, lecturer_id: &String) -> Result<Lecturer, sqlx::Error> {
        let row =
            sqlx::query("select id, code as kode, fullname as nama from Lecturer where id = ?")
                .bind(lecturer_id)
                .fetch_one(self.db)
                .await?;
        Lecturer::from_row(&row)
    }

    pub async fn get_lecturers_by_course_id(
        &self,
        course_id: &String,
    ) -> Result<Vec<Lecturer>, sqlx::Error> {
        let rows = sqlx::query(
            "select distinct l.id, l.code as kode, l.fullname as nama from Lecturer l 
            inner join _ClassToLecturer cl on cl.B = l.id
            inner join Class c on c.id = cl.A
            inner join Matkul m on m.id = c.matkulId
            where m.id = ?
            order by l.code asc",
        )
        .bind(course_id)
        .fetch_all(self.db)
        .await?;
        Vec::from_rows(&rows)
    }
}
