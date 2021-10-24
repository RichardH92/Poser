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

pub fn map_entities_to_api_response(entities: Vec<&Entity>) -> EntityPage {
    let mut entityPage = EntityPage {
        entities: Vec::new(),
        total: 0
    };

    for entity in entities {
        let e = crate::poser::Entity {
            id: entity.id,
            x: entity.x_coordinate,
            y: entity.y_coordinate,
            z: entity.z_coordinate,
        };

        entityPage.entities.push(e);
    }

    return entityPage;
}