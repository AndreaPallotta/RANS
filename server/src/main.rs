use axum::Router;
use server::toml_env::{Config, DatabaseConfig};
use std::net::SocketAddr;
use server::db::{Database, DBConnector};
use server::requests::routes::create_routes;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    // let parse_config = Config::parse("/etc/rans/config.toml");
    let parse_config = Config::parse("../config/config.local.toml");

    let config = match parse_config {
        Ok(config) => config,
        Err(err) => {
            eprintln!("Error reading config file: {:?}", err);
            return;
        }
    };

    let db: Database = get_db(config.db).await;

    let app: Router = create_routes(db, config.log.path.as_str()).await;

    let addr: SocketAddr = SocketAddr::from(config.server.socket_addr());
    tracing::info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("Server failed to start");
}

async fn get_db(config: DatabaseConfig) -> Database {
    let connector: DBConnector = DBConnector {
        db_url: config.get_url(),
        db_name: config.name,
        db_username: config.username,
        db_password: config.password,
    };

    match Database::new(connector).await {
        Ok(db) => db,
        Err(e) => panic!("Failed to connect to database: {:?}", e),
    }
}
