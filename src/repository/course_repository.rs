use crate::{
    db::DbPool,
    model::{
        class_model::CompactClass,
        course_model::{Course, CourseWithClass, CourseWithLecturer},
        lecturer_model::Lecturer,
        FromRow,
    },
};

pub struct CourseRepository<'a> {
    db: &'a DbPool,
}

impl<'a> CourseRepository<'a> {
    pub fn new(db_connection: &'a DbPool) -> CourseRepository {
        CourseRepository { db: db_connection }
    }

    pub async fn get_courses(&self) -> Result<Vec<Course>, sqlx::Error> {
        let rows = sqlx::query(
            "select m.id, m.name as matkul, m.semester, m.sks from Matkul m order by m.semester asc",
        )
        .fetch_all(self.db)
        .await?;
        let mut courses = Vec::with_capacity(rows.len());
        rows.into_iter()
            .for_each(|row| courses.push(Course::from_row(&row)));
        Ok(courses)
    }

    pub async fn get_course_by_id(&self, course_id: &String) -> Result<Course, sqlx::Error> {
        let row = sqlx::query(
            "select m.id, m.name as matkul, m.semester, m.sks from Matkul m where m.id = ?",
        )
        .bind(course_id)
        .fetch_one(self.db)
        .await?;
        Ok(Course::from_row(&row))
    }

    pub async fn get_courses_by_lecturer_id(
        &self,
        lecturer_id: &String,
    ) -> Result<Vec<Course>, sqlx::Error> {
        let rows = sqlx::query(
            "select m.id, m.name as matkul, m.semester, m.sks
                    from Matkul m
                    inner join Class c on c.matkulId = m.id
                    inner join _ClassToLecturer cl on cl.A = c.id
                    inner join Lecturer l on l.id = cl.B
                    where l.id = ?
                    group by m.id
                    order by m.semester asc",
        )
        .bind(lecturer_id)
        .fetch_all(self.db)
        .await?;
        let mut courses: Vec<Course> = Vec::with_capacity(rows.len());
        rows.into_iter()
            .for_each(|row| courses.push(Course::from_row(&row)));
        Ok(courses)
    }

    pub async fn get_courses_with_lecturers(
        &self,
    ) -> Result<Vec<CourseWithLecturer<Vec<Lecturer>>>, sqlx::Error> {
        let rows = sqlx::query(
            "select 
                        m.id,
                        m.name as matkul,
                        m.semester,
                        m.sks,
                        CONCAT('[', GROUP_CONCAT(
                            distinct JSON_OBJECT('id', l.id, 'kode', l.code, 'nama', l.fullname)	
                        ), ']') AS dosen 
                    from Matkul m
                    inner join Class c on c.matkulId = m.id
                    inner join _ClassToLecturer cl on cl.A = c.id
                    inner join Lecturer l on l.id = cl.B
                    group by m.id
                    order by m.semester asc",
        )
        .fetch_all(self.db)
        .await?;
        let mut courses_lecturers: Vec<CourseWithLecturer<Vec<Lecturer>>> =
            Vec::with_capacity(rows.len());
        rows.into_iter()
            .for_each(|row| courses_lecturers.push(CourseWithLecturer::from_row(&row)));
        Ok(courses_lecturers)
    }

    pub async fn get_courses_with_classes(
        &self,
    ) -> Result<Vec<CourseWithClass<Vec<CompactClass>>>, sqlx::Error> {
        let rows = sqlx::query(
            "select m.id, m.name as matkul, m.semester, m.sks,
                        concat('[', group_concat(
                                distinct json_object(
                                    'id', c.id,
                                    'kode_kelas', c.code,
                                    'hari', c.day,
                                    'jam', s.session_time,
                                    'kode_dosen', l.code
                                )	
                            ), ']') as kelas
                    from Matkul m
                    inner join Class c on c.matkulId = m.id
                    inner join Session s on s.id = c.sessionId
                    inner join _ClassToLecturer cl on cl.A = c.id
                    inner join Lecturer l on cl.B = l.id
                    group by m.id
                    order by m.semester asc",
        )
        .fetch_all(self.db)
        .await?;
        let mut courses_classes: Vec<CourseWithClass<Vec<CompactClass>>> =
            Vec::with_capacity(rows.len());
        rows.into_iter().for_each(|row| {
            courses_classes.push(CourseWithClass::<Vec<CompactClass>>::from_row(&row))
        });
        Ok(courses_classes)
    }
}
