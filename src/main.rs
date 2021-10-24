use tonic::{transport::Server, Request, Response, Status};

use poser::poser_server::{Poser, PoserServer};
use poser::{EntityPage, GetEntitiesRequest, AddEntitiesRequest, AddEntitiesResponse};

use std::sync::{RwLock, Arc, mpsc::channel};

mod service;
mod domain;
mod grpc;

pub mod poser {
    tonic::include_proto!("poser"); // The string specified here must match the proto package name
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    println!("Test 123");

    let addr = "[::1]:50051".parse()?;

    Server::builder()
        .add_service(grpc::controller::init())
        .serve(addr)
        .await?;

    Ok(())
}
