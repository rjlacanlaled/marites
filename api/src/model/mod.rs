mod error;
mod store;
mod task;

pub use self::error::{ Error, Result };
use self::store::{ Db, new_db_pool };

#[derive(Clone)]
pub struct ModelManager {
    db: Db,
}

impl ModelManager {
    pub async fn new() -> Result<Self> {
        let db = new_db_pool().await?;
        Ok(ModelManager {
            db,
        })
    }

    /// Returns a reference to the database connection pool.
    /// Only in the model layer
    pub(in crate::model) fn db(&self) -> &Db {
        &self.db
    }
}
