use {
    tokio_postgres::{Client, Error, NoTls},
};

pub struct PostgresConnection {
    pub client: Client,
}

impl PostgresConnection {
    pub async fn connect(database_url: &str) -> Result<Self, Error> {
        let (client, connection) = tokio_postgres::connect(
            database_url, NoTls,
        ).await?;

        tokio::spawn(async {
            if let Err(e) = connection.await {
                eprintln!("Postgres connection error: {}", e);
            }
        });
        Ok(Self { client })
    }
}

const TABLE_NAMES: [&str; 2] = ["transaction", "batches"];

pub async fn create_table(client: &PostgresConnection) {
    let transaction_table = format!(
        "CREATE TABLE IF NOT EXISTS {} (
            blocktime BIGINT,
            meta VARCHAR,
            slot BIGINT,
            transaction VARCHAR
        )", TABLE_NAMES[0]
    );

    if let Err(err) = client.client.execute(&transaction_table, &[]).await {
        println!("Error creating table '{}': {}", TABLE_NAMES[0], err);
    } else {
        println!("Table '{}' created successfully  ✅   ", TABLE_NAMES[0]);
    }


    let batches_table = format!(
        "CREATE TABLE IF NOT EXISTS {} (
            from_txn VARCHAR[],
            to_txn VARCHAR[],
            amounts VARCHAR[],
            signature VARCHAR[],
            sender_balances VARCHAR[],
            receiver_balances VARCHAR[],
            messages VARCHAR[]
        )", TABLE_NAMES[1]
    );

    if let Err(err) = client.client.execute(&batches_table, &[]).await {
        println!("Error creating table '{}': {}", TABLE_NAMES[1], err);
    } else {
        println!("Table '{}' created successfully  ✅   ", TABLE_NAMES[1]);
    }
}








