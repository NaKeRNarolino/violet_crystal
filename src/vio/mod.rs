#[derive(Clone, Copy)]
pub struct Vec3 {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

#[derive(Clone, Copy)]
pub struct Identifier<'a> {
    pub namespace: &'a str,
    pub value: &'a str,
}

impl Identifier<'_> {
    pub fn render(&self) -> String {
        format!("{}:{}", self.namespace, self.value)
    }
}
