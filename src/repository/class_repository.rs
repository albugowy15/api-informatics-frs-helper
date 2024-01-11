use crate::{
    db::DbPool,
    model::{class_model::*, FromRow},
};

pub struct ClassRepository<'a> {
    db: &'a DbPool,
}

impl<'a> ClassRepository<'a> {
    pub fn new(db_connection: &'a DbPool) -> ClassRepository {
        ClassRepository { db: db_connection }
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
        let mut classes: Vec<Class> = Vec::with_capacity(rows.len());
        rows.into_iter()
            .for_each(|row| classes.push(Class::from_row(&row)));

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
        Ok(Class::from_row(&row))
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
        let mut classes: Vec<CompactClass> = Vec::with_capacity(rows.len());
        rows.into_iter().for_each(|row| {
            classes.push(CompactClass::from_row(&row));
        });
        Ok(classes)
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
        let mut classes: Vec<ClassWithSubjectName> = Vec::with_capacity(rows.len());
        rows.into_iter()
            .for_each(|row| classes.push(ClassWithSubjectName::from_row(&row)));
        Ok(classes)
    }
}
