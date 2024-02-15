use catalytic::runtime::{create_test_tables, query};

const CREATE_KEYSPACE: &str = "
CREATE KEYSPACE IF NOT EXISTS aerodisspace WITH REPLICATION = {
    'class': 'NetworkTopologyStrategy',
    'replication_factor': 3
};
";

fn main() {
    let _ = dotenvy::dotenv();
    let run_migrate_on_build = std::env::var("SCYLLA_MIGRATE_ON_BUILD").unwrap_or("false".to_string()) == "true";
    if !run_migrate_on_build {
        return;
    }
    // Create the keyspace
    query(CREATE_KEYSPACE, []);

    // EXECUTE THE COMMAND CHARYBDIS-MIGRATE CLI
    let scylla_uri = std::env::var("SCYLLA_URI").expect("SCYLLA_URI must be set");
    let scylla_user = std::env::var("SCYLLA_USERNAME").expect("SCYLLA_USERNAME must be set");
    let scylla_password = std::env::var("SCYLLA_PASSWORD").expect("SCYLLA_PASSWORD must be set");
    let output = std::process::Command::new("migrate")
        .arg("--host")
        .arg(scylla_uri)
        .arg("--keyspace")
        .arg("aerodisspace")
        .arg("--user")
        .arg(scylla_user)
        .arg("--password")
        .arg(scylla_password)
        .arg("-d")
        .output()
        .expect("Failed running migrate charybdis");
}
