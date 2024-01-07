use crate::db::DbPool;

pub struct CourseRepository<'a> {
    db: &'a DbPool,
}

impl<'a> CourseRepository<'a> {
    pub fn new(db_connection: &'a DbPool) -> CourseRepository {
        CourseRepository { db: db_connection }
    }
}
