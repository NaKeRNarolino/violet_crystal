pub mod block;
pub mod constant;
pub mod item;
pub mod logger;
pub mod pack;
pub mod recipe;
pub mod template;
pub mod vio;

#[cfg(test)]
mod tests {
    use crate::{
        item::{
            component::{
                ItemAllowOffHandComponent, ItemDamageComponent, ItemDisplayNameComponent,
                ItemFuelComponent, ItemIconComponent, ItemRepairEntry, ItemRepairableComponent,
            },
            Item,
        },
        pack::{Pack, ScriptData},
        recipe::{FurnaceRecipe, RecipeInputOrOutput, ShapedRecipe, ShapelessRecipe},
    };
    use crate::block::Block;
    use crate::block::component::{BlockCollisionBoxComponent, BlockCraftingTableComponent, BlockDestructibleByExplosionComponent, BlockDestructibleByMiningComponent};
    use crate::item::item_registry::ItemAtlasEntry;
    use crate::vio::{Identifier, Vec3};

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

        pack.register_item_texture(ItemAtlasEntry {
            id: "test_test".to_string(),
            texture_name: "test_test".to_string(),
            path: r"C:\Users\User\newgito\bluestone.png".to_string(),
        });

        pack.register_item(Item {
            type_id: Identifier {
                namespace: "test",
                value: "test"
            },
            components: vec![
                &ItemDamageComponent { value: 3 },
                &ItemDisplayNameComponent { value: "Test" },
                &ItemIconComponent {
                    texture: "test_test",
                },
                &ItemFuelComponent { duration: 10 },
                &item_repairable,
                &ItemAllowOffHandComponent { value: true },
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

        let test_item_shapeless_recipe = ShapelessRecipe {
            id: "test:test_shapeless",
            tags: vec!["crafting_table"],
            ingredients: vec![
                RecipeInputOrOutput {
                    use_tag: false,
                    item: Some("minecraft:bread"),
                    count: Some(1),
                    data: None,
                    tag: None,
                    key: None,
                },
                RecipeInputOrOutput {
                    use_tag: false,
                    item: Some("minecraft:amethyst_shard"),
                    count: Some(1),
                    data: None,
                    tag: None,
                    key: None,
                },
            ],
            result: RecipeInputOrOutput {
                use_tag: false,
                item: Some("test:test"),
                count: Some(1),
                data: Some(0),
                tag: None,
                key: None,
            },
        };
        let test_item_shaped_recipe = ShapedRecipe {
            id: "test:test_shaped",
            tags: vec!["crafting_table"],
            pattern: vec!["#  ", " # ", "  #"],
            ingredients: vec![RecipeInputOrOutput {
                use_tag: false,
                key: Some("#"),
                item: Some("minecraft:amethyst_shard"),
                data: Some(0),
                count: None,
                tag: None,
            }],
            result: RecipeInputOrOutput {
                use_tag: false,
                item: Some("test:test"),
                count: Some(6),
                data: Some(0),
                tag: None,
                key: None,
            },
        };

        pack.register_recipe(&test_item_recipe);
        pack.register_recipe(&test_item_shapeless_recipe);
        pack.register_recipe(&test_item_shaped_recipe);
        
        let crafting_table_component = &BlockCraftingTableComponent {
            name: "super_duper_crafting_table",
            tags: vec!["super_duper_crafting_table"]
        };

        pack.register_block(Block {
            type_id: Identifier {
                namespace: "amex",
                value: "test"
            },
            components: vec![
                &BlockCollisionBoxComponent {
                    origin: Some(Vec3 {
                        x: 0, y: 0, z: 0
                    }),
                    size: Some(Vec3 {
                        x: 16, y: 16, z: 16
                    }),
                    enabled: true
                },
                crafting_table_component,
                &BlockDestructibleByExplosionComponent {
                    explosion_resistance: Some(1.0)
                },
                &BlockDestructibleByMiningComponent {
                    seconds_to_destroy: Some(10.0)
                }
            ],
            texture_set: r"C:\Users\User\OneDrive\Рабочий стол\chipped_be\i2bdata\acacia_crate_top.png".to_string(),
            sound: "stone".to_string(),
        });

        pack.generate(Some(false));
        pack.build_to_dev();
    }
}
