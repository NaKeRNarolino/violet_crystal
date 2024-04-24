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
        return BlockCollisionBoxComponentTemplate {
            enabled,
            origin,
            size,
        }
        .render()
        .unwrap();
    }
}
