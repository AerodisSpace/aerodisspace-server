use database::{scylla::CachingSession, scylladb::connection::build_session};
use logger::logger::log_init;

use crate::router::{aircraft::aircraft_router::aircract_router, catcher::catcher_router::catcher_router, user::user_router::user_router};

use super::shared::SCYLLADB_SESSION;

pub async fn start() -> Result<(), rocket::Error> {
    // SCYLLADB KEYSPACE IS CONFIGURED IN BUILD.RS

    dotenvy::dotenv().ok();
    
    let session = CachingSession::from(build_session().await.unwrap(), 1000);
    SCYLLADB_SESSION.set(session).expect("Error setting scylladb session");

    log_config();
    let _rocket = rocket::build()
        .mount("/user", user_router())
        .mount("/aircraft", aircract_router())
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
