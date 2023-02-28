use crate::models::user::TenantClaims;
use crate::configs::configs::Settings;
use std::future::{ Ready, ready };
use actix_web::{
    FromRequest,
    Error as ActixWebError,
    error::ErrorUnauthorized,
    HttpRequest,
    dev::Payload,
    http::header::HeaderValue, web::Data,
};
use jsonwebtoken::{
    TokenData,
    Algorithm,
    Validation,
    DecodingKey,
    errors::Error as JwtError,
    decode,
};
use serde::{ Serialize, Deserialize };

#[derive( Debug, Serialize, Deserialize)]
pub struct AuthenticationToken {
    pub id: TenantClaims,
}

impl FromRequest for AuthenticationToken {
    type Error = ActixWebError;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
    let app_data = req.app_data::<Data<Settings>>().unwrap();
    dbg!(&app_data.secret.jwt_secret);
    // Need validations!
	let authorization_header_option: Option<&HeaderValue> = req.headers().get(actix_web::http::header::AUTHORIZATION);
	if authorization_header_option.is_none() { return ready(Err(ErrorUnauthorized("No authentication token sent!"))); }
	let authentication_token: String = authorization_header_option.unwrap().to_str().unwrap_or("").to_string();
	if authentication_token.is_empty() { return ready(Err(ErrorUnauthorized("Authentication token has foreign chars!"))) }

    // TODO: Validate if the token is bearer.

    let token_value = authentication_token.split(' ').last().unwrap_or("");

	let token_result: Result<TokenData<TenantClaims>, JwtError> = decode::<TenantClaims>(
        token_value,
	    &DecodingKey::from_secret(app_data.secret.jwt_secret.as_ref()),
	    &Validation::new(Algorithm::HS256),
	);

	match token_result {
	    Ok(token) => ready(Ok(AuthenticationToken { id: token.claims })),
	    Err(_e) => ready(Err(ErrorUnauthorized("Invalid authentication token sent!"))),
	}
    }
}
