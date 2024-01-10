use serde::{Deserialize, Serialize};
use sqlx::Row;

use crate::db::DbPool;

pub struct ClassRepository<'a> {
    db: &'a DbPool,
}

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

#[derive(Deserialize, Serialize)]
pub struct CompactClass {
    pub id: String,
    pub kode_kelas: String,
    pub hari: String,
    pub jam: String,
    pub kode_dosen: String,
}

#[derive(Deserialize, Serialize)]
pub struct ClassWithSubjectName {
    id: String,
    matkul: String,
    kode_kelas: String,
    hari: String,
    jam: String,
}

impl<'a> ClassRepository<'a> {
    pub fn new(db_connection: &'a DbPool) -> ClassRepository {
        ClassRepository { db: db_connection }
    }

    pub async fn get_classes(&self) -> Result<Vec<Class>, sqlx::Error> {
        let rows = sqlx::query(
            "select c.id, m.name as matkul, c.code as kode_kelas, c.day,
            s.session_time, l.code as kode_dosen, l.fullname as nama_dosen from Class c
            inner join Session s on s.id = c.sessionId 
            inner join Matkul m on m.id = c.matkulId 
            inner join _ClassToLecturer cl on cl.A = c.id 
            inner join Lecturer l on l.id = cl.B
            order by m.name asc",
        )
        .fetch_all(self.db)
        .await?;
        let mut classes: Vec<Class> = Vec::with_capacity(rows.len());
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
                    where c.id = ?
                    order by m.name asc",
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

    pub async fn get_classes_by_course_id(
        &self,
        course_id: &String,
    ) -> Result<Vec<CompactClass>, sqlx::Error> {
        let rows = sqlx::query(
            "select c.id, c.code as kode_kelas,
                c.day, s.session_time, l.code as kode_dosen
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
        let mut classes: Vec<CompactClass> = Vec::with_capacity(rows.len());
        rows.into_iter().for_each(|row| {
            classes.push(CompactClass {
                id: row.get("id"),
                hari: row.get("day"),
                jam: row.get("session_time"),
                kode_dosen: row.get("kode_dosen"),
                kode_kelas: row.get("kode_kelas"),
            });
        });
        Ok(classes)
    }

    pub async fn get_classes_by_lecturer_id(
        &self,
        lecturer_id: &String,
    ) -> Result<Vec<ClassWithSubjectName>, sqlx::Error> {
        let rows = sqlx::query(
            "select c.id, m.name as matkul, c.code as kode_kelas, c.day, s.session_time as jam
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
        let mut classes: Vec<ClassWithSubjectName> = Vec::with_capacity(rows.len());
        rows.into_iter().for_each(|row| {
            classes.push(ClassWithSubjectName {
                id: row.get("id"),
                matkul: row.get("matkul"),
                kode_kelas: row.get("kode_kelas"),
                hari: row.get("day"),
                jam: row.get("jam"),
            })
        });
        Ok(classes)
    }
}
