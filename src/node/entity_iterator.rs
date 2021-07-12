use crate::domain::entity;

pub trait EntityIterator<'a> : Iterator<Item = &'a entity::Entity> {


}
