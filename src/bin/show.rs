extern crate binlog_syncer;

use self::binlog_syncer::*;
use self::models::*;
use sqlx;
use sqlx::Result;

#[tokio::main]
pub async fn main() -> Result<()> {
    let conn = establish_connection().await?;
    let repl_conn_config = sqlx::query_as::<_, ReplicationConnectionConfiguration>(
        "SELECT * FROM replication_connection_configuration",
    )
    .fetch_one(&conn)
    .await?;
    println!("ReplicationConnectionConfiguration: {:?}", repl_conn_config);

    let repl_conn_status = sqlx::query_as::<_, ReplicationConnectionStatus>(
        "SELECT * FROM replication_connection_status",
    )
    .fetch_one(&conn)
    .await?;
    println!("ReplicationConnectionStatus: {:?}", repl_conn_status);

    Ok(())
}
