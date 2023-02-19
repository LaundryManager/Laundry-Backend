use crate::models::user::TenantClaims;
use std::future::{ Ready, ready };
use actix_web::{
    FromRequest,
    Error as ActixWebError,
    error::ErrorUnauthorized,
    HttpRequest,
    dev::Payload,
    http::header::HeaderValue,
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
    id: String,
}


impl FromRequest for AuthenticationToken {
    type Error = ActixWebError;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {

    // Need validations!
	let authorization_header_option: Option<&HeaderValue> = req.headers().get(actix_web::http::header::AUTHORIZATION);
	if authorization_header_option.is_none() { return ready(Err(ErrorUnauthorized("No authentication token sent!"))); }
	let authentication_token: String = authorization_header_option.unwrap().to_str().unwrap_or("").to_string();
	if authentication_token.is_empty() { return ready(Err(ErrorUnauthorized("Authentication token has foreign chars!"))) }

    // TODO: Validate if the token is bearer.

    let token_value = match authentication_token.split(' ').last() {
        Some(token) => token,
        None => "",
    };

	let token_result: Result<TokenData<TenantClaims>, JwtError> = decode::<TenantClaims>(
        &token_value,
	    &DecodingKey::from_secret("secret".as_ref()),
	    &Validation::new(Algorithm::HS256),
	);

	match token_result {
	    Ok(token) => ready(Ok(AuthenticationToken { id: token.claims.login })),
	    Err(_e) => ready(Err(ErrorUnauthorized("Invalid authentication token sent!"))),
	}
    }
}