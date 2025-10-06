use dashboard::application::services::AccountService;
use dashboard::infrastructure::messaging::RabbitMQClient;
use dashboard::infrastructure::persistence::repositories::AccountRepositoryImpl;
use dashboard::presentation::routes;
use rocket_okapi::openapi_get_routes;
use rocket_okapi::swagger_ui::{make_swagger_ui, SwaggerUIConfig};
use sea_orm::{Database, DatabaseConnection};
use std::env;
use std::sync::Arc;

#[macro_use]
extern crate rocket;

#[get("/")]
fn hello() -> &'static str {
    "Hello, world!"
}

async fn establish_db_connection() -> DatabaseConnection {
    let database_url =
        env::var("DATABASE_URL").expect("DATABASE_URL must be set in environment variables");

    Database::connect(&database_url)
        .await
        .expect("Failed to connect to database")
}

#[launch]
async fn rocket() -> _ {
    dotenv::dotenv().ok();

    let db = establish_db_connection().await;

    let rabbitmq_url =
        env::var("RABBITMQ_URL").expect("RABBITMQ_URL must be set in environment variables");

    let rabbitmq = Arc::new(
        RabbitMQClient::new(&rabbitmq_url)
            .await
            .expect("Failed to connect to RabbitMQ"),
    );

    let account_repository = Arc::new(AccountRepositoryImpl::new(db));
    let account_service = Arc::new(AccountService::new(account_repository));

    let mut builder = rocket::build()
        .manage(rabbitmq)
        .manage(account_service)
        .mount("/", routes![hello])
        .mount(
            "/api/v1",
            openapi_get_routes![
                routes::get_account,
                routes::create_account,
                routes::update_account,
                routes::delete_account,
                routes::post_streammer_message
            ],
        )
        .mount("/api/v1", routes![routes::chat_stream]);

    if env::var("DEVELOP").is_ok() {
        builder = builder.mount(
            "/docs/",
            make_swagger_ui(&SwaggerUIConfig {
                url: "../api/v1/openapi.json".to_owned(),
                ..Default::default()
            }),
        );

    }

    builder
}
