use crate::db::DbPool;

pub struct LecturerRepository<'a> {
    db: &'a DbPool,
}

impl<'a> LecturerRepository<'a> {
    pub fn new(db_connection: &'a DbPool) -> LecturerRepository {
        LecturerRepository { db: db_connection }
    }
}
