use actix_web::{post, web, Responder};
use storage::content_manager::toc::TableOfContent;
use actix_web::rt::time::Instant;
use crate::common::helpers::process_response;
use collection::operations::CollectionUpdateOperations;
use actix_web::web::Query;
use serde::{Deserialize, Serialize};
use schemars::{JsonSchema};

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct UpdateParam {
    pub wait: Option<bool>
}

#[post("/collections/{name}")]
pub async fn update_points(
    toc: web::Data<TableOfContent>,
    web::Path(name): web::Path<String>,
    operation: web::Json<CollectionUpdateOperations>,
    params: Query<UpdateParam>,
) -> impl Responder {
    let timing = Instant::now();

    let response = {
        toc.get_collection(&name)
            .and_then(|collection|
                collection
                    .update(operation.0, params.wait.unwrap_or(false))
                    .map_err(|x| x.into())
            )
    };

    process_response(response, timing)
}