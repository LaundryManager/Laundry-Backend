use crate::models::user::{Login, Tenant, TenantClaims};
use anyhow::{anyhow, Result};
use std::collections::BTreeMap;
use surrealdb::sql::{Object, Value};
use surrealdb::{Datastore, Response, Session};

pub type DB = (Datastore, Session);

pub async fn new_session() -> DB {
    let datastore = Datastore::new("file://temp.db").await;
    let session = Session::for_db("my_ns", "mydb");
    match datastore {
        Ok(datastore) => (datastore, session),
        Err(e) => panic!("Error: {}", e),
    }
}

pub fn into_iter_types(ress: Vec<Response>) -> Result<impl Iterator<Item = Result<Object>>> {
    let res = ress.into_iter().next().map(|x| x.result).transpose()?;

    match res {
        Some(Value::Array(arr)) => {
            let it = arr.into_iter().map(|v| match v {
                Value::Object(object) => Ok(object),
                _ => Err(anyhow!("Expected object")),
            });
            Ok(it)
        }
        _ => Err(anyhow!("Expected array")),
    }
}

pub async fn do_email_exist(user: String) -> Result<bool> {
    let (ds, ses): &DB = &new_session().await;
    let sql = "SELECT * FROM user WHERE login = $email LIMIT 1";
    let email: BTreeMap<String, Value> = [("email".into(), user.into())].into();

    let response = ds.execute(sql, ses, Some(email), false).await?;
    let res = into_iter_types(response)?.next().transpose()?;

    match res {
        Some(_) => Ok(false),
        None => Ok(true),
    }
}

pub async fn create_user(user: Tenant) -> Result<bool> {
    match do_email_exist(user.login.clone()).await? {
        true => {
            let (ds, ses): &DB = &new_session().await;
            let sql = "CREATE user CONTENT $user";
            let name: BTreeMap<String, Value> = [
                ("login".into(), user.login.into()),
                ("password".into(), user.password.into()),
                ("apartment".into(), user.apartment.into()),
                ("floor".into(), user.floor.into()),
            ]
            .into();

            let vars: BTreeMap<String, Value> = [("user".into(), name.into())].into();

            let ress = ds.execute(sql, ses, Some(vars), false).await?;
            match into_iter_types(ress)?
                .next()
                .transpose()?
                .and_then(|x| x.get("id").map(|x| x.to_string()))
            {
                Some(id) => {
                    dbg!(id);
                    Ok(true)
                }
                None => Ok(false),
            }
            // Ok(true)
        }
        false => Ok(false),
    }
}

pub async fn verify_password(user: Login) -> Result<bool> {
    let (ds, ses): &DB = &new_session().await;

    let sql = "SELECT password FROM user WHERE login = $email LIMIT 1";

    let vars: BTreeMap<String, Value> = [("email".into(), user.login.into())].into();

    let ress = ds.execute(sql, ses, Some(vars), false).await?;
    dbg!(&ress);
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

pub async fn get_user_claims(login: String) -> Result<TenantClaims> {
    let (ds, ses): &DB = &new_session().await;

    let sql = "SELECT login, apartment, floor FROM user WHERE login = $email LIMIT 1";

    let vars: BTreeMap<String, Value> = [("email".into(), login.into())].into();

    let ress = ds.execute(sql, ses, Some(vars), false).await?;
    let strings = into_iter_types(ress)?
        .next()
        .transpose()
        .unwrap();
    
    match strings {
        Some(obj) => {
            Ok(extract_infos(obj).await)
        }, 
        None => {
            Err(anyhow!("No user found"))
        },
    }
}

pub async fn extract_infos(value: Object) -> TenantClaims {
    let apartment: i32 = match value.get("apartment").map(|apartment| apartment.to_number().to_int()) { 
        Some(ap) => ap as i32,
        None => 0,
    };
    let login = match value.get("login").map(|login| login.to_string()) {
        Some(lg) => lg.replace('"', ""),
        None => "".to_string(),
    };
    let floor: i32 = match value.get("floor").map(|floor| floor.to_number().to_int()) {
        Some(fl) => fl as i32,
        None => 0,
    };

   TenantClaims { login, apartment, floor } 
}

pub async fn show_all() -> Result<()> {
    let (ds, ses): &DB = &new_session().await;
    let sql = "SELECT * FROM user";
    let ress = ds.execute(sql, ses, None, false).await?;
    dbg!(&ress);
    Ok(())
}
