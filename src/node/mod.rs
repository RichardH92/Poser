
use crate::entity::entity;
use crate::node::entity_query;
pub mod entity_query;
pub mod node_impl;

pub trait Node {
    fn new() -> Self;
    fn addEntities(zone: i32, realm: i32, entities: Vec<entity::Entity>);
    fn removeEntities(entityIds: Vec<i32>);
    fn getEntity(id: i32);
    fn newQuery() -> Box<dyn entity::EntityQuery>;
}

pub trait EntityQuery {

    fn limit(limit: i32) -> Self;
    fn offset(offset: i32) -> Self;
    fn getEntities() -> Vec<entity::Entity>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_entities_happy_path() {
        let mut node = new();
        assert_eq!(4, 4);
    }
}
