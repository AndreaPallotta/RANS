use arangors::{
    uclient::surf::SurfClient, ArangoError, Connection, Database as ArangoDatabase,
    GenericConnection,
};

#[derive(Clone)]
pub struct Database {
    pub arango_db: ArangoDatabase<SurfClient>,
}

pub struct DBConnector {
    pub db_url: String,
    pub db_name: String,
    pub db_username: String,
    pub db_password: String,
}

#[derive(Debug)]
pub enum DatabaseError {
    ConnectionError(String),
    ArangoError(ArangoError),
}

impl From<ArangoError> for DatabaseError {
    fn from(error: ArangoError) -> Self {
        DatabaseError::ArangoError(error)
    }
}

impl Database {
    pub async fn new(connector: DBConnector) -> Result<Self, DatabaseError> {
        let arango_conn: GenericConnection<SurfClient> = Connection::establish_basic_auth(
            &connector.db_url,
            &connector.db_username,
            &connector.db_password,
        )
        .await
        .map_err(|err| {
            DatabaseError::ConnectionError(format!(
                "Failed to connect to database {}",
                err.to_string()
            ))
        })?;

        let arango_db: ArangoDatabase<SurfClient> =
            arango_conn.db(&connector.db_name).await.map_err(|err| {
                DatabaseError::ConnectionError(format!(
                    "Failed to connect to database {}",
                    err.to_string()
                ))
            })?;

        Ok(Database { arango_db })
    }
}

pub trait ArangoProvider {
    fn get_db(&self) -> &ArangoDatabase<SurfClient>;
}

impl ArangoProvider for Database {
    fn get_db(&self) -> &ArangoDatabase<SurfClient> {
        &self.arango_db
    }
}
