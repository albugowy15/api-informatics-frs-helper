use crate::{
    db::DbPool,
    model::{
        class_model::ClassWithSubjectName,
        course_model::Course,
        lecturer_model::{Lecturer, LecturerWithClasses, LecturerWithCourses},
        FromRow, FromRows,
    },
};

pub struct LecturerRepository<'a> {
    db: &'a DbPool,
}

impl<'a> LecturerRepository<'a> {
    pub fn new(db_connection: &'a DbPool) -> LecturerRepository {
        LecturerRepository { db: db_connection }
    }

    pub async fn get_lecturers(&self) -> Result<Vec<Lecturer>, sqlx::Error> {
        let rows = sqlx::query(
            "select id, code as kode, fullname as nama from Lecturer order by code asc",
        )
        .fetch_all(self.db)
        .await?;
        Ok(Vec::from_rows(&rows))
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
        Ok(Vec::from_rows(&rows))
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
        Ok(Vec::from_rows(&rows))
    }

    pub async fn get_lecturer_by_id(&self, lecturer_id: &String) -> Result<Lecturer, sqlx::Error> {
        let row =
            sqlx::query("select id, code as kode, fullname as nama from Lecturer where id = ?")
                .bind(lecturer_id)
                .fetch_one(self.db)
                .await?;
        Ok(Lecturer::from_row(&row))
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
        Ok(Vec::from_rows(&rows))
    }
}
