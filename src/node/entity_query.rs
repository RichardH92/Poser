use crate::domain::entity;
use crate::node::Node;
use crate::domain::bound;
use crate::node::node_impl::NodeImpl;

pub trait EntityQuery {
    fn limit(&mut self, limit: u32) -> &mut Self;
    fn offset(&mut self, offset: u32) -> &mut Self;
    fn bound(&mut self, bound: bound::Bound) -> &mut Self;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_entities_happy_path() {
        let mut node : NodeImpl = Node::new();
       
        let e1 = entity::Entity { id: 1, x_coordinate: 2.0, y_coordinate: 2.0, z_coordinate: 3.0 };
        let e2 = entity::Entity { id: 2, x_coordinate: 2.0, y_coordinate: 2.0, z_coordinate: 3.0 };
        let e3 = entity::Entity { id: 3, x_coordinate: 2.0, y_coordinate: 2.0, z_coordinate: 3.0 };

        node.add_entities(vec![e1, e2, e3]);
        
        let query = node.new_query();
        let entities = node.execute_query(query); 

        assert_contains_entities(entities, vec![e1, e2, e3]);
    }

    fn assert_contains_entities(entities: Vec<&entity::Entity>, containees: Vec<entity::Entity>) {
        for containee in containees.iter() {
            assert_contains_entity(&entities, containee);
        }
    }

    fn assert_contains_entity(entities: &Vec<&entity::Entity>, containee: &entity::Entity) {
        for entity in entities.iter() {
            if **entity == *containee {
                return;
            }
        }

        assert!(false, "Entity was not found in the vec");
    }
}

