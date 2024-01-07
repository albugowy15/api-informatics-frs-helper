use crate::db::DbPool;

pub struct ClassRepository<'a> {
    db: &'a DbPool,
}

impl<'a> ClassRepository<'a> {
    pub fn new(db_connection: &'a DbPool) -> ClassRepository {
        ClassRepository { db: db_connection }
    }
}
