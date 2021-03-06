use chrono::{DateTime, Utc};
use sqlx::FromRow;

#[derive(FromRow, Debug)]
#[sqlx(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct ReplicationConnectionConfiguration {
    pub channel_name: String,
    pub host: String,
    pub port: i32,
    pub user: String,
    pub network_interface: String,
    pub auto_position: String,
    pub ssl_allowed: String,
    pub ssl_ca_file: String,
    pub ssl_ca_path: String,
    pub ssl_certificate: String,
    pub ssl_cipher: String,
    pub ssl_key: String,
    pub ssl_verify_server_certificate: String,
    pub ssl_crl_file: String,
    pub ssl_crl_path: String,
    pub connection_retry_interval: i32,
    pub connection_retry_count: u32,
    pub heartbeat_interval: f32,
    pub tls_version: String,
}

#[derive(FromRow, Debug)]
#[sqlx(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct ReplicationConnectionStatus {
    pub channel_name: String,
    pub group_name: String,
    pub source_uuid: String,
    pub thread_id: Option<u32>,
    pub service_state: String,
    pub count_received_heartbeats: u32,
    // set as Option to handle default value '0000-00-00 00:00:00'
    pub last_heartbeat_timestamp: Option<DateTime<Utc>>,
    pub received_transaction_set: String,
    pub last_error_number: i32,
    pub last_error_message: String,
    // set as Option to handle default value '0000-00-00 00:00:00'
    pub last_error_timestamp: Option<DateTime<Utc>>,
}

#[derive(FromRow, Debug)]
#[sqlx(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct ReplicationApplierStatus {
    pub channel_name: String,
    pub service_state: String,
    pub remaining_delay: Option<u32>,
    pub count_transactions_retries: u32,
}

#[derive(FromRow, Debug)]
#[sqlx(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct ReplicationApplierStatusByWorker {
    pub channel_name: String,
    pub worker_id: u32,
    pub thread_id: Option<u32>,
    pub service_state: String,
    pub last_seen_transaction: String,
    pub last_error_number: i32,
    pub last_error_message: String,
    // set as Option to handle default value '0000-00-00 00:00:00'
    pub last_error_timestamp: Option<DateTime<Utc>>,
}
