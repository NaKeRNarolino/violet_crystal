use std::sync::Mutex;

use askama::Template;

pub mod component;
pub mod item_registry;

#[derive(Clone)]
pub struct Item<'a> {
    pub type_id: String,
    pub components: Vec<&'a dyn component::ItemComponent>,
    pub texture: String,
}
impl<'a> Item<'a> {
    pub fn serialize(&self) -> String {
        let components = self.components.clone();
        let mut components_strings: Vec<String> = vec![];
        for component in components {
            let comp = Mutex::new(component);
            let ser = comp.lock();
            let mut fser = match ser {
                Ok(guard) => guard.serialize(),
                Err(_) => "Error serializing".to_string(),
            };
            fser.push(',');
            components_strings.push(fser);
        }
        components_strings.last_mut().unwrap().pop();
        ItemTemplate {
            id: &self.type_id,
            components: components_strings,
        }
        .render()
        .unwrap()
    }
}

#[derive(Template)]
#[template(path = "item_serialization/item_template.json.jinja2", escape = "none")]
struct ItemTemplate<'a> {
    pub id: &'a str,
    pub components: Vec<String>,
}

#[derive(Template)]
#[template(path = "item_serialization/item_texture.json.jinja2", escape = "none")]
pub struct ItemAtlasTemplate<'a> {
    pub name: &'a String,
    pub contents: String,
}
