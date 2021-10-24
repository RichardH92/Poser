use crate::poser::{EntityPage, GetEntitiesRequest, AddEntitiesRequest, AddEntitiesResponse};
use crate::domain::entity::Entity;


pub fn map_add_entities_request_to_domain(req: AddEntitiesRequest) -> Vec<Entity> {
    let mut ret : Vec<Entity> = Vec::new();

    for entity in req.entities {
        let domain = Entity{
            id: entity.id,
            x_coordinate: entity.x,
            y_coordinate: entity.y,
            z_coordinate: entity.z,
        };

        ret.push(domain);
    }

    return ret;
}