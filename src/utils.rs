use std::str::FromStr;

use actix_multipart::Multipart;
use actix_web::{HttpRequest, web};
use futures::StreamExt;
use serde_json::Value;

use crate::dbconfig::{MysqlPool, MySqlPooledConnection};
use crate::error::SakataError;
use crate::error::SakataError::BadRequest;
use crate::SakataResult;

pub fn mysql_pool_handler(pool: web::Data<MysqlPool>) -> SakataResult<MySqlPooledConnection> {
    let mysql_pool = pool.get()
        .map_err(|e| SakataError::DatabaseAccess(e.to_string()))?;
    Ok(mysql_pool)
}

pub fn error_msg<T: ToString>(message: T) -> Value {
    serde_json::json!({"error_message": message.to_string()})
}

pub fn extract_path_param<T: FromStr>(param: &str, req: &HttpRequest) -> SakataResult<T> {
    req.match_info().get(param)
        .ok_or(BadRequest(format!("{} not provided", param)))?
        .parse()
        .map_err(|_| BadRequest(format!("Could not parse path parameter {}", param)))
}

pub async fn image_bytes(mut payload: Multipart) -> SakataResult<(Vec<u8>, String)> {
    while let Some(item) = payload.next().await {
        let mut field = item
            .map_err(|_| BadRequest("Error on handling multipart data".to_string()))?;
        let content_type = field.content_disposition()
            .ok_or(BadRequest("Error on handling multipart data".to_string()))?;
        let name = content_type.get_name()
            .unwrap_or_default();
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
    Err(SakataError::ResourceNotFound("file not found".to_string()))
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

    pub fn bad_request<T: ToString>(msg: T) -> HttpResponse {
        HttpResponse::BadRequest().json(error_msg(msg))
    }

    pub fn forbidden<T: ToString>(msg: T) -> HttpResponse {
        HttpResponse::Forbidden().json(error_msg(msg))
    }

    pub fn not_found<T: ToString>(msg: T) -> HttpResponse {
        HttpResponse::NotFound().json(error_msg(msg))
    }

    pub fn server_error<T: ToString>(msg: T) -> HttpResponse {
        HttpResponse::InternalServerError().json(error_msg(msg))
    }
}