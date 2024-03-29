use rstar::RTree;
use rstar::RTreeObject;
use rstar::AABB;
use crate::service::Service;
use crate::service::AddEntitiesValidationErrors;
use crate::domain::entity;
use crate::domain::bound;
use crate::service::entity_query::EntityQuery;
use std::collections::HashMap;
use event_logger::service::event_log_impl::EventLogImpl;
use event_logger::service::EventLog;
use event_logger::repository::event_repository_impl::EventRepositoryImpl;
use event_logger::repository::EventRepository;
use event_logger::domain::event::Event;

pub struct ServiceImpl {
    tree: RTree<entity::Entity>,
    map: HashMap<u32, entity::Entity>,
    event_log: EventLogImpl
}

pub struct EntityQueryImpl {
    limit: u32,
    offset: u32,
    bound: Option<bound::Bound>
}

impl RTreeObject for entity::Entity {
    
    type Envelope = AABB<[i32; 3]>;

    fn envelope(&self) -> Self::Envelope {
        AABB::from_point([self.x_coordinate, self.y_coordinate, self.z_coordinate])
    }
}

impl<'a> EntityQuery for EntityQueryImpl {

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


impl<'a> Service<'a, EntityQueryImpl> for ServiceImpl {

    fn new() -> ServiceImpl {
        ServiceImpl { 
            tree: RTree::new(),
            map: HashMap::new(),
            event_log: EventLog::new(EventRepository::new("event-log".to_string()))
        }
    }

    fn add_entities(&mut self, entities: Vec<entity::Entity>) -> Result<(), Vec<AddEntitiesValidationErrors>> {
        let mut errs : Vec<AddEntitiesValidationErrors> = Vec::new();
        let mut seen_ids : Vec<u32> = Vec::new();

        for entity in entities.iter() {
            if seen_ids.contains(&entity.id) {
               errs.push(AddEntitiesValidationErrors::EntityAddedTwice); 
            } else {
               seen_ids.push(entity.id);
               match self.map.get(&entity.id) {
                   Some(_e) => errs.push(AddEntitiesValidationErrors::EntityAlreadyExists),
                   _ => { 
                        let mut params : HashMap<String, String> = HashMap::new();

                        params.insert("id".to_string(), "test123".to_string());

                        let event = Event {
                            event_type_key: "upsert-entity".to_string(),
                            params: params
                        };

                        self.event_log.log_event(event);
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

    fn remove_entities(&mut self, entity_ids: Vec<u32>) {
        for id in entity_ids.iter() {
            match self.map.remove(id) {
                Some(entity) => { self.tree.remove(&entity); },
                _ => ()
            }
        }
    }

    fn get_entity(&self, id: u32) -> Option<&entity::Entity> {
        self.map.get(&id)
    }
    
    fn new_query(&self) -> EntityQueryImpl {
        EntityQueryImpl {
            limit: 100,
            offset: 0,
            bound: None
        }
    }

    fn execute_query<'b>(&'b self, query: &EntityQueryImpl) -> Vec<&entity::Entity> {
        match query.bound {
            Some(bound) => {
                let b = AABB::from_corners([bound.x0, bound.y0, bound.z0], [bound.x1, bound.y1, bound.z1]);
                self.tree.locate_in_envelope(&b).collect()
            },
            None => self.tree.iter().collect()
        }
    }
}


