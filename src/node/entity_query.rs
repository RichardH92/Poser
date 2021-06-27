use crate::domain::entity;
use crate::node::EntityQuery;

pub struct EntityQueryImpl {
    limit: i32,
    offset: i32
}

impl EntityQuery for EntityQueryImpl {

    fn new() -> EntityQueryImpl {
        return EntityQueryImpl{ limit: 0, offset: 0 };
    }

    fn limit(&self, limit: i32) -> &EntityQueryImpl {
        return self;
    }
    
    fn offset(&self, offset: i32) -> &EntityQueryImpl {
        return self;
    }

    fn getEntities(&self) -> Vec<entity::Entity> {
        return vec![];
    }
}


