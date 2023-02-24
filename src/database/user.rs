#![allow(dead_code)]
use crate::models::user::{Login, Tenant, TenantClaims};
use std::collections::BTreeMap;
use actix_web::web::Data;
use surrealdb::sql::{Object, Value};
use chrono::{Utc, Duration};
use anyhow::{anyhow, Result};
use super::connection::*;

pub async fn get_user_claims(login: String, conn: Data<SurrealDBRepo>) -> Result<TenantClaims> {
    let sql = format!("SELECT * FROM user:`{login}`");

    let ress = conn.datastore.execute(&sql, &conn.session, None, false).await?;
    dbg!(&ress);
    let strings = into_iter_types(ress)?
        .next()
        .transpose()
        .unwrap();
    
    match strings {
        Some(obj) => {
            Ok(generate_claim(obj).await)
        }, 
        None => {
            Err(anyhow!("No user found"))
        },
    }
}


pub async fn generate_claim(value: Object) -> TenantClaims {
    let id = value.get("id")
    .ok_or("id not found")
    .and_then(|obj| obj.to_owned().record().ok_or("obj has no id"))
    .map(|val| val.id)
    .unwrap_or("".into());

    let apartment: i32 = match value.get("apartment").map(|apartment| apartment.to_number().to_int()) { 
        Some(ap) => ap as i32,
        None => 0,
    };

    let floor: i32 = match value.get("floor").map(|floor| floor.to_number().to_int()) {
        Some(fl) => fl as i32,
        None => 0,
    };

    let exp: usize = (Utc::now() + Duration::days(7)).timestamp() as usize; 
   TenantClaims { login: id.to_raw(), apartment, floor, exp } 
}

pub async fn verify_password(user: Login, conn: Data<SurrealDBRepo>) -> Result<bool> {
    let sql = format!("SELECT password FROM user:`{}`", user.login);

    let ress = conn.datastore.execute(&sql, &conn.session, None, false).await?;
    let db_pass = into_iter_types(ress)?
        .next()
        .transpose()?
        .and_then(|x| x.get("password").map(|x| x.to_string()))
        .ok_or_else(|| anyhow!("no password"));

    let password_quotes_removed = db_pass?.replace('"', "");
    match argon2::verify_encoded(&password_quotes_removed, user.password.as_bytes()) {
        Ok(true) => Ok(true),
        Ok(false) => Ok(false),
        Err(_) => Ok(false),
    }
}

pub async fn create_user(user: Tenant, conn: Data<SurrealDBRepo>) -> Result<bool> {
    match do_email_exist(user.login.clone(), conn.clone()).await? {
        true => {
            let sql = format!("CREATE user:`{}` CONTENT $user", user.login);
            let name: BTreeMap<String, Value> = [
                ("password".into(), user.password.into()),
                ("apartment".into(), user.apartment.into()),
                ("floor".into(), user.floor.into()),

            ]
            .into();

            let vars: BTreeMap<String, Value> = [("user".into(), name.into())].into();

            let ress = conn.datastore.execute(&sql, &conn.session, Some(vars), false).await?;
            dbg!(&ress);
            let info =  into_iter_types(ress)?
                .next()
                .transpose()?
                .and_then(|x| x.get("id").map(|x| x.to_string()));
            
            dbg!(&info);

            match info {
                Some(id) => {
                    dbg!(id);
                    Ok(true)
                }
                None => Ok(false),
            }
        }
        false => Ok(false),
    }
}

pub async fn do_email_exist(user: String, surreal: Data<SurrealDBRepo>) -> Result<bool> {
    let sql = format!("SELECT * FROM user:`{}`", user);
    
    let response = surreal.datastore.execute(&sql, &surreal.session, None, false).await?;
    let res = into_iter_types(response)?.next().transpose()?;

    match res {
        Some(_) => Ok(false),
        None => Ok(true),
    }
}
pub async fn show_all(conn: Data<SurrealDBRepo>) -> Result<()> {
    let sql = "SELECT * FROM user";
    let ress = conn.datastore.execute(sql, &conn.session, None, false).await?;
    let all = into_iter_types(ress);
    for item in all? {
        dbg!(item?);
        // TryInto::<Tenant>::try_into(item?)?
    }
    Ok(())
}
