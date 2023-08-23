use mongodb::{Database, options::ClientOptions, Client};

pub async fn dbconnect() -> mongodb::error::Result<Database> {
    // Parse a connection string into an options struct.
    let mut client_options = ClientOptions::parse("mongodb://root:123456@localhost:27017").await?;

    // Manually set an option.
    client_options.app_name = Some("My App".to_string());

    // Get a handle to the deployment.
    let client = Client::with_options(client_options)?;

    // List the names of the databases in that deployment.
    let db = client.database("crab_db");

    Ok(db)
}