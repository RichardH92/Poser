use crate::domain::entity;
use crate::node::Node;
use crate::node::node_impl::NodeImpl;

pub trait EntityQuery {
    fn limit(&mut self, limit: u32) -> &mut Self;
    fn offset(&mut self, offset: u32) -> &mut Self;
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

        assert_eq!(*entities[0], e1);
    }
}

