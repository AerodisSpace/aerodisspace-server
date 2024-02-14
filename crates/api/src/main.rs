mod app;
mod database;
mod dto;
mod envs;
mod models;
mod router;
mod utils;
use envs::DEBUG_MODE;
use logger::logger;
use router::{catcher::catcher_router::catcher_router, user::user_router::user_router};


#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    // SCYLLADB KEYSPACE IS CONFIGURED IN BUILD.RS

    dotenvy::dotenv().ok();
    logger();

    let _rocket = rocket::build()
        .mount("/user", user_router())
        .register("/", catcher_router())
        .launch()
        .await?;
    Ok(())
}

fn logger() {
    let write_log: bool = match std::env::var("WRITE_LOG").unwrap().as_str() {
        "true" => true,
        _ => false,
    };
    let level_log = std::env::var("LEVEL_LOG").unwrap();
    logger::log_init(write_log, &level_log);
}
