pub mod constant;
pub mod item;
pub mod logger;
pub mod pack;
pub mod template;

#[cfg(test)]
mod tests {
    use crate::{
        item::{
            component::{ItemDamageComponent, ItemDisplayNameComponent, ItemIconComponent},
            Item,
        },
        pack::{Pack, ScriptData},
    };

    #[test]
    fn main() {
        let binding = Some(ScriptData {
            mc_server_ui_version: "1.2.0-beta".to_string(),
            mc_server_version: "1.9.0-beta".to_string(),
        });
        let mut pack = Pack::new(
            "Test".to_string(),
            "test".to_string(),
            "NaKeR".to_string(),
            vec!["1".to_string(), "0".to_string(), "0".to_string()],
            "Nothing here".to_string(),
            true,
            &binding,
        );

        pack.register_item(Item {
            type_id: "test:test".to_string(),
            texture: r"C:\Users\User\newgito\bluestone.png".to_string(),
            components: vec![
                &ItemDamageComponent { value: 3 },
                &ItemDisplayNameComponent { value: "Test" },
                &ItemIconComponent {
                    texture: "test:test",
                },
            ],
        });

        pack.generate(Some(false));
    }
}
