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
    fn test_execute_query_happy_path() {
        let mut node : NodeImpl = Node::new();
       
        let e1 = entity::Entity { id: 1, x_coordinate: 2.0, y_coordinate: 2.0, z_coordinate: 3.0 };
        let e2 = entity::Entity { id: 2, x_coordinate: 2.0, y_coordinate: 2.0, z_coordinate: 3.0 };
        let e3 = entity::Entity { id: 3, x_coordinate: 2.0, y_coordinate: 2.0, z_coordinate: 3.0 };

        node.add_entities(vec![e1, e2, e3]);
        
        let query = node.new_query();
        let entities = node.execute_query(&query); 

        assert_contains_entities(entities, vec![e1, e2, e3]);
    }


    #[test]
    fn test_execute_query_paginates_correctly() {
        let mut node : NodeImpl = Node::new();
       
        let e1 = entity::Entity { id: 1, x_coordinate: 2.0, y_coordinate: 2.0, z_coordinate: 3.0 };
        let e2 = entity::Entity { id: 2, x_coordinate: 2.0, y_coordinate: 2.0, z_coordinate: 3.0 };
        let e3 = entity::Entity { id: 3, x_coordinate: 2.0, y_coordinate: 2.0, z_coordinate: 3.0 };
        let e4 = entity::Entity { id: 4, x_coordinate: 2.0, y_coordinate: 2.0, z_coordinate: 3.0 };

        node.add_entities(vec![e1, e2, e3, e4]);
        
        let mut query = node.new_query();
        query.offset(1).limit(2);

        let entities = node.execute_query(&query); 

        assert_contains_entities(entities, vec![e2, e3]);
    }

    #[test]
    fn test_execute_query_filters_by_bound_correctly() {
        let mut node : NodeImpl = Node::new();
       
        let e1 = entity::Entity { id: 1, x_coordinate: -15.0, y_coordinate: -15.0, z_coordinate: -15.0 };
        let e2 = entity::Entity { id: 2, x_coordinate: 0.0, y_coordinate: 0.0, z_coordinate: 0.0 };
        let e3 = entity::Entity { id: 3, x_coordinate: 15.0, y_coordinate: 15.0, z_coordinate: 15.0 };

        node.add_entities(vec![e1, e2, e3]);
        
        let mut query = node.new_query();
        query.bound(bound::Bound { x0: -1.0, x1: 1.0, y0: -1.0, y1: 1.0, z0: -1.0, z1: 1.0 });
        
        let entities1 = node.execute_query(&query);
        assert_contains_entities(entities1, vec![e2]);
        
        let entities2 = node.execute_query(&query);
        assert_not_contains_entities(entities2, vec![e1, e3]);
    }

    fn assert_not_contains_entities(entities: Vec<&entity::Entity>, containees: Vec<entity::Entity>) {
        for containee in containees.iter() {
            assert_not_contains_entity(&entities, containee);
        }
    }
    
    fn assert_not_contains_entity(entities: &Vec<&entity::Entity>, containee: &entity::Entity) {
        for entity in entities.iter() {
            if **entity == *containee {
                assert!(false, "Entity was found in the vec");
            }
        }
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

