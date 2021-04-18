#[derive(Queryable, Debug)]
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

#[derive(Queryable, Debug)]
pub struct ReplicationConnectionStatus {
    pub channel_name: String,
    pub group_name: String,
    pub source_uuid: String,
    pub thread_id: Option<u32>,
    pub service_state: String,
    pub count_reveived_heartbeats: u32,
    pub last_heartbeat_timestamp: String,
    pub received_transaction_set: String,
    pub last_error_number: i32,
    pub last_errer_message: String,
    pub last_error_timestamp: String,
}
