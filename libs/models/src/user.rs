use super::Result;
use ulid::Ulid;

pub struct User {
    pub id: Ulid,
    pub first_name: String,
    pub last_name: String,
    pub contacts: Vec<String>,
}

trait PostgresStore<T> {
    fn get(&self, id: Ulid) -> Result<Option<T>>;
    fn add(&self, value: T) -> Result<T>;
    fn update(&self, value: T) -> Result<T>;
    fn delete(&self, id: Ulid) -> Result<()>;
}
