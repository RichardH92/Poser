
use crate::domain::entity;

pub mod entity_query;
pub mod node_impl;

pub trait Node<T: EntityQuery> {
    fn new() -> Self;
    fn addEntities(&self, zone: i32, realm: i32, entities: Vec<entity::Entity>);
    fn removeEntities(&self, entityIds: Vec<i32>);
    fn getEntity(&self, id: i32);
    fn newQuery(&self) -> T;
}

pub trait EntityQuery {
    fn new() -> Self
    fn limit(&self, limit: i32) -> &Self;
    fn offset(&self, offset: i32) -> &Self;
    fn getEntities(&self) -> Vec<entity::Entity>;
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
