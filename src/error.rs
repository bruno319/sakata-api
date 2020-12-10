use std::error::Error;

use actix_web::HttpResponse;

use crate::utils::http_res::*;

type DieselError = diesel::result::Error;

#[derive(Debug)]
pub enum SakataError {
    DatabaseAccess(String),
    ResourceNotFound(String),
    NotEnoughResource(String),
    BadRequest(String),
    ServerErr(String),
}

impl From<SakataError> for HttpResponse {
    fn from(err: SakataError) -> Self {
        match err {
            SakataError::DatabaseAccess(msg) => server_error(msg),
            SakataError::ResourceNotFound(msg) => not_found(msg),
            SakataError::NotEnoughResource(msg) => forbidden(msg),
            SakataError::ServerErr(msg) => server_error(msg),
            SakataError::BadRequest(msg) => bad_request(msg),
        }
    }
}

impl From<DieselError> for SakataError {
    fn from(err: DieselError) -> Self {
        match err {
            DieselError::NotFound => SakataError::ResourceNotFound(err.to_string()),
            _ => SakataError::DatabaseAccess(err.to_string())
        }
    }
}

impl From<Box<dyn Error + Send + Sync>> for SakataError {
    fn from(err: Box<dyn Error + Send + Sync>) -> Self {
        SakataError::ServerErr(err.to_string())
    }
}