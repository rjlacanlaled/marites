#[derive(Debug, Deserialize)]
pub struct UserCreate {
    pub username: String,
    pub password: String,
}

impl UserCreate {
    pub fn new(username: String, password: String) -> Self {
        Self { username, password }
    }

    pub async fn create(user_create: &UserCreate) -> Result<User, Error> {
        let user = User::find();
        if user.is_some() {
            return Err(Error::UserAlreadyExists(username));
        }
        let user = User::insert(
            Entity::new()
                .set(user::Column::Username, username)
                .set(user::Column::Password, password)
        ).exec(&db).await?;
        Ok(user)
    }
}
