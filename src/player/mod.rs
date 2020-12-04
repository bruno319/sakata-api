mod dao;
pub(crate) mod handlers;

use serde::{Deserialize, Serialize};

use crate::schema::players;

#[derive(Queryable, Identifiable, Insertable, Serialize, Deserialize)]
pub struct Player {
    pub id: i32,
    pub nickname: String,
}