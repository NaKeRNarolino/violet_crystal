pub mod constant;
pub mod item;
pub mod logger;
pub mod pack;
pub mod template;

#[cfg(test)]
mod tests {
    use crate::{
        item::{
            component::{
                ItemCustomComponents, ItemDamageComponent, ItemDisplayNameComponent,
                ItemDurabilityComponent, ItemFuelComponent, ItemIconComponent, ItemRepairEntry,
                ItemRepairableComponent,
            },
            Item,
        },
        pack::{Pack, ScriptData},
    };

    #[test]
    fn main() {
        let scripts = Some(ScriptData {
            mc_server_ui_version: "1.2.0-beta".to_string(),
            mc_server_version: "1.9.0-beta".to_string(),
        });
        let mut pack = Pack::new(
            "Test".to_string(),
            "test".to_string(),
            "NaKeR".to_string(),
            "1, 0, 0",
            "Nothing here".to_string(),
            true,
            r"C:\Users\User\AppData\Roaming\.minecraft_bedrock\installations\Latest Release\packageData\development_behavior_packs",
            "",
            r"C:\Users\User\newgito\bluestone.png",
            &scripts,
        );

        let item_repairable = ItemRepairableComponent {
            repair_entries: vec![ItemRepairEntry {
                items: vec!["minecraft:stick"],
                amount: "10".to_string(),
            }],
        };
        let binding = ItemCustomComponents {
            components: vec!["vc:custom_components"],
        };
        pack.register_item(Item {
            type_id: "test:test".to_string(),
            texture: r"C:\Users\User\newgito\bluestone.png".to_string(),
            components: vec![
                &ItemDamageComponent { value: 3 },
                &ItemDisplayNameComponent { value: "Test" },
                &ItemIconComponent {
                    texture: "test_test",
                },
                &ItemFuelComponent { duration: 10 },
                &item_repairable,
                &binding,
            ],
        });

        pack.generate(Some(true));
        pack.build_to_dev();
    }
}
