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
use toboggan::{
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
    #[clap(short, default_value_os_t=PathBuf::from("db"))]
    /// Relative or absolute path to the database to be opened or created
    db: PathBuf,
    #[clap(short)]
    /// Port to listen on
    port: u16,
    #[clap(short,long, default_value_t=IpAddr::V6(Ipv6Addr::LOCALHOST))]
    /// IP address to listen on
    address: IpAddr,
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
    let dbserver = DbServer { db: Arc::new(sled::open(&args.db)?)};
    let server_addr = (args.address, args.port);
    //let server_addr = (args.address.unwrap_or(IpAddr::V6(Ipv6Addr::LOCALHOST)), args.port);
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
