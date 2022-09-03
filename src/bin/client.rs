use clap::{Parser,Subcommand};
use std::{net::{IpAddr,Ipv6Addr}, time::Duration};
use tarpc::{client, context, tokio_serde::formats::Bincode};
use tokio::time::sleep;
use tracing::{Instrument,info,error};
use toboggan::{Error,DatabaseClient};

#[derive(Subcommand)]
pub enum Command {
    /// Create a new named keyspace in the database if it does not already exist
    NewTree {
        #[clap(short,long)]
        /// The name of the tree to be created
        tree_name: String
    },
    /// Insert a value into the database, optionally into the named tree
    Insert {
        #[clap(short,long)]
        /// The tree to insert into, the default (unnamed) tree if not provided
        tree: Option<String>,
        #[clap(short,long)]
        key: String,
        #[clap(short,long)]
        value: String,
    },

   /// Return a monotonically-generated u64 ID from the database
    GenerateID,

    /// Returns a list of the trees in the database.
    GetTreeNames,

    /// Remove a value from the database, returning the previous value if there is one.
    Remove {
        #[clap(short,long)]
        /// The tree to insert into, the default (unnamed) tree if not provided
        tree: Option<String>,

        /// The key to be removed
        #[clap(short,long)]
        key: String,
    }
}
#[derive(Parser)]
struct CliArgs {
    #[clap(short)]
    /// Port to connect on
    port: u16,
    #[clap(short,long, default_value_t=IpAddr::V6(Ipv6Addr::LOCALHOST))]
    /// IP address to connect to
    address: IpAddr,
    #[clap(subcommand)]
    command: Command,
}



fn main() -> Result<(), Error> {
    tracing_subscriber::fmt::init();
    run().map_err(|e| { error!("{}",e); e })?;
    Ok(())
}

#[tokio::main]
async fn run() -> Result<(),  Error> {
    let args = CliArgs::parse();
    let server_address = (args.address, args.port);
    let transport = tarpc::serde_transport::tcp::connect(server_address, Bincode::default);
    let client = DatabaseClient::new(client::Config::default(), transport.await?).spawn();
    match args.command {
        Command::NewTree { tree_name } => {
            let _newtree = async move {
                let resp = client.new_tree(context::current(), tree_name).await;
                resp
            }
            .instrument(tracing::debug_span!("New Tree"))
            .await?;
            info!("Tree either already exists or was created successfully.");

        },
        Command::Insert { tree, key, value } => {
            let prev = async move {
                let resp = client.insert(context::current(), tree, key.as_bytes().to_vec(), value.as_bytes().to_vec()).await;
                resp
            }
            .instrument(tracing::debug_span!("Insert"))
            .await??;
            match prev {
                Some(val) => {
                    info!("Previous value: {}", String::from_utf8(val).unwrap());
                }
                _ => {
                    info!("No previous value for the key provided.");
                }
            }
        },
        Command::GenerateID => {
            let id = async move {
                let resp = client.generate_id(context::current()).await;
                resp
            }.instrument(tracing::debug_span!("Generate ID"))
            .await??;
            info!("ID: {}", id);
        },
        Command::GetTreeNames => {
            let names = async move {
                let resp = client.tree_names(context::current()).await;
                resp
            }
            .instrument(tracing::debug_span!("GetTreeNames"))
            .await?;
            info!("Tree Names: {:?}", names);
        },
        Command::Remove { tree, key } => {
            let prev = async move {
                let resp = client.remove(context::current(), key.as_bytes().to_vec(), tree).await;
                resp
            }
            .instrument(tracing::debug_span!("Remove"))
            .await??;
            match prev {
                Some(val) => {
                    info!("Previous value: {}", String::from_utf8(val).unwrap());
                }
                _ => {
                    info!("No previous value for the key provided.");
                }
            }

        }
    }
    // Let logger finish up
    sleep(Duration::from_micros(1)).await;
    Ok(())
}





