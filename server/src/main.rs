use axum::Router;
use server::constants::DEV_CONFIG_PATH;
use server::db::{DBConnector, Database, DatabaseError};
use server::requests::routes::create_routes;
use server::toml_env::{Config, DatabaseConfig};
use std::net::SocketAddr;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    #[derive(OpenApi)]
    #[openapi(
        paths(
            server::requests::auth::handle_login,
            server::requests::auth::handle_signup,
            server::requests::jwt::refresh,
            server::requests::items::get_item,
            server::requests::items::get_items,
            server::requests::items::add_item,
            server::requests::items::edit_item,
            server::requests::items::delete_item,
        ),
        components(
            schemas(
                server::models::User,
                server::models::Order,
                server::models::Item,
                server::api::ErrorResponse,
                server::requests::auth::LoginParams,
                server::requests::auth::AuthRes,
                server::requests::auth::SignupParams,
                server::requests::items::GetItemReq,
                server::requests::items::AddItemReq,
                server::requests::items::UpdateItemReq,
                server::requests::items::DeleteItemReq,
                server::requests::items::ItemUpdate,
            )
        ),
        tags(
            (name = "RANS API", description = "REST API for RANS tech stack")
        )
    )]
    struct ApiDoc;

    let parsed_config = Config::parse(DEV_CONFIG_PATH);

    let config = match parsed_config {
        Ok(config) => config,
        Err(err) => {
            eprintln!("Error reading config file: {:?}", err);
            return;
        }
    };

    let db_result = get_db(config.db).await;

    let db: Database = match db_result {
        Ok(db) => db,
        Err(e) => {
            eprintln!("{:?}", e);
            return;
        }
    };

    println!("Successfully connected to database");

    let app: Router = create_routes(db, config.log.path.as_str(), &config.server)
        .await
        .merge(SwaggerUi::new("/api/v1").url("/api-docs/openapi.json", ApiDoc::openapi()));

    let addr: SocketAddr = SocketAddr::from(config.server.socket_addr());
    tracing::info!("listening on {}", addr);
    println!("Server listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("Server failed to start");
}

async fn get_db(config: DatabaseConfig) -> Result<Database, Box<DatabaseError>> {
    let connector: DBConnector = DBConnector {
        db_url: config.get_url(),
        db_name: config.name,
        db_username: config.username,
        db_password: config.password,
    };

    Database::new(connector).await.map_err(|e| e.into())
}
