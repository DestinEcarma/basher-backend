use serde::Deserialize;
use surrealdb::sql::Thing;

#[derive(Deserialize)]
pub struct User {
    id: Thing,
    email: String,
    password: String,
}

impl User {
    pub fn id(&self) -> &Thing {
        &self.id
    }

    pub fn password(&self) -> &str {
        &self.password
    }
}
