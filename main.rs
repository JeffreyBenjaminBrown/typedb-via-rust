use typedb_driver::{
    Credentials, DriverOptions, TransactionType, TypeDBDriver,
};
use futures::StreamExt;

fn main() {
    async_std::task::block_on(async {
        // Connect to TypeDB server with credentials
        let driver = TypeDBDriver::new_core(
            TypeDBDriver::DEFAULT_ADDRESS,
            Credentials::new("admin", "password"),
            DriverOptions::new(false, None).unwrap(),
        )
        .await
        .unwrap();

        // Check for existing database or create a new one
        let db_name = "test_db";
        if !driver.databases().contains(db_name).await.unwrap() {
            println!("Creating database '{}'...", db_name);
            driver.databases().create(db_name).await.unwrap();
        }

        let database = driver.databases().get(db_name).await.unwrap();
        println!("Connected to database: {}", database.name());

        // Define a schema
        let transaction = driver.transaction(database.name(), TransactionType::Schema).await.unwrap();
        let define_query = r#"
        define
          entity person, owns name;
          attribute name, value string;
        "#;

        let result = transaction.query(define_query).await;
        match result {
            Ok(_)      => println!("Schema defined successfully"),
            Err(error) => println!("Error defining schema: {}", error),
        }

        transaction.commit().await.unwrap();
        println!("Schema transaction committed");

        // List all entity types to verify
        let transaction = driver.transaction(database.name(), TransactionType::Read).await.unwrap();
        let query_result = transaction.query("match entity $x;")
	    .await.unwrap();

        let mut rows = Vec::new();
        let mut stream = query_result.into_rows();
        while let Some(row_result) = stream.next().await {
            if let Ok(row) = row_result {
                rows.push(row);
            }
        }

        println!("Found {} entity types:", rows.len());

        for row in rows {
            if let Ok(Some(concept)) = row.get("x") {
                println!("- {}", concept.get_label());
            }
        }

        println!("Test complete!");
    })
}
