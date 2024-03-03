use std::fs;

use askama::Template;

use crate::item::item_registry::{serialize_item_atlas, ItemAtlasEntry};
use crate::item::ItemAtlasTemplate;
use crate::item::{item_registry::ItemRegistry, Item};
use crate::logger::info;
use crate::template::{BpManifestTemplate, RpManifestTemplate};

use uuid::Uuid;

pub struct ScriptData {
    pub mc_server_version: String,
    pub mc_server_ui_version: String,
}

pub struct Pack<'a> {
    pub name: String,
    pub id: String,
    pub author: String,
    pub version: Vec<String>,
    pub description: String,
    pub use_scripts: bool,
    pub scripts: &'a Option<ScriptData>,
    pub item_registry: ItemRegistry<'a>,
}

impl<'a> Pack<'a> {
    pub fn new(
        name: String,
        id: String,
        author: String,
        version: Vec<String>,
        description: String,
        use_scripts: bool,
        scripts: &'a Option<ScriptData>,
    ) -> Self {
        info(
            format!("Registering Pack \"{}\"(\"{}\")", name, id),
            "[ PACK ]".to_string(),
        );
        let items: ItemRegistry<'_> = ItemRegistry::new();
        let pack = Self {
            name,
            id,
            author,
            version,
            description,
            scripts,
            use_scripts,
            item_registry: items.clone(),
        };
        pack
    }

    pub fn generate(&mut self, regenerate_manifests: Option<bool>) -> () {
        info(
            format!("Creating Pack \"{}\"(\"{}\")", &self.name, &self.id),
            "[ PACK ]".to_string(),
        );

        let _ = fs::create_dir_all(format!("./violet_crystal_results/packs/{}/BP", &self.id));
        let _ = fs::create_dir_all(format!("./violet_crystal_results/packs/{}/RP", &self.id));

        if match regenerate_manifests {
            Some(v) => v,
            None => true,
        } {
            let bp_manifest: String = BpManifestTemplate {
                name: &self.name.as_str(),
                author: &self.author.as_str(),
                description: &self.description.as_str(),
                use_scripts: &self.use_scripts,
                uuid_1: Uuid::new_v4().to_string().as_str(),
                uuid_2: Uuid::new_v4().to_string().as_str(),
                uuid_3: Uuid::new_v4().to_string().as_str(),
                server_ui_version: match &self.scripts {
                    Some(scripts) => scripts.mc_server_ui_version.as_str(),
                    None => "0.0.0",
                },
                server_version: match &self.scripts {
                    Some(scripts) => scripts.mc_server_version.as_str(),
                    None => "0.0.0",
                },
            }
            .render()
            .unwrap();
            match fs::write(
                format!(
                    "./violet_crystal_results/packs/{}/BP/manifest.json",
                    &self.id
                ),
                bp_manifest,
            ) {
                Ok(_) => (),
                Err(_) => (),
            };

            let rp_manifest: String = RpManifestTemplate {
                name: &self.name.as_str(),
                author: &self.author.as_str(),
                description: &self.description.as_str(),
                uuid_1: Uuid::new_v4().to_string().as_str(),
                uuid_2: Uuid::new_v4().to_string().as_str(),
            }
            .render()
            .unwrap();
            match fs::write(
                format!(
                    "./violet_crystal_results/packs/{}/RP/manifest.json",
                    &self.id
                ),
                rp_manifest,
            ) {
                Ok(_) => (),
                Err(_) => (),
            };

            let _ = match fs::write(
                format!(
                    "./violet_crystal_results/packs/{}/IMPORTANT.MD",
                    &self.id
                ),
                crate::constant::important::IMPORTANTMD,
            ) {
                Ok(_) => info("VioletCrystal has generated an IMPORTANT.MD file. Consider checking it before the next run".to_string(), "[ IMPORTANT ]".to_string()),
                Err(_) => (),
            };
        }

        self.generate_items();
    }

    pub fn register_item(&mut self, item: Item<'a>) {
        self.item_registry.add_item(item.clone());
        info(
            format!("Registering Item \"{}\"", &item.type_id),
            "[ ITEM ]".to_string(),
        );
    }

    fn generate_items(&mut self) {
        let _ = fs::create_dir_all(format!(
            "./violet_crystal_results/packs/{}/BP/items/",
            &self.id
        ));
        let _ = fs::create_dir_all(format!(
            "./violet_crystal_results/packs/{}/RP/textures/",
            &self.id
        ));

        let itreg = self.item_registry.clone();

        for item in itreg.items {
            extern crate jsonxf;
            info(
                format!("Generating Item \"{}\"", &item.type_id),
                "[ ITEM ]".to_string(),
            );
            let file_name: String = item
                .type_id
                .chars()
                .into_iter()
                .map(|el| if el == ':' { '_' } else { el })
                .collect();
            let content = item.serialize();
            let pretty_content = jsonxf::pretty_print(&content).unwrap();
            let _ = match fs::write(
                format!(
                    "./violet_crystal_results/packs/{}/BP/items/{}.item.json",
                    &self.id, &file_name
                ),
                pretty_content,
            ) {
                Ok(_) => "Ok!",
                Err(_) => "Err!",
            };
            let _ = fs::create_dir_all(format!(
                "./violet_crystal_results/packs/{}/RP/textures/items",
                &self.id
            ));
            let _ = match fs::copy(
                item.texture,
                format!(
                    "./violet_crystal_results/packs/{}/RP/textures/items/{}.png",
                    &self.id, &file_name
                ),
            ) {
                Ok(_) => "Ok!",
                Err(_) => "Err!",
            };
        }

        self.generate_item_atlas();
    }

    fn generate_item_atlas(&self) {
        let _ = fs::create_dir_all(format!(
            "./violet_crystal_results/packs/{}/RP/textures/items",
            &self.id
        ));
        let content_raw = ItemAtlasTemplate {
            name: &self.name,
            contents: serialize_item_atlas(&self.item_registry.item_atlas),
        }
        .render()
        .unwrap();
        let content = jsonxf::pretty_print(content_raw.as_str()).unwrap();
        let _ = match fs::write(
            format!(
                "./violet_crystal_results/packs/{}/RP/textures/item_texture.json",
                &self.id
            ),
            content,
        ) {
            Ok(_) => "Ok!",
            Err(_) => "Err!",
        };
    }
}
