// PURPOSE: See if the table made by `main.rs`
// persists in a new Docker session.
// (It did a year ago, but now it doesn't. Not sure what changed.)

use typedb_driver::{
    Credentials, DriverOptions, TransactionType, TypeDBDriver,
};
use futures::StreamExt;

fn main() {
    async_std::task::block_on(async {
        println!("Checking for existing TypeDB data...");

        // Connect to TypeDB server
        let driver = TypeDBDriver::new_core(
            TypeDBDriver::DEFAULT_ADDRESS,
            Credentials::new("admin", "password"),
            DriverOptions::new(false, None).unwrap(),
        )
        .await
        .unwrap();

        // Check if our test database exists
        let db_name = "test_db";
        if driver.databases().contains(db_name).await.unwrap() {
            println!("✓ Database '{}' exists", db_name);

            // Connect to the database
            let database = driver.databases().get(db_name).await.unwrap();

            // Open a read transaction to check schema
            let transaction = driver.transaction(database.name(), TransactionType::Read).await.unwrap();

            // Check if "person" entity type exists
            let query_result = transaction.query("match entity $x;").await.unwrap();

            let mut rows = Vec::new();
            let mut stream = query_result.into_rows();
            while let Some(row_result) = stream.next().await {
                if let Ok(row) = row_result {
                    rows.push(row);
                }
            }

            println!("Found {} entity types:", rows.len());

            let mut found_person = false;
            for row in rows {
                if let Ok(Some(concept)) = row.get("x") {
                    let label = concept.get_label();
                    println!("- {}", label);

                    if label == "person" {
                        found_person = true;
                    }
                }
            }

            if found_person {
                println!("✓ 'person' entity type exists");
            } else {
                println!("✗ 'person' entity type not found");
            }

            // Transactions are automatically closed when dropped
            // No explicit close() needed
        } else {
            println!("✗ Database '{}' does not exist", db_name);
        }

        println!("Check complete!");
    })
}
