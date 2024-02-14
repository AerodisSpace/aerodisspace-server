use logger::logger::log_init;

use crate::router::{catcher::catcher_router::catcher_router, user::user_router::user_router};

pub async fn start() -> Result<(), rocket::Error> {
    // SCYLLADB KEYSPACE IS CONFIGURED IN BUILD.RS

    dotenvy::dotenv().ok();
    log_config();
    let _rocket = rocket::build()
        .mount("/user", user_router())
        .register("/", catcher_router())
        .launch()
        .await?;
    Ok(())
}

fn log_config() {
    let write_log: bool = match std::env::var("WRITE_LOG").unwrap().as_str() {
        "true" => true,
        _ => false,
    };
    let level_log = std::env::var("LEVEL_LOG").unwrap();
    log_init(write_log, &level_log);
}
