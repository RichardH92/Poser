use tonic::{transport::Server, Request, Response, Status};

use poser::poser_server::{Poser, PoserServer};
use poser::{EntityPage, GetEntitiesRequest, AddEntitiesRequest, AddEntitiesResponse};

mod service;
mod domain;

/*fn main() {
    let _e = domain::entity::Entity { id: 1, x_coordinate: 0, y_coordinate: 1, z_coordinate: 3 };
    
    //let mut node = node::Node::new();

    //node.addEntities(vec![e]);
    // Statements here are executed when the compiled binary is called

    // Print text to the console
    println!("Hello World!");
}*/

pub mod poser {
    tonic::include_proto!("poser"); // The string specified here must match the proto package name
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    println!("Test 123");

    let addr = "[::1]:50051".parse()?;
    let greeter = PoserImpl::default();

    Server::builder()
        .add_service(PoserServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}

#[derive(Debug, Default)]
pub struct PoserImpl {}

#[tonic::async_trait]
impl Poser for PoserImpl {

    async fn add_entities(
        &self,
        request: Request<AddEntitiesRequest>,
    ) -> Result<Response<AddEntitiesResponse>, Status> {

        let reply = poser::AddEntitiesResponse {
            name: "Hello!".to_string()
        };

        Ok(Response::new(reply))
    }

    async fn get_entities(
        &self,
        request: Request<GetEntitiesRequest>, // Accept request of type HelloRequest
    ) -> Result<Response<EntityPage>, Status> { // Return an instance of type HelloReply
        println!("Got a request: {:?}", request);

        let reply = poser::EntityPage {
            entities: [].to_vec(),
            total: 0,
        };

        Ok(Response::new(reply)) // Send back our formatted greeting
    }
}
