use askama::Template;

pub trait Recipe {
    fn serialize(&self) -> String;
    fn id(&self) -> String;
}

pub struct RecipeInputOrOutput<'a> {
    pub use_tag: bool,
    pub item: Option<&'a str>,
    pub data: Option<i32>,
    pub count: Option<i32>,
    pub tag: Option<&'a str>,
    pub key: Option<&'a str>,
}

#[derive(Template)]
#[template(
    path = "recipe_serialization/recipe_input.json.jinja2",
    escape = "none"
)]
struct RecipeInputTemplate {
    pub use_tag: bool,
    pub item: String,
    pub data: i32,
    pub count: i32,
    pub tag: String,
    pub has_count: bool,
    pub key: String,
    pub has_key: bool,
}

impl<'a> RecipeInputOrOutput<'a> {
    pub fn serialize(&self) -> String {
        RecipeInputTemplate {
            use_tag: self.use_tag,
            item: self.item.unwrap_or("null").to_string(),
            data: self.data.unwrap_or(0),
            count: self.count.unwrap_or(0),
            tag: self.tag.unwrap_or("null").to_string(),
            has_count: self.count.is_some(),
            key: self.key.unwrap_or("null").to_string(),
            has_key: self.key.is_some(),
        }
        .render()
        .unwrap()
    }
}

// * FurnaceRecipe

pub struct FurnaceRecipe<'a> {
    pub id: &'a str,
    pub tags: Vec<&'a str>,
    pub input: RecipeInputOrOutput<'a>,
    pub output: &'a str,
}
impl<'a> Recipe for FurnaceRecipe<'a> {
    fn serialize(&self) -> String {
        FurnaceRecipeTemplate {
            id: self.id.to_string(),
            tags: format!("{:?}", self.tags),
            input: self.input.serialize(),
            output: self.output.to_string(),
        }
        .render()
        .unwrap()
    }
    fn id(&self) -> String {
        self.id.to_string()
    }
}

#[derive(Template)]
#[template(
    path = "recipe_serialization/furnace_recipe.json.jinja2",
    escape = "none"
)]
struct FurnaceRecipeTemplate {
    id: String,
    tags: String,
    input: String,
    output: String,
}
