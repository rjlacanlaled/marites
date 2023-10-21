mod error;
mod store;

pub use self::error::{ Error, Result };

#[derive(Clone)]
pub struct ModelManager {
    // db: Db,
}

impl ModelManager {
    pub async fn new() -> Result<Self> {
        Ok(ModelManager {})
    }
}
