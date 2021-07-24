use crate::domain::entity;

pub trait EntityIterator<'a> : Iterator<Item = entity::Entity> { }
