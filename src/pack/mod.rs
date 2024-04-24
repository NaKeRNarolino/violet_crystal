use std::fs;

use crate::recipe::Recipe;
use askama::Template;
use fs_extra::dir;

use crate::item::item_registry::serialize_item_atlas;
use crate::item::ItemAtlasTemplate;
use crate::item::{item_registry::ItemRegistry, Item};
use crate::logger::info;
use crate::template::{BpManifestTemplate, RpManifestTemplate};

use uuid::Uuid;
use crate::block::Block;
use crate::block::block_registry::{BlockAtlasEntry, BlockAtlasTemplate, BlockRegistry, serialize_block_atlas, serialize_terrain_atlas};

pub struct ScriptData<'a> {
    pub mc_server_version: String,
    pub mc_server_ui_version: String,
    pub paired_scripts_folder: &'a str,
}

pub struct Pack<'a> {
    pub name: String,
    pub id: String,
    pub author: String,
    pub version: &'a str,
    pub description: String,
    pub use_scripts: bool,
    pub scripts: &'a Option<ScriptData<'a>>,
    pub dev_bp_folder: &'a str,
    pub dev_rp_folder: &'a str,
    pub icon: &'a str,
    pub item_registry: ItemRegistry<'a>,
    pub recipes: Vec<&'a dyn Recipe>,
    pub block_registry: BlockRegistry<'a>
}

impl<'a> Pack<'a> {
    pub fn new(
        name: String,
        id: String,
        author: String,
        version: &'a str,
        description: String,
        use_scripts: bool,
        dev_bp_folder: &'a str,
        dev_rp_folder: &'a str,
        icon: &'a str,
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
            dev_bp_folder,
            dev_rp_folder,
            icon,
            item_registry: items.clone(),
            recipes: Vec::new(),
            block_registry: BlockRegistry::new()
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
                version: &self.version,
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
                version: &self.version,
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

        let _ = fs::copy(
            &self.icon,
            format!(
                "./violet_crystal_results/packs/{}/BP/pack_icon.png",
                &self.id
            ),
        );

        let _ = fs::copy(
            &self.icon,
            format!(
                "./violet_crystal_results/packs/{}/RP/pack_icon.png",
                &self.id
            ),
        );

        if self.use_scripts {
            self.pair_scripts();
        }

        self.generate_items();
        self.generate_blocks();
        self.generate_recipes();
    }

