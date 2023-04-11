
// #![deny(warnings, missing_docs)]

// use async_trait::async_trait;
use deadpool::{
    async_trait,
    managed,
};

pub use deadpool::managed::reexports::*;

// use surrealdb;
// use surreadb::dbs::Session;
pub use surrealdb::Error;
pub use surrealdb::kvs::Datastore;
// pub use surrealdb::Datastore;



/// A type alias for using `deadpool::Pool` with `surrealdb::kvs::Datastore`
pub type Pool = managed::Pool<Manager>;

/// A type alias for using `deadpool::PoolError` with `surrealdb::kvs::Datastore`
pub type PoolError = managed::PoolError<Error>;

/// A type alias for using `deadpool::Object` with `surrealdb::kvs::Datastore`
pub type Connection = managed::Object<Datastore>;

type RecycleResult = managed::RecycleResult<Error>;

/// The manager for creating and recyling memcache connections
pub struct Manager {
    path: String,
}

impl Manager {
    /// Create a new manager for the path.
    pub fn new(path: String) -> Self {
        Self { path }
    }
}

#[async_trait]
impl managed::Manager for Manager {
    type Error = surrealdb::Error;
    type Type = surrealdb::kvs::Datastore;

    async fn create(&self) -> Result<Self::Type, Self::Error> {
        println!("Creating a new connection: {}", self.path);
        Ok(Datastore::new(&self.path).await?)
    }

    async fn recycle(&self, _conn: &mut Datastore) -> RecycleResult {
        // TODO: This needs a liveness check of the underlying connection
        Ok(())
    }
}


