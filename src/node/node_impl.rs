
use rstar::RTree;
use rstar::RTreeObject;
use rstar::AABB;
use crate::node::Node;
use crate::domain::entity;
use crate::node::entity_query::EntityQueryImpl;

pub struct NodeImpl {
    tree: RTree<entity::Entity>
}

impl RTreeObject for entity::Entity {
    
    type Envelope = AABB<[f32; 2]>;

    fn envelope(&self) -> Self::Envelope
    {
        AABB::from_point([self.x_coordinate, self.y_coordinate])
    }
}

impl Node<EntityQueryImpl> for NodeImpl {

    fn new() -> NodeImpl {
        NodeImpl { tree: RTree::new() }
    }

    fn addEntities(self, zone: i32, realm: i32, entities: Vec<entity::Entity>) {

    }

    fn removeEntities(self, entityIds: Vec<i32>) {
    
    }

    fn getEntity(self, id: i32) {

    }
    
    fn newQuery(self) -> EntityQueryImpl {
        return EntityQueryImpl::new();
    }
}


