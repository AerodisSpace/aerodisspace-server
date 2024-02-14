/// Logic for the APPLICATION
mod app;
/// Configuration of the database connection
mod database;
/// Data Transfer Objects or Response Objects
mod dto;
/// Environment variables and configurations
mod envs;
/// Models for the database (TABLES AND UDTs(UDT = User Defined Types))
mod models;
/// Routers for the application
mod router;
/// Server for mount and start the application
mod server;
/// Utilities for the application
mod utils;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    server::server::start().await?;
    Ok(())
}
