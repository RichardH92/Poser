use tonic::{transport::Server, Request, Response, Status};
use crate::poser::poser_server::{Poser, PoserServer};
use crate::poser::{EntityPage, GetEntitiesRequest, AddEntitiesRequest, AddEntitiesResponse};
use crate::service::service_impl::ServiceImpl;
use crate::service::Service;
use crate::grpc::mapper;

use std::sync::{RwLock, Arc, mpsc::channel};

pub struct PoserImpl {
    service: RwLock<ServiceImpl>
}

pub fn init() -> PoserServer<PoserImpl> {
    return PoserServer::new(PoserImpl{
        service: RwLock::new(Service::new())
    });
}

#[tonic::async_trait]
impl Poser for PoserImpl {

    async fn add_entities(
        &self,
        request: Request<AddEntitiesRequest>,
    ) -> Result<Response<AddEntitiesResponse>, Status> {

        let addEntitiesReq = request.into_inner();

        let entities = mapper::mapAddEntitiesRequestToDomain(addEntitiesReq);

        let reply = AddEntitiesResponse {
            name: "Hello!".to_string()
        };

        Ok(Response::new(reply))
    }

    async fn get_entities(
        &self,
        request: Request<GetEntitiesRequest>, // Accept request of type HelloRequest
    ) -> Result<Response<EntityPage>, Status> { // Return an instance of type HelloReply
        println!("Got a request: {:?}", request);

        let reply = EntityPage {
            entities: [].to_vec(),
            total: 0,
        };

        Ok(Response::new(reply)) // Send back our formatted greeting
    }
}
