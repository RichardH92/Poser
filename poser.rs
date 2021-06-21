mod entity;

trait Node {
    fn new() -> Self;
    fn addEntities(zone: i32, realm: i32, entities: Vec<entity::Entity>);
    fn removeEntities(entityIds: Vec<i32>);
    fn getEntity(id: i32);
    fn newQuery() -> EntityQuery;
}

trait EntityQuery {
    fn filterByZone(zones: Vec<i32>) -> Self;
    fn filterByRealm(realms: Vec<i32>) -> Self;
    fn filterByBound(bound: ZoneBound) -> Self;
    fn limit(limit: i32) -> Self;
    fn offset(offset: i32) -> Self;
    fn getEntities() -> Vec<entity::Entity>;
}

trait ZoneBound {
    fn containsPoint(position: Vec<f32>) -> bool;
    fn intersect(bound: Self) -> Self;
}
