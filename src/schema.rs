table! {
    replication_connection_configuration (channel_name) {
        channel_name -> Varchar,
        host -> Varchar,
        port -> Integer,
        user -> Varchar,
        network_interface -> Varchar,
        auto_position -> Varchar,
        ssl_allowed -> Varchar,
        ssl_ca_file -> Varchar,
        ssl_ca_path -> Varchar,
        ssl_certificate -> Varchar,
        ssl_cipher -> Varchar,
        ssl_key -> Varchar,
        ssl_verify_server_certificate -> Varchar,
        ssl_crl_file -> Varchar,
        ssl_crl_path -> Varchar,
        connection_retry_interval -> Integer,
        connection_retry_count -> Integer,
        heartbeat_interval -> Float,
        tls_version -> Varchar,
    }
}

table! {
    replication_connection_status (channel_name) {
        channel_name -> Varchar,
        group_name -> Varchar,
        source_uuid -> Varchar,
        thread_id -> Integer,
        service_state -> Varchar,
        count_received_heartbeats -> Integer,
        last_heartbeat_timestamp -> Timestamp,
        received_transaction_set -> Varchar,
        last_error_number -> Integer,
        last_error_message -> Varchar,
        last_error_timestamp -> Timestamp,
    }
}
