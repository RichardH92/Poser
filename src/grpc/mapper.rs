use crate::poser::{EntityPage, GetEntitiesRequest, AddEntitiesRequest, AddEntitiesResponse};
use crate::domain::entity::Entity;


pub fn mapAddEntitiesRequestToDomain(req: AddEntitiesRequest) -> Vec<Entity> {
    let ret = vec!();

    println!("{:?}", req.entities);

    return ret;
}