use crate::configs::configs::{Settings, DatabaseConfig};
use surrealdb::engine::remote::ws::{Ws, Client};
use surrealdb::opt::auth::Root;
use surrealdb::{Error, Surreal};

pub struct Datab {
    pub connection: Surreal<Client>
}

impl Datab {
    pub async fn init() -> Result<Self, Error> {
        let settings: DatabaseConfig = Settings::development().unwrap().database;
        let datastore = Surreal::new::<Ws>(settings.url).await?;
            datastore.signin(Root {
                username: &settings.username,
                password: &settings.password,
            }).await?;
            datastore.use_ns(settings.namespace).use_db(settings.database).await?;
            Ok(Datab { connection: datastore })
    }
}