#![allow(unused_imports, unused_variables)]

use futures::prelude::*;
use tarpc::{
    client, context,
    server::{self, Channel},
};

// This is the service definition. It looks a lot like a trait definition.
// It defines one RPC, hello, which takes one arg, name, and returns a String.
#[tarpc::service]
pub trait Oliana {
    /// Returns a greeting for name.
    async fn hello(name: String) -> String;
}

// This is the type that implements the generated World trait. It is the business logic
// and is used to start the server.
#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct OlianaServer {
    pub client_socket: std::net::SocketAddr,
    pub some_data: String,
}

impl Oliana for OlianaServer {
    async fn hello(self, _: context::Context, name: String) -> String {
        format!("Hello, {name}!")
    }
}
