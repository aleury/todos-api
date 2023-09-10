use std::sync::Arc;

use async_trait::async_trait;
use sqlx::{Connection, SqlitePool};

use crate::error::Error;

pub type DynPinger = Arc<dyn Pinger + Send + Sync>;

#[async_trait]
pub trait Pinger {
    async fn ping(&self) -> Result<(), Error>;
}

pub struct SqlitePinger {
    dbpool: SqlitePool,
}

impl SqlitePinger {
    pub fn new(dbpool: SqlitePool) -> Self {
        Self { dbpool }
    }
}

#[async_trait]
impl Pinger for SqlitePinger {
    async fn ping(&self) -> Result<(), Error> {
        let mut conn = self.dbpool.acquire().await?;

        conn.ping().await.map_err(Into::into)
    }
}
