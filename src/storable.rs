// use std::collections::HashMap;
// use std::sync::{Arc, Mutex};
//
// use crate::events::combat::CombatantIndex;
//
// #[derive(Debug, Clone)] // Useful for seeing the values during development
// pub enum StoreValue {
//     CombatEncounter {
//         id: String,
//         combatant1_id: String,
//         combatant2_id: String,
//         active_dots: Vec<String>,
//         current_turn: CombatantIndex,
//     },
//     // Add more variants as needed for other storable types
// }
//
// // impl Storable for StoreValue {
// //     fn get_id(&self) -> String {
// //         match self {
// //             StoreValue::CombatEncounter { id, .. } => id.clone(),
// //             // ... (Implement similarly for other variants)
// //         }
// //     }
// // }
//
// // A trait for items you want to store - adding a unique 'get_id' method is key
// // trait Storable: Send + Sync + 'static {
// //     fn get_id(&self) -> String;
// // }
//
// // The in-memory store
// pub struct MemoryStore {
//     data: Arc<Mutex<HashMap<String, StoreValue>>>,
// }
//
// impl MemoryStore {
//     pub fn new() -> Self {
//         MemoryStore {
//             data: Arc::new(Mutex::new(HashMap::new())),
//         }
//     }
//
//     fn store(&mut self, item: StoreValue) {
//         // Notice we store StoreValue directly
//         let mut data = self.data.lock().unwrap();
//         let id = item.get_id();
//         data.insert(id, item);
//     }
//
//     fn retrieve(&self, id: &str) -> Option<StoreValue> {
//         let data = self.data.lock().unwrap();
//         data.get(id).cloned()
//     }
// }
