use tonic::{transport::Server, Request, Response, Status};
use crate::poser::poser_server::{Poser, PoserServer};
use crate::poser::{EntityPage, GetEntitiesRequest, AddEntitiesRequest, AddEntitiesResponse};
use crate::service::service_impl::ServiceImpl;
use crate::service::Service;
use crate::grpc::mapper;

use std::sync::{RwLock, Arc, mpsc::channel};

pub struct PoserImpl {
    service_lock: RwLock<ServiceImpl>
}

pub fn init() -> PoserServer<PoserImpl> {
    return PoserServer::new(PoserImpl{
        service_lock: RwLock::new(Service::new())
    });
}

#[tonic::async_trait]
impl Poser for PoserImpl {

    async fn add_entities(
        &self,
        request: Request<AddEntitiesRequest>,
    ) -> Result<Response<AddEntitiesResponse>, Status> {

        let addEntitiesReq = request.into_inner();

        let entities = mapper::map_add_entities_request_to_domain(addEntitiesReq);

        let mut service = self.service_lock.write().unwrap();

        let result = service.add_entities(entities);

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

        let service = self.service_lock.read().unwrap();
        let query = service.new_query();
        let entities = service.execute_query(&query); 

        Ok(Response::new(mapper::map_entities_to_api_response(entities)))
    }
}
