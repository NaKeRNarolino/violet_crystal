use askama::Template;

#[derive(Template)]
#[template(path = "bp_manifest.json", escape = "none")]
pub struct BpManifestTemplate<'a> {
    pub author: &'a str,
    pub name: &'a str,
    pub description: &'a str,
    pub uuid_1: &'a str,
    pub uuid_2: &'a str,
    pub uuid_3: &'a str,
    pub use_scripts: &'a bool,
    pub server_version: &'a str,
    pub server_ui_version: &'a str,
}
