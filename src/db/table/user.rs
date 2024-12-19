use serde::Deserialize;
use surrealdb::sql::Thing;

#[derive(Deserialize, Clone)]
pub struct User {
    id: Thing,
    email: String,
    password: String,
}

impl Default for User {
    fn default() -> Self {
        Self {
            id: Thing::from(("0", "0")),
            email: String::default(),
            password: String::default(),
        }
    }
}

impl User {
    pub fn id(&self) -> &Thing {
        &self.id
    }

    pub fn password(&self) -> &str {
        &self.password
    }
}
