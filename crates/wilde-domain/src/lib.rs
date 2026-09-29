//! Isolated synthetic SQLite pilot; legacy CRM parity remains open.
mod model;
mod store;

pub use model::*;
pub use store::Store;

#[derive(Debug)]
pub enum Error {
    Invalid(String),
    RevisionConflict,
    RequestIdConflict,
    NeedsPreview,
    UnsupportedSchema,
    WrongDatabase,
    Capacity,
    Sqlite(rusqlite::Error),
    Json(serde_json::Error),
}
impl From<rusqlite::Error> for Error {
    fn from(value: rusqlite::Error) -> Self {
        Self::Sqlite(value)
    }
}
impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Self::Json(value)
    }
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
impl std::error::Error for Error {}
pub type Result<T> = std::result::Result<T, Error>;
