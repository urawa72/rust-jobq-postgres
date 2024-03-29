use std::time::Duration;

use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

pub type DB = Pool<Postgres>;

pub async fn connect(database_url: &str) -> Result<DB, crate::Error> {
    PgPoolOptions::new()
        .max_connections(100)
        .max_lifetime(Duration::from_secs(30 * 60))
        .connect(database_url)
        .await
        .map_err(|err| crate::Error::ConnectiongToDatabase(err.to_string()))
}

pub async fn migrate(db: &DB) -> Result<(), crate::Error> {
    match sqlx::migrate!("./migrations").run(db).await {
        Ok(_) => Ok(()),
        Err(err) => Err(err)
    }?;

    Ok(())
}
