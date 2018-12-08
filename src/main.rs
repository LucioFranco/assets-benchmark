#![feature(duration_as_u128)]

use assets_benchmark::{transfer_tcp, transfer_uds};
use std::time::Instant;

fn main() {
    let addr = "127.0.0.1:0".parse().unwrap();

    println!("Starting TCP benchmark");

    let instant = Instant::now();
    transfer_tcp(&addr);
    let total_time_tcp = Instant::now().duration_since(instant);

    println!("TCP took: {}ms", total_time_tcp.as_millis());

    let path = format!("/tmp/assets-benchmark-{}", uuid::Uuid::new_v4().to_string());

    println!("Starting UDS benchmark");
    let instant = Instant::now();
    transfer_uds(path);
    let total_time_uds = Instant::now().duration_since(instant);

    println!("UDS took: {}ms", total_time_uds.as_millis());
}
