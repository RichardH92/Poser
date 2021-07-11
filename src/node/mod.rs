
use crate::domain::entity;

pub mod entity_query;
pub mod node_impl;

pub trait Node<Q: entity_query::EntityQuery> {
    fn new() -> Self;
    fn add_entities(&mut self, entities: Vec<entity::Entity>);
    fn remove_entities(&mut self, entity_ids: Vec<i32>);
    fn get_entity(&self, id: i32) -> Option<&entity::Entity>;
    fn new_query(&self) -> Q; 
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_entities_happy_path() {
        let mut node : node_impl::NodeImpl = Node::new();
       
        let entity = entity::Entity { id: 1, x_coordinate: 2.0, y_coordinate: 2.0, z_coordinate: 3.0 };

        node.add_entities(vec![entity]);

        let expected = entity::Entity { id: 1, x_coordinate: 2.0, y_coordinate: 2.0, z_coordinate: 3.0 };
        match node.get_entity(1) {
            Some(actual) => assert_eq!(expected, *actual),
            None => assert!(false, "Returned empty incorrectly"),
        }
    }
    
    #[test]
    fn test_get_entity_returns_empty_when_entity_dne() {
        let mut node : node_impl::NodeImpl = Node::new();
       
        let entity = entity::Entity { id: 1, x_coordinate: 2.0, y_coordinate: 2.0, z_coordinate: 3.0 };
        node.add_entities(vec![entity]);

        match node.get_entity(2) {
            Some(_x) => assert!(false, "Returned an entity incorrectly"),
            None => assert!(true, ""),
        }
    }

    #[test]
    fn test_remove_entities_noops_if_entity_dne() {
        let mut node : node_impl::NodeImpl = Node::new();
        node.remove_entities(vec![1]);
    }

    #[test]
    fn test_remove_entities_happy_path() {
        let mut node : node_impl::NodeImpl = Node::new();
       
        let entity = entity::Entity { id: 1, x_coordinate: 2.0, y_coordinate: 2.0, z_coordinate: 3.0 };
        node.add_entities(vec![entity]);
        node.remove_entities(vec![1]);

        match node.get_entity(1) {
            Some(_x) => assert!(false, "Returned an entity incorrectly"),
            None => assert!(true, ""),
        }
    }


}
