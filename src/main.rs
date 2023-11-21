extern crate ipfs_api;

use std::io::{self, Read};
use ipfs_api::IpfsClient;

#[tokio::main]
async fn main() {
    // Connect to the local IPFS daemon
    let client = IpfsClient::default();

    // Get the version of the connected IPFS daemon
    match client.version().await {
        Ok(version) => println!("Connected to IPFS daemon version: {}", version.version),
        Err(e) => eprintln!("Error getting IPFS version: {}", e),
    }

    // Take user input for the data to be added to IPFS
    let mut input = String::new();
    println!("Enter the data to add to IPFS:");
    io::stdin().read_to_string(&mut input).expect("Failed to read input");

    // Add the data to IPFS
    match client.add(io::Cursor::new(input)).await {
        Ok(res) => {
            // Print the hash of the added content
            println!("Added to IPFS: {}", res.hash);
        }
        Err(e) => eprintln!("Error adding to IPFS: {}", e),
    }
}

