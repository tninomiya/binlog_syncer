extern crate binlog_syncer;
extern crate diesel;

use self::binlog_syncer::*;
use self::diesel::prelude::*;
use self::models::*;

fn main() {
    use binlog_syncer::schema::replication_connection_configuration::dsl::*;

    let connection = establish_connection();
    let results = replication_connection_configuration
        .load::<ReplicationConnectionConfiguration>(&connection)
        .expect("Error loading posts");

    println!("Displaying {} cfg", results.len());
    for cfg in results {
        println!("{:?}", cfg);
    }
}
