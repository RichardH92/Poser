use tonic::transport::Server;

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
