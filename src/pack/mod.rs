mod pack_registry;

pub struct ScriptData {
    pub mc_server_version: String,
    pub mc_server_ui_version: String,
}

pub struct Pack {
    pub name: String,
    pub author: String,
    pub version: Vec<String>,
    pub description: String,
    pub scripts: Option<ScriptData>,
}
