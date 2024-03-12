pub trait BlockComponent {
    fn serialize(&self) -> String;
}

// * BlockCollisionBoxComponent

pub struct BlockCollisionBoxComponent {
    enabled: bool,
    origin: Option<Vec<i32>>,
    size: Option<Vec<i32>>,
}

impl BlockComponent for BlockCollisionBoxComponent {}
