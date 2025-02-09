use std::collections::HashMap;

use sqlx::FromRow;

use crate::{
    db::DbPool,
    models::class::{Class, ClassWithSubjectName, CompactClass},
    FromRows,
};

pub struct ClassRepository<'a> {
    db: &'a DbPool,
}

impl<'a> ClassRepository<'a> {
    pub fn new(db_connection: &'a DbPool) -> ClassRepository<'a> {
        ClassRepository { db: db_connection }
    }

    fn filter(params: &HashMap<String, String>, class: &Class) -> bool {
        let day_param = params.get("hari").map(|s| s.to_lowercase());
        let hour_param = params.get("jam").map(|s| s.to_string());
        let course_param = params.get("matkul").map(|s| s.to_lowercase());
        let lecturer_code_param = params.get("kode_dosen").map(|s| s.to_lowercase());
        let matches_day = day_param
            .as_ref()
            .map_or(true, |day| class.hari.to_lowercase() == *day);
        let matches_hour = hour_param.as_ref().map_or(true, |hour| class.jam == *hour);
        let matches_course = course_param
            .as_ref()
            .map_or(true, |course| class.matkul.to_lowercase().contains(course));
        let matches_lecturer_code = lecturer_code_param.as_ref().map_or(true, |lecturer| {
            class.kode_dosen.to_lowercase() == *lecturer
        });
        matches_day && matches_hour && matches_course && matches_lecturer_code
    }

    pub async fn get_classes(&self) -> Result<Vec<Class>, sqlx::Error> {
        let rows = sqlx::query(
            "select c.id, m.name as matkul, c.code as kode_kelas, c.day as hari,
            s.session_time as jam, l.code as kode_dosen, l.fullname as nama_dosen from Class c
            inner join Session s on s.id = c.sessionId 
            inner join Matkul m on m.id = c.matkulId 
            inner join _ClassToLecturer cl on cl.A = c.id 
            inner join Lecturer l on l.id = cl.B
            order by m.name asc",
        )
        .fetch_all(self.db)
        .await?;
        Vec::from_rows(&rows)
    }

    pub async fn get_classes_with_filter(
        &self,
        params: &HashMap<String, String>,
    ) -> Result<Vec<Class>, sqlx::Error> {
        let rows = sqlx::query(
            "select c.id, m.name as matkul, c.code as kode_kelas, c.day as hari,
            s.session_time as jam, l.code as kode_dosen, l.fullname as nama_dosen from Class c
            inner join Session s on s.id = c.sessionId 
            inner join Matkul m on m.id = c.matkulId 
            inner join _ClassToLecturer cl on cl.A = c.id 
            inner join Lecturer l on l.id = cl.B
            order by m.name asc",
        )
        .fetch_all(self.db)
        .await?;

        let classes = rows
            .into_iter()
            .filter_map(|row| {
                let class = Class::from_row(&row).ok()?;
                if Self::filter(params, &class) {
                    Some(class)
                } else {
                    None
                }
            })
            .collect();
        Ok(classes)
    }

    pub async fn get_class_by_id(&self, class_id: &String) -> Result<Class, sqlx::Error> {
        let row = sqlx::query(
            "select c.id, m.name as matkul, c.code as kode_kelas, c.day as hari,
                    s.session_time as jam, l.code as kode_dosen, l.fullname as nama_dosen from Class c
                    inner join Session s on s.id = c.sessionId 
                    inner join Matkul m on m.id = c.matkulId 
                    inner join _ClassToLecturer cl on cl.A = c.id 
                    inner join Lecturer l on l.id = cl.B
                    where c.id = ?
                    order by m.name asc",
        )
        .bind(class_id)
        .fetch_one(self.db)
        .await?;
        Class::from_row(&row)
    }

    pub async fn get_classes_by_course_id(
        &self,
        course_id: &String,
    ) -> Result<Vec<CompactClass>, sqlx::Error> {
        let rows = sqlx::query(
            "select c.id, c.code as kode_kelas,
                c.day as hari, s.session_time as jam, l.code as kode_dosen
                from Class c
                inner join Session s on s.id = c.sessionId 
                inner join Matkul m on m.id = c.matkulId 
                inner join _ClassToLecturer cl on cl.A = c.id 
                inner join Lecturer l on l.id = cl.B
                where m.id = ?
                order by c.code asc",
        )
        .bind(course_id)
        .fetch_all(self.db)
        .await?;
        Vec::from_rows(&rows)
    }

    pub async fn get_classes_by_lecturer_id(
        &self,
        lecturer_id: &String,
    ) -> Result<Vec<ClassWithSubjectName>, sqlx::Error> {
        let rows = sqlx::query(
            "select c.id, m.name as matkul, c.code as kode_kelas, c.day as hari, s.session_time as jam
                    from Class c
                    inner join Session s on s.id = c.sessionId 
                    inner join Matkul m on m.id = c.matkulId 
                    inner join _ClassToLecturer cl on cl.A = c.id 
                    inner join Lecturer l on l.id = cl.B
                    where l.id = ?
                    order by c.code asc",
        )
        .bind(lecturer_id)
        .fetch_all(self.db)
        .await?;
        Vec::from_rows(&rows)
    }
}
