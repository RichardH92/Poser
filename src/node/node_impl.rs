
use rstar::RTree;
use rstar::RTreeObject;
use rstar::AABB;
use crate::node::Node;
use crate::domain::entity;
// use crate::node::EntityQuery;
// use crate::node::entity_query::EntityQueryImpl;
use std::collections::HashMap;

pub struct NodeImpl {
    tree: RTree<entity::Entity>,
    map: HashMap<i32, entity::Entity>
}

impl RTreeObject for entity::Entity {
    
    type Envelope = AABB<[f32; 2]>;

    fn envelope(&self) -> Self::Envelope
    {
        AABB::from_point([self.x_coordinate, self.y_coordinate])
    }
}

impl Node/*<EntityQueryImpl>*/ for NodeImpl {

    fn new() -> NodeImpl {
        NodeImpl { 
            tree: RTree::new(),
            map: HashMap::new()
        }
    }

    fn addEntities(&mut self, entities: Vec<entity::Entity>) {
        
        for entity in entities.iter() {
            self.map.insert(entity.id, entity.clone());
        }

    }

    fn removeEntities(&mut self, entityIds: Vec<i32>) {
    
    }

    fn getEntity(&self, id: i32) -> Option<&entity::Entity> {
        self.map.get(&id) 
    }
    
    /*fn newQuery(&self) -> EntityQueryImpl {
        let query : EntityQuery = EntityQuery::new();
        return query;
    }*/
}


