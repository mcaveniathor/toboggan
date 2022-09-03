extern crate clap;
use clap::{Parser,Subcommand};
#[macro_use] extern crate tracing;
extern crate tracing_subscriber;

use tarpc::{
    server::{self, incoming::Incoming, Channel},
    tokio_serde::formats::Bincode,
};
use std::{
    sync::Arc,
    path::PathBuf,
    net::{IpAddr,Ipv6Addr},
};
use toboggan_lib::{
    Database,
    Error,
    server::{
        DbServer,
    }
};
use futures::{future,StreamExt};

#[derive(Subcommand)]
enum Command {
    Tcp,
}
#[derive(Parser)]
struct Args {
    #[clap(short)]
    /// Relative or absolute path to the database to be opened or created (default: <current
    /// working directory>/db
    db: Option<PathBuf>,
    #[clap(short)]
    /// Port to listen on
    port: u16,
    #[clap(short,long)]
    /// IP address to listen on (default: ipv6 localhost)
    address: Option<IpAddr>,
    #[clap(subcommand)]
    command: Command,
}


fn main() -> Result<(), Error> {
    let args = Args::parse();
    tracing_subscriber::fmt::init();
    info!("Started logger.");
    run(args).map_err(|e| { error!("{}",e); e })?;
    Ok(())
}


#[tokio::main]
async fn run(args: Args) -> Result<(), Error> {
    let path = args.db.unwrap_or(PathBuf::from("db"));
    let dbserver = DbServer { db: Arc::new(sled::open(&path)?)};
    let server_addr = (args.address.unwrap_or(IpAddr::V6(Ipv6Addr::LOCALHOST)), args.port);
    match args.command {
        Command::Tcp => {
            let mut listener = tarpc::serde_transport::tcp::listen(&server_addr, Bincode::default).await?;
            listener.config_mut().max_frame_length(usize::MAX);
            listener
            // Ignore accept errors.
            .filter_map(|r| future::ready(r.ok()))
            .map(server::BaseChannel::with_defaults)
            // Limit channels to 1 per IP.
            .max_channels_per_key(1, |t| t.transport().peer_addr().unwrap().ip())
            .map(|channel| {
                info!("Incoming connection from {} accepted", channel.transport().peer_addr().unwrap().ip());
                let server = dbserver.clone();
                channel.execute(server.serve())
            })
            // Max 10 channels.
            .buffer_unordered(10)
            .for_each(|_| async {})
            .await;
            }
    }
    Ok(())
}
