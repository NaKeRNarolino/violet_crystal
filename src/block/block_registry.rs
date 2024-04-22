use askama::Template;

use super::Block;

#[derive(Clone)]
pub struct BlockRegistry<'a> {
    pub items: Vec<Block<'a>>,
    pub item_atlas: Vec<BlockAtlasEntry>,
}

#[derive(Template)]
#[template(
    path = "block_serialization/block_atlas_entry.json.jinja2",
    escape = "none"
)]
struct BlockAtlasEntryTemplate {
    pub id: String,
    pub textures: String,
    pub sound: String,
}

#[derive(Clone)]
pub struct BlockAtlasEntry {
    pub id: String,
    pub path: String,
    pub textures: String,
    pub sound: String,
}
impl BlockAtlasEntry {
    fn serialize(&self) -> String {
        BlockAtlasEntryTemplate {
            textures: self
                .clone()
                .id
                .chars()
                .map(|x| if x == ':' { '_' } else { x })
                .collect(),
            id: self
                .clone()
                .id
                .chars()
                .map(|x| if x == ':' { '_' } else { x })
                .collect(),
            sound: self.clone().sound,
        }
        .render()
        .unwrap()
    }
}

pub fn serialize_item_atlas(atlas: &Vec<BlockAtlasEntry>) -> String {
    let mut atlas_string = String::new();
    for entry in atlas {
        atlas_string.push_str(&entry.serialize());
        atlas_string.push(',');
    }
    atlas_string.pop();
    atlas_string
}

impl<'a> BlockRegistry<'a> {
    pub fn new() -> Self {
        Self {
            items: vec![],
            item_atlas: vec![],
        }
    }

    pub fn add_block(&mut self, block: Block<'a>) {
        self.items.push(block.clone());
        self.item_atlas.push(BlockAtlasEntry {
            id: block.clone().type_id,
            path: block.clone().texture_set,
            textures: block
                .type_id
                .chars()
                .into_iter()
                .map(|x| if x == ':' { '_' } else { x })
                .collect(),
            sound: block.sound,
        });
    }
}
