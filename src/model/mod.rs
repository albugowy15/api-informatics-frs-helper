use sqlx::mysql::MySqlRow;

pub mod class_model;
pub mod course_model;
pub mod lecturer_model;
pub mod response_model;

pub trait FromRows {
    fn from_rows(rows: &[MySqlRow]) -> Result<Self, sqlx::Error>
    where
        Self: Sized;
}
