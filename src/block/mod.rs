use std::sync::Mutex;
use askama::Template;
use crate::vio::Identifier;
use self::component::BlockComponent;

pub mod block_registry;
pub mod component;

#[derive(Clone)]
pub struct Block<'a> {
    pub type_id: Identifier<'a>,
    pub components: Vec<&'a dyn BlockComponent>,
    pub texture_set: String,
    pub sound: String,
}

impl<'a> Block<'a> {
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
        BlockTemplate {
            type_id: self.clone().type_id.render(),
            components: components_strings.join("\n"),
            traits: "".to_string(),
            states: "".to_string(),
            permutations: "".to_string()
        }
            .render()
            .unwrap()
    }
}

#[derive(Template)]
#[template(
    path = "block_serialization/block.json.jinja2",
    escape = "none"
)]
struct BlockTemplate {
    type_id: String,
    components: String,
    traits: String,
    permutations: String,
    states: String,
}