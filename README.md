### ipfs_rust

This project allows you to upload files to the InterPlanetary File System (IPFS) using Rust.

### Prerequisites

* Ubuntu operating system
* IPFS installed (`sudo apt install ipfs`)
* Rust and Cargo installed (`sudo apt install rustc cargo`)

### Usage

1. **Initialize IPFS:**
    ```bash
    ipfs init
    ```
2. **Start IPFS daemon:**
    ```bash
    ipfs daemon
    ```
3. **Clone this repository:**
    ```bash
    git clone https://github.com/shubham-kshetre/ipfs_rust.git && cd ipfs_rust
    ```
4. **Build and run the code:**
    ```bash
    cargo run
    ```
    Provide path of the file like `/home/user/Downloads/image.png`

### Accessing the File

Once the upload is complete, you can access the file hosted on IPFS using:

**Locally:**

Open your web browser and enter the following URL in the address bar, replacing `QmXQVydQJDW1dETa7T549qwDcPK4Rzw3eoZP58wAWvwmaE` with the hash displayed after running the code:

```
http://localhost:8080/ipfs/QmXQVydQJDW1dETa7T549qwDcPK4Rzw3eoZP58wAWvwmaE
```

**Publicly:**

The file is now accessible on the IPFS network. Share the hash (`QmXQVydQJDW1dETa7T549qwDcPK4Rzw3eoZP58wAWvwmaE`) with anyone, and they can access the file using any IPFS client or gateway.

### Additional Notes

* This README assumes you have basic knowledge of IPFS and command-line usage.
* You can upload different file types by replacing `data.txt` with the desired file name.
* Feel free to contribute or suggest improvements to this project!

## Further Information

* IPFS documentation: [https://docs.ipfs.io/](https://docs.ipfs.io/)
* Rust documentation: [https://doc.rust-lang.org/](https://doc.rust-lang.org/)
* ipfs_rust code: [https://github.com/shubham-kshetre/ipfs_rust.git](https://github.com/shubham-kshetre/ipfs_rust.git)

