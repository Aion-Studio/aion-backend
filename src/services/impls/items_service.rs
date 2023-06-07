#![allow(dead_code)]

use std::{sync::Arc, collections::HashMap};

use tokio::sync::RwLock;

use crate::models::hero::Item;

//allow unused
pub struct ItemService {
    pub items: Arc<RwLock<HashMap<String, Item>>>,
    // pub materials: Arc<RwLock<HashMap<String, Material>>>,
}

impl ItemService {
    pub fn new() -> Self {
        Self {
            items: Arc::new(RwLock::new(HashMap::new())),
            // materials: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn add_item(&self, item: Item) {
        let mut items = self.items.write().await;
        println!("Adding item: {:?}", item);
        items.insert(item.id.clone(), item);
    }

    pub async fn remove_item(&self, item_id: &str) {
        let mut items = self.items.write().await;
        items.remove(item_id);
    }

    // pub async fn add_material(&self, material: Material) {
    //     let mut materials = self.materials.write().await;
    //     materials.insert(material.id.clone(), material);
    // }
    //
    // pub async fn remove_material(&self, material_id: &str) {
    //     let mut materials = self.materials.write().await;
    //     materials.remove(material_id);
    // }

    // Other methods for manipulating items and materials...
}
