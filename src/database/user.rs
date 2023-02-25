#![allow(dead_code)]
use crate::models::user::{Tenant, TenantClaims};
use crate::utils::strings::clean_string;
use actix_web::{web::Data};
use chrono::{Utc, Duration};
use super::connection::*;
use crate::models::user::Login;
use crate::utils::hash_password::{hash_password, verify_password};

pub enum Error {
    UserNotFound,
    WrongPassword,
    InternalError,
}

pub async fn create_claims_from_login(login: Login, conn: Data<Datab>) -> Result<TenantClaims, Error> {
    let sql = format!("SELECT * FROM tenant:`{}`", login.login);
    
    let mut query = match conn.connection.query(sql).await {
        Ok(query) => query,
        Err(errinho) => {
            dbg!(errinho);
            return Err(Error::WrongPassword)},
    };

    let user: Option<Tenant> = match query.take(0) {
        Ok(value) => value,
        Err(errinho) => {
            dbg!(errinho);
            return Err(Error::InternalError)
        },
    };

    match user {
        Some(user) => {
            
            if !verify_password(login.password, user.password) {
                return Err(Error::WrongPassword);
            }

            return Ok(TenantClaims {
                login: clean_string(user.id),
                apartment: user.apartment,
                floor: user.floor,
                exp: (Utc::now() + Duration::days(7)).timestamp() as usize,
            })
        },
        None => {
            dbg!("n achei nada");
            return Err(Error::UserNotFound)},
    }
}

pub async fn create_user(user: Tenant, conn: Data<Datab>, salt: &String) -> Result<bool, Error> {
    let _new_user: Tenant = match conn.connection.create("tenant").content(Tenant {
        id: user.id,
        password: hash_password(user.password, salt.clone()),
        apartment: user.apartment,
        floor: user.floor,
    }).await {
        Ok(user) => user,
        Err(_) => return Err(Error::InternalError),
    };

    Ok(true)
}