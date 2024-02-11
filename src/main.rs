extern crate ipfs_api;

use std::io;
use std::fs::File;
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

    // Take user input for the path of the file to be added to IPFS
    let mut file_path = String::new();
    println!("Enter the path of the file to add to IPFS:");
    io::stdin().read_line(&mut file_path).expect("Failed to read input");

    // Trim the newline character from the input
    let file_path = file_path.trim();

    // Open the file
    let file = match File::open(&file_path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Error opening file: {}", e);
            return;
        }
    };

    // Add the file to IPFS
    match client.add(file).await {
        Ok(res) => {
            // Print the hash of the added content
            println!("Added to IPFS: {}", res.hash);
        }
        Err(e) => eprintln!("Error adding to IPFS: {}", e),
    }
}
