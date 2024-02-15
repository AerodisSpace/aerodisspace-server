use std::env;

use dotenvy;
use error_mapper::errors::AerodisSpaceError;
use logger::tracing::{error, info};
use scylla::{
    execution_profile::ExecutionProfileHandle, statement::Consistency, transport::errors::NewSessionError, CachingSession,
    ExecutionProfile, Session, SessionBuilder, SessionConfig,
};
pub enum SessionType {
    Local,
    Cloud,
}

pub async fn build_session() -> Result<Session, AerodisSpaceError> {
    let uri = env::var("SCYLLA_URI").expect("SCYLLA_URI must be set");
    let username = env::var("SCYLLA_USERNAME").expect("SCYLLA_USERNAME must be set");
    let password = env::var("SCYLLA_PASSWORD").expect("SCYLLA_PASSWORD must be set");
    let keyspace = env::var("SCYLLA_KEYSPACE").expect("SCYLLA_KEYSPACE must be set");
    
    let session_type: SessionType = match env::var("SCYLLA_SESSION_TYPE").unwrap_or_else(|_| "local".into()).as_str() {
        "local" => SessionType::Local,
        "cloud" => SessionType::Cloud,
        _ => SessionType::Local,
    };
    info!("Connecting to ScyllaDB at {}", uri);

    let session = match session_type {
        SessionType::Local => SessionBuilder::new()
            .known_node(uri)
            .user(username, password)
            .default_execution_profile_handle(profile_builder())
            .build()
            .await
            .map_err(|_err| AerodisSpaceError::Internal(_err.to_string()))?,
        SessionType::Cloud => {
            #[cfg(target_os = "linux")]
            {
                let cloud_config_path = env::var("SCYLLA_CLOUD_CONFIG_PATH").expect("SCYLLA_CLOUD_CONFIG_PATH must be set");
                scylla::CloudSessionBuilder::new(cloud_config_path)?
                    .default_execution_profile_handle(profile_builder())
                    .build()
                    .await?
            }
            #[cfg(not(target_os = "linux"))]
            {
                error!("Cloud session is only supported on Linux");
                panic!("Cloud session is only supported on Linux")
            }
        }
    };
    info!("Session Created and Connected to ScyllaDB");
    session.use_keyspace(keyspace, true).await;
    Ok(session)
}

fn profile_builder() -> ExecutionProfileHandle {
    let consistency = match std::env::var("SCYLLA_CONSISTENCY").unwrap_or_else(|_| "LOCAL_ONE".into()).as_str() {
        "LOCAL_ONE" => Consistency::LocalOne,
        "LOCAL_QUORUM" => Consistency::LocalQuorum,
        "LOCAL_SERIAL" => Consistency::LocalSerial,
        "SERIAL" => Consistency::Serial,
        "ONE" => Consistency::One,
        "TWO" => Consistency::Two,
        "THREE" => Consistency::Three,
        "QUORUM" => Consistency::Quorum,
        "EACH_QUORUM" => Consistency::EachQuorum,
        "ALL" => Consistency::All,
        "ANY" => Consistency::Any,
        _ => Consistency::LocalOne,
    };
    // Set consistency
    let profile = ExecutionProfile::builder().consistency(consistency).request_timeout(None).build();
    let profile_handle = profile.into_handle();
    profile_handle
}
