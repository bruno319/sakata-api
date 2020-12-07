use std::error::Error;

use actix_web::HttpResponse;

use crate::utils::http_res::*;

type DieselError = diesel::result::Error;

#[derive(Debug)]
pub enum SakataError {
    DatabaseAccess(HttpResponse),
    ResourceNotFound(HttpResponse),
    NotEnoughResource(HttpResponse),
    InternalServerError(HttpResponse),
}

impl From<SakataError> for HttpResponse {
    fn from(err: SakataError) -> Self {
        match err {
            SakataError::DatabaseAccess(http_res) => http_res,
            SakataError::ResourceNotFound(http_res) => http_res,
            SakataError::NotEnoughResource(http_res) => http_res,
            SakataError::InternalServerError(http_res) => http_res,
        }
    }
}

impl From<DieselError> for SakataError {
    fn from(err: DieselError) -> Self {
        match err {
            DieselError::NotFound => SakataError::ResourceNotFound(not_found(err)),
            _ => SakataError::DatabaseAccess(server_error(err))
        }
    }
}

impl From<Box<dyn Error + Send+ Sync>> for SakataError {
    fn from(err: Box<dyn Error + Send+ Sync>) -> Self {
        SakataError::InternalServerError(server_error(err))
    }
}