use rstar::RTree;
use rstar::RTreeObject;
use rstar::AABB;
use crate::node::Node;
use crate::node::AddEntitiesValidationErrors;
use crate::domain::entity;
use crate::domain::bound;
use crate::node::entity_query::EntityQuery;
use std::collections::HashMap;

pub struct NodeImpl {
    tree: RTree<entity::Entity>,
    map: HashMap<i32, entity::Entity>
}

pub struct EntityQueryImpl {
    limit: u32,
    offset: u32,
    bound: Option<bound::Bound>
}

impl RTreeObject for entity::Entity {
    
    type Envelope = AABB<[f32; 3]>;

    fn envelope(&self) -> Self::Envelope {
        AABB::from_point([self.x_coordinate, self.y_coordinate, self.z_coordinate])
    }
}

impl EntityQuery for EntityQueryImpl {

    fn limit(&mut self, limit: u32) -> &mut Self {
        self.limit = limit;
        self
    }
    
    fn offset(&mut self, offset: u32) -> &mut Self {
        self.offset = offset;
        self
    }

    fn bound(&mut self, bound: bound::Bound) -> &mut Self {
        self.bound = Some(bound);
        self
    }
}

impl Node<EntityQueryImpl> for NodeImpl {

    fn new() -> NodeImpl {
        NodeImpl { 
            tree: RTree::new(),
            map: HashMap::new()
        }
    }

    fn add_entities(&mut self, entities: Vec<entity::Entity>) -> Result<(), Vec<AddEntitiesValidationErrors>> {
        let mut errs : Vec<AddEntitiesValidationErrors> = Vec::new();
        let mut seen_ids : Vec<i32> = Vec::new();

        for entity in entities.iter() {
            if seen_ids.contains(&entity.id) {
               errs.push(AddEntitiesValidationErrors::EntityAddedTwice); 
            } else {
               seen_ids.push(entity.id);
               match self.map.get(&entity.id) {
                   Some(_e) => errs.push(AddEntitiesValidationErrors::EntityAlreadyExists),
                   _ => { 
                       self.map.insert(entity.id, entity.clone());
                       self.tree.insert(entity.clone());
                   },
               }
            }
        }

        if errs.is_empty() {
            return Ok(());
        }

        Err(errs) 
    }

    fn remove_entities(&mut self, entity_ids: Vec<i32>) {
        for id in entity_ids.iter() {
            match self.map.remove(id) {
                Some(entity) => { self.tree.remove(&entity); },
                _ => ()
            }
        }
    }

    fn get_entity(&self, id: i32) -> Option<&entity::Entity> {
        self.map.get(&id) 
    }
    
    fn new_query(&self) -> EntityQueryImpl {
        EntityQueryImpl {
            limit: 100,
            offset: 0,
            bound: None
        }
    }

    fn execute_query(&self, query: &EntityQueryImpl) -> Vec<&entity::Entity> {
        let mut ret : Vec<&entity::Entity> = Vec::new();
        
        match query.bound {
            Some(bound) => { 
                let b = AABB::from_corners([bound.x0, bound.x1], [bound.y0, bound.y1], [bound.z0, bound.z1]);
                return self.tree.locate_in_envelope(&b);
            },
            _ => {
                for entry in self.map.iter() {
                    ret.push(entry.1);
                }           
            } 
        }
    
        ret
    }
}


