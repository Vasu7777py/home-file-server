use ::serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;
use rocket::{
    serde::json::{serde_json::json, Value},
    *,
};
use sqlx::{self, postgres::PgPool, FromRow};
use uuid::Uuid;

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct UserFile {
    pub file_id: Uuid,
    pub user_id: Uuid,
    pub file_name: String,
    pub created_date: NaiveDateTime,
    pub file_size: i32,
    pub file_hash: String,
    pub is_shared: bool,
}

#[get("/")]
pub async fn get_all_files(pool: &rocket::State<PgPool>) -> Value {
    let q = "SELECT * FROM userfile";

    let query = sqlx::query_as::<_, UserFile>(q);

    let files = query
        .fetch_all(pool.inner())
        .await
        .expect("Failed To load Files.");

    json!(files)
}
