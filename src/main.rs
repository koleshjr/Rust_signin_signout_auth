// Import necessary modules and crates for our application
mod config;
mod db;

use actix_web::{get, middleware::Logger, web , App, HttpResponse, HttpServer, Responder};
use config::Config;
use db::DBClient;
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;


// define the AppState struct that holds the configuration (env) and the database client (db_client)
#[derive(Debug, Clone)]
pub struct AppState {
    pub env: Config,
    pub db_client: DBClient,
}


#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //set the default logging level and load environment variables from the .env file
    if std::env::var_os("RUST_LOG").is_none(){
        std::env::set_var("RUST_LOG", "actix_web=info");
    }

    dotenv().ok();
    // initialize logging using env_logger::init()
    env_logger::init();

    //initialize config by reafing environment variables from the runtime
    let config = Config::init();

    //A postgres connection pool (pool) is created with a maximum of 10 connections
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&config.database_url)
        .await?;

    // an instance of db client is created to manage the connection pool
    let db_client = DBClient::new(pool);

    // The AppState is insantiated with the configuration and the database client
    let app_state: AppState = AppState { env: config.clone(), db_client: db_client };

    println! (
        "{}",
        format! ("Server is running on http://localhost:{}", config.port)
    );

    // start the HTTP server using HttpServer::new(), with route handlers and middleware, the server startup message is printed
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .wrap(Logger::default())
            .service(health_check_handler)

    })
    .bind(("0.0.0.0", config.port))?
    .run()
    .await?;

    Ok(())
}

#[get("/api/healthchecker")]
async fn health_check_handler() -> impl Responder {
    const MESSAGE: &str = "Complete Restful API in rust";

    HttpResponse::Ok().json(serde_json::json!(
        {"status": "success",
        "message": MESSAGE}
    ))
}