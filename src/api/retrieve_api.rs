use actix_web::{get, post, web, Responder};
use storage::content_manager::toc::TableOfContent;
use crate::common::helpers::process_response;
use actix_web::rt::time::Instant;
use segment::types::PointIdType;
use serde::{Deserialize, Serialize};
use schemars::{JsonSchema};
use storage::content_manager::errors::StorageError;

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct PointRequest {
    pub ids: Vec<PointIdType>
}

#[get("/collections/{name}/points/{id}")]
pub async fn get_point(
    toc: web::Data<TableOfContent>,
    web::Path((name, point_id)): web::Path<(String, PointIdType)>,
) -> impl Responder {
    let timing = Instant::now();

    let response = {
        toc.get_collection(&name)
            .and_then(|collection| collection
                .retrieve(&vec![point_id], true, true)
                .map_err(|err| err.into())
                .map(|points| points.into_iter().next())
            )
    };

    let response = match response {
        Ok(record) => match record {
            None => Err(StorageError::NotFound { description: format!("Point with id {} does not exists!", point_id) }),
            Some(record) => Ok(record)
        },
        Err(e) => Err(e)
    };

    process_response(response, timing)
}


#[post("/collections/{name}/points")]
pub async fn get_vectors(
    toc: web::Data<TableOfContent>,
    web::Path(name): web::Path<String>,
    request: web::Json<PointRequest>,
) -> impl Responder {
    let timing = Instant::now();

    let response = {
        toc.get_collection(&name)
            .and_then(|collection| collection
                .retrieve(&request.ids, true, true)
                .map_err(|err| err.into())
            )
    };

    process_response(response, timing)
}
