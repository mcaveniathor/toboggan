extern crate sled;
extern crate futures;
extern crate tarpc;
#[macro_use] extern crate tracing;

/**
# toboggan

[![Rust Report Card](https://rust-reportcard.xuri.me/badge/github.com/mcaveniathor/toboggan)](https://rust-reportcard.xuri.me/report/github.com/mcaveniathor/toboggan)
![Crates.io](https://img.shields.io/crates/v/toboggan)

## Description
Toboggan is an RPC-based key/value database client and server system built on Google's tarpc library and the sled embedded database. This crate contains client and server binaries using Bincode serialization over TCP, *as well as* a library with the traits, boilerplate, stubs, and utilities to integrate the transport and business logic of your choosing with the RPC interface and server functionality provided by the crate.


## Features
At time of writing, the following operations are supported, with more to come soon.

- NewTree
- Insert
- Get
- GetTreeNames
- GetID (returns a monotonically generated ID)
- Remove


## Installation
`cargo install toboggan`

## Binary Usage
### Server
  You can view the helptext for the server cli using the command `cargo run --bin server -- -h`. 
#### Example
  The command `cargo run --bin server -- -a 10.0.18.135 -p 5050 -d ./db tcp`
  will create/open a database at <current directory>/db and  listen for RPC requests on 10.0.18.135:5050
  
### Client
The client helptext can similarly be viewed with `cargo run --bin client -- -h`.
  
  Helptext for a specific subcommand can  be viewed using `cargo run --bin client -- <subcommand> -h`
#### Example
  To insert the value "Thor M." into the "my_name" key of the "names" tree of the server above, use the following command:
  `cargo run --bin client -- -a 10.0.18.135 -p 5050 insert -k my_name "Thor M." -t names`
*/

#[tarpc::service]
/// The trait used by [tarpc] to generate the client stubs and boilerplate. 
pub trait Database {
    /// Creates a new database tree with the given name, returning Ok without overwriting if one already exists
    async fn new_tree(name: String) -> Result<(),Error>;
    /// Get the tree names saved in the databse
    async fn tree_names() -> Vec<String>;
    /// Generate a monotonic ID
    async fn generate_id() -> Result<u64, Error>;
    /// Insert a key/value pair into the desired tree, returning previous value if one exists
    async fn insert(tree: Option<String>, key: Vec<u8>, value: Vec<u8>) -> Result<Option<Vec<u8>>, Error>;
    /// Retrieve a value from the database if it exists
    async fn get(tree: Option<String>, key: Vec<u8>) -> Result<Option<Vec<u8>>, Error>;
    /// Delete a value, returning the old value if it existed.
    async fn remove(key: Vec<u8>, tree: Option<String>) -> Result<Option<Vec<u8>>, Error>;
}



#[derive(thiserror::Error,Debug, tarpc::serde::Serialize, tarpc::serde::Deserialize)]
#[serde(crate="tarpc::serde")]
/// \[De\]Serializable error type
pub enum Error {
    #[error("RPC error occurred")]
    RpcError(#[from] tarpc::client::RpcError),
    #[error("Database error occurred")]
    DatabaseError(String),
    #[error("IO error occurred: {0}")]
    Io(String),
    #[error("Other error occurred: {0}")]
    Other(String),
}
impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error.to_string())
    }
}

impl From<sled::Error> for Error {
    fn from(error: sled::Error) -> Self {
        Self::DatabaseError(error.to_string())
    }
}



pub mod server;

