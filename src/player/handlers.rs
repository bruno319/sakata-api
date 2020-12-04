use actix_web::{get, HttpRequest, HttpResponse, web};

use crate::dbconfig::MysqlPool;
use crate::utils::{extract_path_param, mysql_pool_handler, http_res};

use super::dao;

#[get("/players/{id}")]
pub async fn get_player_by_id(
    req: HttpRequest,
    pool: web::Data<MysqlPool>,
) -> Result<HttpResponse, HttpResponse> {
    let mysql_pool = mysql_pool_handler(pool)?;
    let player_id: i32 = extract_path_param("id", &req)?;

    match dao::find_by_id(&mysql_pool, player_id) {
        Ok(p) => Ok(http_res::ok(p)),
        Err(e) => Err(http_res::internal_server_error(&e.to_string())),
    }
}