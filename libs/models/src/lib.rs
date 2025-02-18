mod etl;
mod time_log;
mod user;
mod user_overview;

pub use time_log::*;
pub use user::*;
pub use user_overview::*;

pub type Result<T> = core::result::Result<T, Error>;

pub enum Error {
    DatabaseError(sqlx::Error),
    Unauthorized,
    NotFound,
}
