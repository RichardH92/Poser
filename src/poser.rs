mod entity;

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
