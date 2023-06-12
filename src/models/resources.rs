#[derive(Clone, Debug, PartialEq)]
pub enum Resource {
    Aion(i32),
    Valor(i32),
    NexusShard(i32),
    Oak(i32),
    IronOre(i32),
    Copper(i32),
    Silk(i32),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MaterialType {
    Oak,
    IronOre,
    Copper,
    Silk,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ResourceCost {
    resource_type: Resource,
    amount: i32,
}
