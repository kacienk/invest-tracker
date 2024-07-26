use actix_web::{
    error::ResponseError,
    get,
    http::{header::ContentType, StatusCode},
    post, put,
    web::Data,
    web::Json,
    web::Path,
    HttpResponse,
};
use serde::{Deserialize, Serialize};

#[get("/investments")]
pub async fn get_investments() -> Result<Json<String>, InvestmentsError> {
    return Ok(Json("hello world!".to_string()));
}
