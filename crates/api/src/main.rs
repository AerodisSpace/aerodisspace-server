/// Routers for the application
mod router;
/// Server for mount and start the application
mod server;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    server::server::start().await?;
    Ok(())
}
