use askama::Template;

pub trait ItemComponent {
    fn serialize(&self) -> String;
}

// * ItemDamageComponent
#[derive(Template)]
#[template(
    path = "item_serialization/components/damage.json.jinja2",
    escape = "none"
)]
struct ItemDamageComponentTemplate {
    value: i32,
}
pub struct ItemDamageComponent {
    pub value: i32,
}
impl ItemComponent for ItemDamageComponent {
    fn serialize(&self) -> String {
        let value = self.value;
        let val = ItemDamageComponentTemplate { value: value }
            .render()
            .unwrap();
        val
    }
}

// * ItemDisplayNameComponent
#[derive(Template)]
#[template(
    path = "item_serialization/components/display_name.json.jinja2",
    escape = "none"
)]
struct ItemDisplayNameComponentTemplate<'a> {
    value: &'a str,
}
pub struct ItemDisplayNameComponent<'a> {
    pub value: &'a str,
}
impl<'a> ItemComponent for ItemDisplayNameComponent<'a> {
    fn serialize(&self) -> String {
        let value = self.value;
        let val: String = ItemDisplayNameComponentTemplate { value: value }
            .render()
            .unwrap();
        val
    }
}

// * ItemIconComponent
#[derive(Template)]
#[template(
    path = "item_serialization/components/icon.json.jinja2",
    escape = "none"
)]
struct ItemIconComponentTemplate<'a> {
    texture: &'a str,
}
pub struct ItemIconComponent<'a> {
    pub texture: &'a str,
}
impl<'a> ItemComponent for ItemIconComponent<'a> {
    fn serialize(&self) -> String {
        let value = self.texture;
        let val: String = ItemIconComponentTemplate { texture: value }
            .render()
            .unwrap();
        val
    }
}

// * ItemFuelComponent
#[derive(Template)]
#[template(
    path = "item_serialization/components/fuel.json.jinja2",
    escape = "none"
)]
struct ItemFuelComponentTemplate {
    duration: i32,
}
pub struct ItemFuelComponent {
    pub duration: i32,
}
impl ItemComponent for ItemFuelComponent {
    fn serialize(&self) -> String {
        let value = self.duration;
        let val: String = ItemFuelComponentTemplate { duration: value }
            .render()
            .unwrap();
        val
    }
}

// * ItemHandEquippedComponent
#[derive(Template)]
#[template(
    path = "item_serialization/components/hand_equipped.json.jinja2",
    escape = "none"
)]
struct ItemHandEquippedComponentTemplate {
    value: bool,
}
pub struct ItemHandEquippedComponent {
    pub value: bool,
}
impl ItemComponent for ItemHandEquippedComponent {
    fn serialize(&self) -> String {
        let value = self.value;
        let val: String = ItemHandEquippedComponentTemplate { value: value }
            .render()
            .unwrap();
        val
    }
}

// * ItemAllowOffHandComponent
#[derive(Template)]
#[template(
    path = "item_serialization/components/allow_off_hand.json.jinja2",
    escape = "none"
)]
struct ItemAllowOffHandComponentTemplate {
    value: bool,
}
pub struct ItemAllowOffHandComponent {
    pub value: bool,
}
impl ItemComponent for ItemAllowOffHandComponent {
    fn serialize(&self) -> String {
        let value = self.value;
        let val: String = ItemAllowOffHandComponentTemplate { value: value }
            .render()
            .unwrap();
        val
    }
}

// * ItemMaxStackValueComponent
#[derive(Template)]
#[template(
    path = "item_serialization/components/max_stack_value.json.jinja2",
    escape = "none"
)]
struct ItemMaxStackValueComponentTemplate {
    value: bool,
}
pub struct ItemMaxStackValueComponent {
    pub value: bool,
}
impl ItemComponent for ItemMaxStackValueComponent {
    fn serialize(&self) -> String {
        let value = self.value;
        let val: String = ItemMaxStackValueComponentTemplate { value: value }
            .render()
            .unwrap();
        val
    }
}

// * ItemDurabilityComponent
#[derive(Template)]
#[template(
    path = "item_serialization/components/durability.json.jinja2",
    escape = "none"
)]
struct ItemDurabilityComponentTemplate {
    min_chance: i32,
    max_chance: i32,
    durability: i32,
}
pub struct ItemDurabilityComponent {
    min_chance: i32,
    max_chance: i32,
    durability: i32,
}
impl ItemComponent for ItemDurabilityComponent {
    fn serialize(&self) -> String {
        let value = self.durability;
        let min_c = self.min_chance;
        let max_c = self.max_chance;
        let val: String = ItemDurabilityComponentTemplate {
            max_chance: max_c,
            min_chance: min_c,
            durability: value,
        }
        .render()
        .unwrap();
        val
    }
}
