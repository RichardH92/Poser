use crate::domain::entity;

pub trait EntityQuery : Sized {
    fn limit(&mut self, limit: u32) -> &mut Self;
    fn offset(&mut self, offset: u32) -> &mut Self;
    fn get_entities(&self) -> Vec<entity::Entity>;
}


