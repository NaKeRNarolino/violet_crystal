use std::path::Components;
use askama::Template;
use crate::block::Block;

use crate::vio::Vec3;

pub trait BlockComponent {
    fn serialize(&self) -> String;
}

// * BlockCollisionBoxComponent

pub struct BlockCollisionBoxComponent {
    pub enabled: bool,
    pub origin: Option<Vec3>,
    pub size: Option<Vec3>,
}

#[derive(Template)]
#[template(
    path = "block_serialization/components/collision_box.json.jinja2",
    escape = "none"
)]
struct BlockCollisionBoxComponentTemplate {
    pub enabled: bool,
    pub origin: String,
    pub size: String,
}

impl BlockComponent for BlockCollisionBoxComponent {
    fn serialize(&self) -> String {
        let enabled = self.enabled;
        let orgn = self.origin.unwrap_or(Vec3 {
            x: -1,
            y: -1,
            z: -1,
        });
        let sz = self.size.unwrap_or(Vec3 {
            x: -1,
            y: -1,
            z: -1,
        });
        let origin = if orgn.x != -1 {
            format!("[{}, {}, {}]", orgn.x, orgn.y, orgn.z)
        } else {
            "[]".to_string()
        };
        let size = if sz.x != -1 {
            format!("[{}, {}, {}]", sz.x, sz.y, sz.z)
        } else {
            "[]".to_string()
        };
        BlockCollisionBoxComponentTemplate {
            enabled,
            origin,
            size,
        }
        .render()
        .unwrap()
    }
}


// * BlockCraftingTableBoxComponent

pub struct BlockCraftingTableComponent<'a> {
    pub name: &'a str,
    pub tags: Vec<&'a str>,
}

#[derive(Template)]
#[template(
path = "block_serialization/components/crafting_table.json.jinja2",
escape = "none"
)]
struct BlockCraftingTableComponentTemplate {
    name: String,
    tags: String,
}

impl<'a> BlockComponent for BlockCraftingTableComponent<'a> {
    fn serialize(&self) -> String {
        let tags = format!("{:?}", self.tags);
        BlockCraftingTableComponentTemplate {
            tags,
            name: self.name.to_string()
        }.render().unwrap()
    }
}

// * BlockDestructibleByExplosionComponent

pub struct BlockDestructibleByExplosionComponent {
    pub explosion_resistance: Option<f64>
}

#[derive(Template)]
#[template(
path = "block_serialization/components/destructible_by_explosion.json.jinja2",
escape = "none"
)]
struct BlockDestructibleByExplosionComponentTemplate {
    pub explosion_resistance: f64
}

impl BlockComponent for BlockDestructibleByExplosionComponent {
    fn serialize(&self) -> String {
        BlockDestructibleByExplosionComponentTemplate {
            explosion_resistance: self.explosion_resistance.unwrap_or(0.0)
        }.render().unwrap()
    }
}

// * BlockDestructibleByMiningComponent

pub struct BlockDestructibleByMiningComponent {
    pub seconds_to_destroy: Option<f64>
}

#[derive(Template)]
#[template(
path = "block_serialization/components/destructible_by_mining.json.jinja2",
escape = "none"
)]
struct BlockDestructibleByMiningComponentTemplate {
    pub seconds_to_destroy: f64
}

impl BlockComponent for BlockDestructibleByMiningComponent {
    fn serialize(&self) -> String {
        BlockDestructibleByMiningComponentTemplate {
            seconds_to_destroy: self.seconds_to_destroy.unwrap_or(0.0)
        }.render().unwrap()
    }
}


// * BlockCustomComponents

pub struct BlockCustomComponents<'a> {
    pub components: Vec<&'a str>
}

#[derive(Template)]
#[template(
    path = "block_serialization/components/custom_components.json.jinja2",
    escape = "none"
)]
struct BlockCustomComponentsTemplate {
    pub components: String
}

impl BlockComponent for BlockCustomComponents<'_> {
    fn serialize(&self) -> String {
        BlockCustomComponentsTemplate {
            components: format!("{:?}", self.components)
        }.render().unwrap()
    }
}