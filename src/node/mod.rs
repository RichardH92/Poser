use crate::domain::entity;

pub mod entity_iterator;
pub mod entity_query;
pub mod node_impl;

#[derive(PartialEq)]
pub enum AddEntitiesValidationErrors {
    EntityAddedTwice,
    EntityAlreadyExists
}

pub trait Node<'a, Q: entity_query::EntityQuery, I: Iterator<Item = &'a entity::Entity>> {
    fn new() -> Self;
    fn add_entities(&mut self, entities: Vec<entity::Entity>) -> Result<(), Vec<AddEntitiesValidationErrors>>;
    fn remove_entities(&mut self, entity_ids: Vec<i32>);
    fn get_entity(&self, id: i32) -> Option<&entity::Entity>;
    fn new_query(&self) -> Q; 
    fn execute_query<'b : 'a>(&self, query: &Q) -> I; 
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_entities_happy_path() {
        let mut node : node_impl::NodeImpl = Node::new();
        let entity = entity::Entity { id: 1, x_coordinate: 2.0, y_coordinate: 2.0, z_coordinate: 3.0 };

        let result = node.add_entities(vec![entity]);
        match result {
            Err(_errs) => assert!(false, "Validation error returned incorrectly"),
            _ => (),
        }

        let expected = entity::Entity { id: 1, x_coordinate: 2.0, y_coordinate: 2.0, z_coordinate: 3.0 };
        match node.get_entity(1) {
            Some(actual) => assert_eq!(expected, *actual),
            None => assert!(false, "Returned empty incorrectly"),
        }
    }
    
    #[test]
    fn test_add_entities_returns_error_when_entity_added_twice() {
        let mut node : node_impl::NodeImpl = Node::new();
       
        let e1 = entity::Entity { id: 1, x_coordinate: 2.0, y_coordinate: 2.0, z_coordinate: 3.0 };
        let e2 = entity::Entity { id: 1, x_coordinate: 2.0, y_coordinate: 2.0, z_coordinate: 3.0 };
        
        let result = node.add_entities(vec![e1, e2]);
        match result {
            Err(errs) => {
                assert_eq!(1, errs.len());
                assert!(errs.contains(&AddEntitiesValidationErrors::EntityAddedTwice), "Validation error not returned");
            },
            _ => assert!(false, "No validation error was returned")
        }
        
    }
    
    #[test]
    fn test_add_entities_returns_error_when_entity_already_exists() {
        let mut node : node_impl::NodeImpl = Node::new();
       
        let e1 = entity::Entity { id: 1, x_coordinate: 2.0, y_coordinate: 2.0, z_coordinate: 3.0 };
        let e2 = entity::Entity { id: 1, x_coordinate: 2.0, y_coordinate: 2.0, z_coordinate: 3.0 };
        
        match node.add_entities(vec![e1]) {
            Err(_errs) => assert!(false, "Validation errors returned incorrectly"),
            _ => ()
        }
        
        match node.add_entities(vec![e2]) {
            Err(errs) => {
                assert_eq!(1, errs.len());
                assert!(errs.contains(&AddEntitiesValidationErrors::EntityAlreadyExists), "Validation error not returned");
            },
            _ => assert!(false, "No validation error was returned")
        }
    }

    #[test]
    fn test_add_entities_returns_multiple_validation_errors_correctly() {
        let mut node : node_impl::NodeImpl = Node::new();
       
        let e1 = entity::Entity { id: 1, x_coordinate: 2.0, y_coordinate: 2.0, z_coordinate: 3.0 };
        let e2 = entity::Entity { id: 1, x_coordinate: 2.0, y_coordinate: 2.0, z_coordinate: 3.0 };
        let e3 = entity::Entity { id: 2, x_coordinate: 2.0, y_coordinate: 2.0, z_coordinate: 3.0 };
        let e4 = entity::Entity { id: 2, x_coordinate: 2.0, y_coordinate: 2.0, z_coordinate: 3.0 };
        
        match node.add_entities(vec![e1]) {
            Err(_errs) => assert!(false, "Validation errors returned incorrectly"),
            _ => ()
        }
        
        match node.add_entities(vec![e2, e3, e4]) {
            Err(errs) => {
                assert_eq!(2, errs.len());
                assert!(errs.contains(&AddEntitiesValidationErrors::EntityAlreadyExists), "Validation error not returned");
                assert!(errs.contains(&AddEntitiesValidationErrors::EntityAddedTwice), "Validation error not returned");
            }
            _ => assert!(false, "Validation errors not returned")
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
