use std::sync::{Arc, Mutex};

use serde::{Deserialize, Serialize};

use crate::core::{catalogue::Catalogues, configs::Configs, posts::Posts};

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

#[derive(Debug, Default)]
pub struct ConfigsState(pub Arc<Mutex<Configs>>);

#[derive(Debug, Default)]
pub struct CataloguesState(pub Arc<Mutex<Catalogues>>);

#[derive(Debug, Default)]
pub struct PostsState(pub Arc<Mutex<Posts>>);
