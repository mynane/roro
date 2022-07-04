use std::sync::{Arc, Mutex};

use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Caches {
    pub name: String,
    pub age: String,
}

impl Caches {
    pub fn age(&mut self, name: String) {
        self.name = name;
    }
}

#[derive(Default, Debug)]
pub struct CachesState(pub Arc<Mutex<Caches>>);

#[derive(Default, Debug)]
pub struct TestState(pub Arc<Mutex<Caches>>);
