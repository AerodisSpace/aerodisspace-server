use database::scylla::CachingSession;
use tokio::sync::OnceCell;

pub static SCYLLADB_SESSION: OnceCell<CachingSession> = OnceCell::const_new();
