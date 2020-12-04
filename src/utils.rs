use actix_web::{HttpResponse, web, HttpRequest};
use crate::dbconfig::{MysqlPool, MySqlPooledConnection};
use serde_json::Value;
use std::str::FromStr;

pub fn mysql_pool_handler(pool: web::Data<MysqlPool>) -> Result<MySqlPooledConnection, HttpResponse> {
    pool.get()
        .map_err(|e| HttpResponse::InternalServerError().json(e.to_string()))
}

pub fn error_msg(message: &str) -> Value {
    serde_json::json!({"errorMessage": message})
}

pub fn extract_path_param<T: FromStr>(param: &str, req: &HttpRequest) -> Result<T, HttpResponse> {
    req.match_info().get(param)
        .ok_or(HttpResponse::BadRequest().json(error_msg(&format!("{} not provided", param))))?
        .parse()
        .map_err(|_| HttpResponse::BadRequest()
            .json(error_msg(&format!("Could not parse path parameter {}", param))))
}

pub mod http_res {
    use actix_web::HttpResponse;
    use serde::Serialize;
    use crate::utils::error_msg;

    pub fn ok<T: Serialize>(json: T) -> HttpResponse {
        HttpResponse::Ok().json(json)
    }

    pub fn internal_server_error(msg: &str) -> HttpResponse {
        HttpResponse::InternalServerError().json(error_msg(msg))
    }
 }