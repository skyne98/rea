use std::{collections::HashMap, marker::PhantomData};

use crate::data::{Data, DataRef, DataStore};

pub struct Context {
    pub next_id: usize,
    data: HashMap<usize, DataStore>,
}

impl Context {
    pub fn new() -> Self {
        Context {
            next_id: 0,
            data: HashMap::new(),
        }
    }

    pub fn next_id(&mut self) -> usize {
        let id = self.next_id;
        self.next_id += 1;
        id
    }

    // CRUD for data
    pub fn insert<D: Data>(&mut self, data: D) -> DataRef<D> {
        let id = self.next_id();
        self.data.insert(id, DataStore::new(data));
        DataRef {
            id,
            phantom: PhantomData,
        }
    }
    pub fn get<D: Data>(&self, reference: DataRef<D>) -> Option<&D> {
        self.data
            .get(&reference.id)
            .and_then(|store| store.get_ref())
    }
    pub fn get_mut<D: Data>(&mut self, reference: DataRef<D>) -> Option<&mut D> {
        self.data
            .get_mut(&reference.id)
            .and_then(|store| store.get_mut())
    }
    pub fn remove<D: Data>(&mut self, reference: DataRef<D>) -> Option<D> {
        self.data
            .remove(&reference.id)
            .and_then(|store| store.get())
    }
}
