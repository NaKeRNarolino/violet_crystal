use askama::Template;
use crate::block::Block;
use crate::vio::Identifier;

pub trait BlockState {
    fn serialize(&self) -> String;
}

pub struct BoolBlockState<'a> {
    pub id: Identifier<'a>
}

#[derive(Template)]
#[template(path = "block_serialization/state/bool_state.json.jinja2", escape = "none")]
struct BoolBlockStateTemplate {
    identifier: String,
}

impl BlockState for BoolBlockState<'_> {
    fn serialize(&self) -> String {
        BoolBlockStateTemplate {
            identifier: self.id.render().to_string()
        }.render().unwrap()
    }
}

pub struct NumericBlockState<'a> {
    pub id: Identifier<'a>,
    pub values: Vec<i32>
}

#[derive(Template)]
#[template(path = "block_serialization/state/numeric_state.json.jinja2", escape = "none")]
struct VariableBlockStateTemplate {
    identifier: String,
    values: String
}

impl BlockState for NumericBlockState<'_> {
    fn serialize(&self) -> String {
        VariableBlockStateTemplate {
            identifier: self.id.render().to_string(),
            values: format!("{:?}", self.values)
        }.render().unwrap()
    }
}

pub struct StringBlockState<'a> {
    pub id: Identifier<'a>,
    pub values: Vec<&'a str>
}

impl BlockState for StringBlockState<'_> {
    fn serialize(&self) -> String {
        VariableBlockStateTemplate {
            identifier: self.id.render().to_string(),
            values: format!("{:?}", self.values)
        }.render().unwrap()
    }
}

pub struct RangedBlockState<'a> {
    pub id: Identifier<'a>,
    pub min: i32,
    pub max: i32
}

#[derive(Template)]
#[template(path = "block_serialization/state/ranged_state.json.jinja2", escape = "none")]
struct RangedBlockStateTemplate {
    identifier: String,
    min: i32,
    max: i32
}

impl BlockState for RangedBlockState<'_> {
    fn serialize(&self) -> String {
        RangedBlockStateTemplate {
            identifier: self.id.render().to_string(),
            min: self.min,
            max: self.max
        }.render().unwrap()
    }
}
