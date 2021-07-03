
use crate::domain::entity;

pub mod entity_query;
pub mod node_impl;

pub trait Node/*<T: EntityQuery>*/ {
    fn new() -> Self;
    fn addEntities(&self, zone: i32, realm: i32, entities: Vec<entity::Entity>);
    fn removeEntities(&self, entityIds: Vec<i32>);
    fn getEntity(&self, id: i32);
    //fn newQuery(&self) -> T;
}

/*pub trait EntityQuery {
    fn new() -> Self
    fn limit(&mut self, limit: i32) -> &mut Self;
    fn offset(&mut self, offset: i32) -> &mut Self;
    fn getEntities(&self) -> Vec<entity::Entity>;
}*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_entities_happy_path() {
        let mut node : node_impl::NodeImpl = Node::new();
        assert_eq!(4, 4);
    }
}
