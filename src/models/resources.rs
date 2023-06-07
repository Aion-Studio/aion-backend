
#[derive(Clone, Debug, PartialEq)]
pub enum Resource {
    Aion(i32),
    Material(MaterialType),
    Valor(i32),
    NexusShard(i32),
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
