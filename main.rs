use typedb_driver::{
    Connection, DatabaseManager, Options, Session, SessionType, TransactionType,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to the TypeDB server
    let connection = Connection::new_core("localhost:1729").await?;
    println!("Connected to TypeDB server");

    // Access the database management interface
    let databases = DatabaseManager::new(connection.clone());

    // List all databases
    let db_names = databases.all().await?;
    println!("Available databases:");
    for name in db_names {
        println!("- {}", name);
    }

    // Create a test database if it doesn't exist
    let db_name = "test_db";
    if !databases.contains(db_name).await? {
        println!("Creating database '{}'...", db_name);
        databases.create(db_name).await?;
    }

    // Open a session with the database
    let session = Session::new(connection, db_name.to_string(), SessionType::Schema, Options::new()).await?;
    println!("Opened session to database '{}'", db_name);

    // Start a transaction
    let tx = session.transaction(TransactionType::Write, Options::new()).await?;

    // Define a simple schema
    let define_query = "define person sub entity, owns name; name sub attribute, value string;";
    tx.query().define(define_query).await?;
    println!("Defined schema: person entity with name attribute");

    // Commit the transaction
    tx.commit().await?;

    println!("Successfully committed schema changes");

    // Close the session
    session.close().await?;

    Ok(())
}
