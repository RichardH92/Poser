mod entity

struct EntityQueryImpl {
    zones: Vec<i32>,
    realms: Vec<i32>,
    bound: ZoneBound,
    limit: i32,
    offset: i32
}

impl EntityQueryImpl for EntityQuery {
    fn filterByZone(zones: Vec<i32>) -> EntityQueryImpl {

    }
    
    fn filterByRealm(realms: Vec<i32>) -> EntityQueryImpl {

    }

    fn filterByBound(bound: ZoneBound) -> EntityQueryImpl {

    }

    fn limit(limit: i32) -> EntityQueryImpl {

    }
    
    fn offset(offset: i32) -> EntityQueryImpl {

    }

    fn getEntities() -> Vec<entity::Entity> {

    }
}


