use crate::configs::configs::Settings;
use anyhow::{anyhow, Result};
use surrealdb::sql::{Object, Value};
use surrealdb::{Datastore, Response, Error, Session};
use std::sync::Arc;

#[derive(Clone)]
pub struct SurrealDBRepo {
    pub datastore: Arc<Datastore>,
    pub session: Session
}

impl SurrealDBRepo {
    pub async fn init() -> Result<Self, Error> {
        let settings = Settings::development().expect("Failed to read settings");
        let datastore = Arc::new(Datastore::new(&settings.database.url).await?);
        let session = Session::for_db(&settings.database.ns, &settings.database.db);
        Ok(SurrealDBRepo { session, datastore })
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