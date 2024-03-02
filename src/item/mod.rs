pub mod component;

pub struct Item<'a> {
    pub display_name: String,
    pub type_id: String,
    pub components: Vec<&'a dyn component::ItemComponent>,
}
