use self::component::BlockComponent;

pub mod block_registry;
pub mod component;

#[derive(Clone)]
pub struct Block<'a> {
    pub type_id: String,
    pub components: Vec<&'a dyn BlockComponent>,
    pub texture_set: String,
    pub sound: String,
}
