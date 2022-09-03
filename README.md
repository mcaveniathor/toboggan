# toboggan

[![Rust Report Card](https://rust-reportcard.xuri.me/badge/github.com/mcaveniathor/toboggan)](https://rust-reportcard.xuri.me/report/github.com/mcaveniathor/toboggan)
[![Crates.io](https://img.shields.io/crates/v/toboggan)](https://crates.io/crates/toboggan)
[![docs.rs](https://img.shields.io/docsrs/toboggan)](https://docs.rs/crate/toboggan/latest)

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
