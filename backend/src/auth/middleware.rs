use crate::db::AppState;
use actix_web::dev::ServiceRequest;
use actix_web::web::Data;
use actix_web::Error;
use actix_web_httpauth::extractors::bearer::{BearerAuth, Config};
use actix_web_httpauth::extractors::AuthenticationError;

use super::services::jwt_service::JwtService;

pub async fn auth_validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    let data: &Data<AppState> = req.app_data::<Data<AppState>>().unwrap();
    let secret: &String = &data.secret;
    let token: &str = credentials.token();

    let config: Config = req
        .app_data::<Config>()
        .map(|data| data.clone())
        .unwrap_or_else(Default::default);

    if data.invalid_tokens.contains(token) {
        let error: Error = AuthenticationError::from(config).into();
        return Err((error, req));
    }

    let token_service = JwtService::new(secret);
    match token_service.validate_token(token) {
        Ok(res) => {
            if res {
                Ok(req)
            } else {
                let error = AuthenticationError::from(config).into();
                Err((error, req))
            }
        }
        Err(_) => {
            let error = AuthenticationError::from(config).into();
            Err((error, req))
        }
    }
}
