use std::str::FromStr;

use actix_multipart::{Field, Multipart};
use actix_web::{HttpRequest, HttpResponse, web};
use futures::StreamExt;
use serde_json::Value;

use http_res::server_error;

use crate::dbconfig::{MysqlPool, MySqlPooledConnection};
use crate::error::SakataError;
use crate::SakataResult;
use crate::utils::http_res::not_found;

pub fn mysql_pool_handler(pool: web::Data<MysqlPool>) -> SakataResult<MySqlPooledConnection> {
    let mysql_pool = pool.get()
        .map_err(|e| SakataError::DatabaseAccess(server_error(e)))?;
    Ok(mysql_pool)
}

pub fn error_msg<T: ToString>(message: T) -> Value {
    serde_json::json!({"error_message": message.to_string()})
}

pub fn extract_path_param<T: FromStr>(param: &str, req: &HttpRequest) -> Result<T, HttpResponse> {
    req.match_info().get(param)
        .ok_or(HttpResponse::BadRequest().json(error_msg(&format!("{} not provided", param))))?
        .parse()
        .map_err(|_| HttpResponse::BadRequest()
            .json(error_msg(&format!("Could not parse path parameter {}", param))))
}

pub async fn image_bytes(payload: &mut Multipart) -> SakataResult<(Vec<u8>, String)> {
    while let Some(item) = payload.next().await {
        let mut field: Field = item.expect(" split_payload err");
        let content_type = field.content_disposition().unwrap();
        let name = content_type.get_name().unwrap_or_default();
        if name == "basecard" {
            if let Some(filename) = content_type.get_filename() {
                let mut output = Vec::new();

                while let Some(chunk) = field.next().await {
                    let data = chunk.unwrap().to_vec();
                    output.extend(data);
                }
                return Ok((output, filename.to_string()));
            }
        }
    }
    Err(SakataError::ResourceNotFound(not_found("file not found")))
}

pub mod http_res {
    use actix_web::HttpResponse;
    use serde::Serialize;

    use crate::utils::error_msg;

    pub fn ok<T: Serialize>(json: T) -> HttpResponse {
        HttpResponse::Ok().json(json)
    }

    pub fn created<T: Serialize>(json: T) -> HttpResponse {
        HttpResponse::Created().json(json)
    }

    pub fn server_error<T: ToString>(msg: T) -> HttpResponse {
        HttpResponse::InternalServerError().json(error_msg(msg))
    }

    pub fn not_found<T: ToString>(msg: T) -> HttpResponse {
        HttpResponse::NotFound().json(error_msg(msg))
    }

    pub fn forbidden<T: ToString>(msg: T) -> HttpResponse {
        HttpResponse::Forbidden().json(error_msg(msg))
    }
}