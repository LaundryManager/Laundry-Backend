
use anyhow::{anyhow, Result};
use std::collections::BTreeMap;
use surrealdb::sql::{Object, Value, thing};
use surrealdb::{Datastore, Response, Session};
use crate::models::user::{Tenant, Login};

pub type DB = (Datastore, Session);

// create a new session that return DB
pub async fn new_session() -> DB {
    let datastore = Datastore::new("file://temp.db").await;
    let session = Session::for_db("my_ns", "mydb");
    match datastore {
        Ok(datastore) => (datastore, session),
        Err(e) => panic!("Error: {}", e),
    }
    
}

pub async fn show_all() -> Result<()>{
    let (ds, ses): &DB = &new_session().await;
    let sql = "SELECT * FROM user";
    let ress = ds.execute(sql, ses, None, false).await?;
    dbg!(&ress);
    Ok(())
}

pub async fn create_user(user: Tenant) -> Result<String> {
    let (ds, ses): &DB = &new_session().await;
    let sql = "CREATE user CONTENT $user";

    let name: BTreeMap<String, Value> = [
        ("login".into(), user.login.into()),
        ("password".into(), user.password.into()),
        ("apartment".into(), user.apartment.into()),
        ("floor".into(), user.floor.into()),

    ]
    .into();

    let vars: BTreeMap<String, Value> = [
        ("user".into(), name.into()),
    ].into(); 

    let ress = ds.execute(sql, ses, Some(vars), false).await?;

    into_iter_types(ress)?
        .next()
        .transpose()?
        .and_then(|x| x.get("id").map(|x| x.to_string()))
        .ok_or_else(|| anyhow!("No id"))    
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
        },
        _ => Err(anyhow!("Expected array")),
    }
}

pub async fn verify_password(user: Login) -> Result<bool> {
    let (ds, ses): &DB = &new_session().await;

    let sql = "SELECT password FROM user WHERE login = $email LIMIT 1";

    let vars: BTreeMap<String, Value> = [
        ("email".into(), user.login.into()),
    ].into();

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
