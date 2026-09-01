//! Manual smoke test for vpp-api-statclient: connects to a real VPP
//! stats-segment socket and dumps a handful of entries.
//!
//! This needs a running VPP with a live stats socket, so it isn't part of
//! `cargo test` (no VPP runs in CI). Run it by hand against a real VPP,
//! from the workspace root:
//!
//!   cargo run -p vpp-api-statclient --example vpp-api-statclient-test -- \
//!       [/run/vpp/stats.sock]
//!
//! (defaults to vpp_api_statclient::DEFAULT_STAT_SEGMENT_SOCKET if no path
//! is given).

use vpp_api_statclient::{StatClient, DEFAULT_STAT_SEGMENT_SOCKET};

fn main() {
    let socket_path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| DEFAULT_STAT_SEGMENT_SOCKET.to_string());

    println!("Connecting to {}...", socket_path);
    let mut client = StatClient::connect(&socket_path)
        .unwrap_or_else(|e| panic!("failed to connect to {}: {}", socket_path, e));

    println!("Dumping stats directory...");
    let entries = client.dump_all().expect("dump_all failed");

    println!("Got {} entries; showing up to 20:", entries.len());
    for entry in entries.iter().take(20) {
        println!("  {} ({:?}) = {:?}", entry.name, entry.dir_type, entry.value);
    }
}