    pub fn register_recipe<'b>(&mut self, recipe: &'a dyn Recipe) {
        self.recipes.push(recipe);
        info(
            format!("Registering recipe {}", recipe.id()),
            "[ RECIPE ]".to_string(),
        );
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

    pub fn build_to_dev(&self) {
        let _ = fs::create_dir_all(format!("{}/{}_BP", &self.dev_bp_folder, &self.id));
        let _ = fs::create_dir_all(format!("{}/{}_RP", &self.dev_rp_folder, &self.id));

        info(
            format!("Copying {}'s BP to DevBPFolder", &self.id),
            "[ PACK ]".to_string(),
        );
        let _ = fs_extra::dir::copy(
            format!("./violet_crystal_results/packs/{}/BP/", &self.id),
            format!("{}/{}_BP/", &self.dev_bp_folder, &self.id),
            &dir::CopyOptions::new().content_only(true).overwrite(true),
        );
        info(
            format!("Copying {}'s RP to DevRPFolder", &self.id),
            "[ PACK ]".to_string(),
        );
        let _ = fs_extra::dir::copy(
            format!("./violet_crystal_results/packs/{}/RP/", &self.id),
            format!("{}/{}_RP/", &self.dev_rp_folder, &self.id),
            &dir::CopyOptions::new().content_only(true).overwrite(true),
        );
    }

    pub fn pair_scripts(&self) {
        let _ = fs::create_dir_all(format!(
            "./violet_crystal_results/packs/{}/BP/scripts/",
            &self.id
        ));
        let path = self.scripts.as_ref().unwrap().paired_scripts_folder;
        let _ = fs_extra::dir::copy(
            path,
            format!("./violet_crystal_results/packs/{}/BP/scripts/", &self.id),
            &fs_extra::dir::CopyOptions::new()
                .overwrite(true)
                .content_only(true),
        );
        info(
            format!("Paired scripts from folder {}", path),
            "[ SCRIPTS ]".to_string(),
        )
    }

    fn generate_recipes(&self) {
        let _ = fs::create_dir_all(format!(
            "./violet_crystal_results/packs/{}/BP/recipes/",
            &self.id
        ));
        let iterator: &Vec<&dyn Recipe> = self.recipes.as_ref();
        for recipe in iterator {
            info(
                format!("Generating Recipe \"{}\"", &recipe.id()),
                "[ RECIPE ]".to_string(),
            );
            let file_name: String = recipe
                .id()
                .chars()
                .into_iter()
                .map(|el| if el == ':' { '_' } else { el })
                .collect();
            let content = recipe.serialize();
            let pretty_content = jsonxf::pretty_print(&content).unwrap();
            let _ = match fs::write(
                format!(
                    "./violet_crystal_results/packs/{}/BP/recipes/{}.recipe.json",
                    &self.id, &file_name
                ),
                pretty_content,
            ) {
                Ok(_) => "Ok!",
                Err(_) => "Err!",
            };
        }
    }

    fn generate_blocks(&mut self) {
        let _ = fs::create_dir_all(format!(
            "./violet_crystal_results/packs/{}/BP/blocks/",
            &self.id
        ));
        let _ = fs::create_dir_all(format!(
            "./violet_crystal_results/packs/{}/RP/textures/",
            &self.id
        ));

        let itreg = self.block_registry.clone();

        for block in self.block_registry.blocks.iter() {
            extern crate jsonxf;
            info(
                format!("Generating Block \"{}\"", &block.type_id),
                "[ BLOCK ]".to_string(),
            );
            let file_name: String = block
                .type_id
                .chars()
                .into_iter()
                .map(|el| if el == ':' { '_' } else { el })
                .collect();
            let content = block.serialize();
            let pretty_content = jsonxf::pretty_print(&content).unwrap();
            let _ = match fs::write(
                format!(
                    "./violet_crystal_results/packs/{}/BP/blocks/{}.block.json",
                    &self.id, &file_name
                ),
                pretty_content,
            ) {
                Ok(_) => "Ok!",
                Err(_) => "Err!",
            };
            let _ = fs::create_dir_all(format!(
                "./violet_crystal_results/packs/{}/RP/textures/blocks",
                &self.id
            ));
            let _ = match fs::copy(
                block.clone().texture_set,
                format!(
                    "./violet_crystal_results/packs/{}/RP/textures/blocks/{}.png",
                    &self.id, &file_name
                ),
            ) {
                Ok(_) => "Ok!",
                Err(_) => "Err!",
            };
        }

        self.generate_block_atlas();
        self.generate_terrain_atlas();
    }

    fn generate_block_atlas(&self) {
        let _ = fs::create_dir_all(format!(
            "./violet_crystal_results/packs/{}/RP/textures/blocks",
            &self.id
        ));
        let content_raw = BlockAtlasTemplate {
            content: serialize_block_atlas(&self.block_registry.block_atlas),
        }
            .render()
            .unwrap();
        let content = jsonxf::pretty_print(content_raw.as_str()).unwrap();
        let _ = match fs::write(
            format!(
                "./violet_crystal_results/packs/{}/RP/blocks.json",
                &self.id
            ),
            content,
        ) {
            Ok(_) => "Ok!",
            Err(_) => "Err!",
        };
    }

    fn generate_terrain_atlas(&self) {
        let _ = fs::create_dir_all(format!(
            "./violet_crystal_results/packs/{}/RP/textures/",
            &self.id
        ));
        let content_raw = BlockAtlasTemplate {
            content: serialize_terrain_atlas(&self.block_registry.terrain_atlas),
        }
            .render()
            .unwrap();
        let content = jsonxf::pretty_print(content_raw.as_str()).unwrap();
        let _ = match fs::write(
            format!(
                "./violet_crystal_results/packs/{}/RP/textures/terrain_texture.json",
                &self.id
            ),
            content,
        ) {
            Ok(_) => "Ok!",
            Err(_) => "Err!",
        };
    }

    pub fn register_block(&mut self, block: Block<'a>) {
        self.block_registry.add_block(block.clone());
        info(format!("Registering block {}", block.type_id), "[ BLOCK ]".to_string());
    }
}
