
use rstar::RTree;
use node::Node;
use entity::entity;
use create::node::EntityQuery;

    pub struct NodeImpl {
        tree: RTree
    }

    pub impl Node for NodeImpl {

        pub fn new() -> NodeImpl {
            NodeImpl { tree: RTree::new() }
        }

        fn addEntities(self, zone: i32, realm: i32, entities: Vec<entity::Entity>) {

        }

        fn removeEntities(self, entityIds: Vec<i32>) {
    
        }

        fn getEntity(self, id: i32) {

        }
    
        fn newQuery(self) -> Box<dyn EntityQuery> {

        }
    }


