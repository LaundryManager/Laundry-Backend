use crate::configs::configs::Settings;
use surrealdb::engine::remote::ws::{Ws, Client};
use surrealdb::opt::auth::Root;
use surrealdb::{Error, Surreal};

pub struct Datab {
    pub connection: Surreal<Client>
}

impl Datab {
    pub async fn init() -> Result<Self, Error> {
        let datastore = Surreal::new::<Ws>("localhost:8000").await?;
            datastore.signin(Root {
                username: "root",
                password: "root",
            }).await?;
            datastore.use_ns("namespace").use_db("database").await?;
            Ok(Datab { connection: datastore })
    }
}