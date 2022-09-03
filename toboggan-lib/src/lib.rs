extern crate sled;
extern crate futures;
extern crate tarpc;
#[macro_use] extern crate tracing;


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
