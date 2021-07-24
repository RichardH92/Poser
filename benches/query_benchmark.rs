use criterion::{black_box, criterion_group, criterion_main, Criterion};
use service::service::Service;
use service::service::service_impl::ServiceImpl;
use service::domain::entity;
use service::domain::bound;
use service::service::service_impl::EntityQueryImpl;
use service::service::entity_query::EntityQuery;
use rand::Rng;
use unchecked_unwrap::UncheckedUnwrap;

static mut service100k: Option<ServiceImpl> = None;

fn setup() {
    unsafe {
        service100k = Some(initialize_service_with_random_points(100_000));
    }
}

fn initialize_service_with_random_points(num_points: i32) -> ServiceImpl {

    let mut rng = rand::thread_rng();
    let mut service : ServiceImpl = Service::new();
    let mut entityVec : Vec<entity::Entity> = Vec::new();

    for i in 0..num_points {
       let entity = entity::Entity {
            id: i,
            x_coordinate: rng.gen_range(0..1000),
            y_coordinate: rng.gen_range(0..1000),
            z_coordinate: rng.gen_range(0..1000),
       };

       entityVec.push(entity);
    }
    service.add_entities(entityVec);

    service
}

fn execute_query_with_bound_and_limit(limit: u32, service: &mut ServiceImpl) {
    let mut rng = rand::thread_rng();

    let bound = bound::Bound {
       x0: rng.gen_range(0..250),
       x1: rng.gen_range(750..1000),
       y0: rng.gen_range(0..250),
       y1: rng.gen_range(750..1000),
       z0: rng.gen_range(0..250),
       z1: rng.gen_range(750..1000),
    };
        
    let mut query = service.new_query();
    query.limit(limit).bound(bound);

    service.execute_query(&query);
}

fn benchmark_execute_query_bound_filter_100k_items_limit_1000(c: &mut Criterion) {
    setup();

    unsafe {
        let service = service100k.as_mut().unwrap();
        c.bench_function("query 100k items with bound filter and limit 1000", |b| b.iter(|| execute_query_with_bound_and_limit(1000, service)));
    }
}

fn benchmark_execute_query_bound_filter_100k_items_limit_100(c: &mut Criterion) {
    setup();

    unsafe {
        let service = service100k.as_mut().unwrap();
        c.bench_function("query 100k items with bound filter and limit 100", |b| b.iter(|| execute_query_with_bound_and_limit(100, service)));
    }
}

criterion_group!(benches, benchmark_execute_query_bound_filter_100k_items_limit_100, benchmark_execute_query_bound_filter_100k_items_limit_1000);
criterion_main!(benches);
