pub mod constant;
pub mod item;
pub mod logger;
pub mod pack;
pub mod recipe;
pub mod template;

#[cfg(test)]
mod tests {
    use crate::{
        item::{
            component::{
                ItemDamageComponent, ItemDisplayNameComponent, ItemFuelComponent,
                ItemIconComponent, ItemRepairEntry, ItemRepairableComponent,
            },
            Item,
        },
        pack::{Pack, ScriptData},
        recipe::{FurnaceRecipe, RecipeInputOrOutput},
    };

    #[test]
    fn main() {
        let scripts = Some(ScriptData {
            mc_server_ui_version: "1.2.0-beta".to_string(),
            mc_server_version: "1.9.0-beta".to_string(),
            paired_scripts_folder: r"./src-scripts",
        });
        let mut pack = Pack::new(
            "TestThird".to_string(),
            "testThird".to_string(),
            "NaKeR".to_string(),
            "1, 0, 0",
            "Nothing here".to_string(),
            true,
            r"C:\Users\User\AppData\Roaming\.minecraft_bedrock\installations\Latest Release\packageData\development_behavior_packs",
            r"C:\Users\User\AppData\Roaming\.minecraft_bedrock\installations\Latest Release\packageData\development_resource_packs",
            r"C:\Users\User\newgito\bluestone.png",
            &scripts,
        );

        let item_repairable = ItemRepairableComponent {
            repair_entries: vec![ItemRepairEntry {
                items: vec!["minecraft:stick"],
                amount: "10".to_string(),
            }],
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
            ],
        });

        let test_item_recipe = FurnaceRecipe {
            id: "test:test_recipe",
            tags: vec!["furnace"],
            input: RecipeInputOrOutput {
                use_tag: false,
                item: Some("minecraft:stick"),
                count: None,
                data: None,
                tag: None,
                key: None,
            },
            output: "test:test",
        };

        pack.register_recipe(&test_item_recipe);

        pack.generate(Some(false));
        pack.build_to_dev();
    }
}
