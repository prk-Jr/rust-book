#[derive(Debug)]
pub struct DBTable {
    pub id: i32,
    pub username: String,
    pub email: Option<String>,
    pub password: String,
}
