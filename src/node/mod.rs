
use crate::domain::entity;

pub mod entity_query;
pub mod node_impl;

pub trait Node/*<T: EntityQuery>*/ {
    fn new() -> Self;
    fn addEntities(&mut self, entities: Vec<entity::Entity>);
    fn removeEntities(&mut self, entityIds: Vec<i32>);
    fn getEntity(&self, id: i32) -> Option<&entity::Entity>;
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
       
        let entity = entity::Entity { id: 1, x_coordinate: 2.0, y_coordinate: 2.0, z_coordinate: 3.0 };

        node.addEntities(vec![entity]);

        let expected = entity::Entity { id: 1, x_coordinate: 2.0, y_coordinate: 2.0, z_coordinate: 3.0 };
        match node.getEntity(1) {
            Some(actual) => assert_eq!(expected, *actual),
            None => assert!(false, "false"),
        }
    }
}
