mod mutation;
mod query;

use ::entity::{ user, user::Entity as User };
use sea_orm::*;

pub struct UserService {
    pub db: &DbConn,
}

impl UserService {
    pub async fn new(db: &DbConn) -> Self {
        Self { db }
    }
}
