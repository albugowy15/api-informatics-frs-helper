use sqlx::mysql::MySqlRow;

pub mod class_model;
pub mod course_model;
pub mod lecturer_model;

pub trait FromRow {
    fn from_row(row: &MySqlRow) -> Self;
}
