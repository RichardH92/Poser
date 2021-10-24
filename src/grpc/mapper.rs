use crate::poser::{
    EntityPage, 
    GetEntitiesRequest, 
    AddEntitiesRequest, 
    AddEntitiesResponse
};
use crate::domain::entity::Entity;
use crate::service::AddEntitiesValidationErrors;
use crate::service::service_impl::EntityQueryImpl;
use crate::service::entity_query::EntityQuery;


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
    let mut entity_page = EntityPage {
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

        entity_page.entities.push(e);
    }

    return entity_page;
}

pub fn apply_query_critera(request: GetEntitiesRequest, mut query: EntityQueryImpl) -> EntityQueryImpl {

    query.limit(get_limit(&request)).offset(request.offset);


    return query;
}

fn get_limit(request: &GetEntitiesRequest) -> u32 {
    if request.limit > 0 {
        if request.limit > 10000 {
            return 10000;
        } else {
            return request.limit;
        }
    } else {
        return 1000;
    }
}

pub fn map_add_validation_errors_to_api_response(result: Result<(), Vec<AddEntitiesValidationErrors>>) -> AddEntitiesResponse {
    
    
    let mut response = AddEntitiesResponse {
        errors: Vec::new()
    };

    match result {
        Err(errs) => {
            for error in errs {
                response.errors.push(map_add_validation_error(error));
            }
        },
        _ => {}
    }
    
    return response;
}

fn map_add_validation_error(error: AddEntitiesValidationErrors) -> i32 {
    match error {
        AddEntitiesValidationErrors::EntityAddedTwice => 0,
        AddEntitiesValidationErrors::EntityAlreadyExists => 1,
    }
}