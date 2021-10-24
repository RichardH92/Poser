use tonic::{Request, Response, Status};
use crate::poser::poser_server::{Poser, PoserServer};
use crate::poser::{
    EntityPage, 
    GetEntitiesRequest, 
    AddEntitiesRequest, 
    AddEntitiesResponse, 
    GetEntityRequest, 
    RemoveEntitiesRequest, 
    RemoveEntitiesResponse
};
use crate::service::service_impl::ServiceImpl;
use crate::service::Service;
use crate::grpc::mapper;

use std::sync::{RwLock};

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

    async fn get_entity(
        &self,
        _request: Request<GetEntityRequest>,
    ) -> Result<Response<crate::poser::Entity>, Status> {

        Err(Status::unimplemented("not implemented"))
    }

    async fn remove_entities(
        &self,
        _request: Request<RemoveEntitiesRequest>,
    ) -> Result<Response<RemoveEntitiesResponse>, Status> {

        Err(Status::unimplemented("not implemented"))
    }

    async fn add_entities(
        &self,
        request: Request<AddEntitiesRequest>,
    ) -> Result<Response<AddEntitiesResponse>, Status> {

        let add_entities_req = request.into_inner();
        let entities = mapper::map_add_entities_request_to_domain(add_entities_req);
        let mut service = self.service_lock.write().unwrap();
        let result = service.add_entities(entities);
        
        Ok(Response::new(mapper::map_add_validation_errors_to_api_response(result)))
    }

    async fn get_entities(
        &self,
        request: Request<GetEntitiesRequest>,
    ) -> Result<Response<EntityPage>, Status> {
        
        println!("Got a request: {:?}", request);

        let service = self.service_lock.read().unwrap();
        let mut query = service.new_query();
        let filtered_query = mapper::apply_query_critera(request.into_inner(), query);
        let entities = service.execute_query(&filtered_query); 

        Ok(Response::new(mapper::map_entities_to_api_response(entities)))
    }
}
