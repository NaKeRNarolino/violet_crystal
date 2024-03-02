use std::fs;

use askama::Template;

use crate::item::{self, Item};
use crate::logger::info;
use crate::template::BpManifestTemplate;

use uuid::{uuid, Uuid};

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
        Self {
            name,
            id,
            author,
            version,
            description,
            scripts,
            use_scripts,
        }
    }
    pub fn create(&mut self) {
        info(
            format!("Creating Pack \"{}\"(\"{}\")", &self.name, &self.id),
            "[ PACK ]".to_string(),
        );

        let _ = fs::create_dir_all(format!("./violet_crystal_results/packs/{}/BP", &self.id));
        let _ = fs::create_dir_all(format!("./violet_crystal_results/packs/{}/RP", &self.id));

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
    }

    pub fn regiter_item(&mut self, item: Item) {}
}
