use std::{
    cell::{Ref, RefCell},
    collections::{HashMap, HashSet},
    marker::PhantomData,
};

use crate::{
    computed::{Computed, ComputedStore},
    context_ref::ContextRef,
    data::{Data, DataRef, DataStore},
};

pub struct Context {
    pub next_id: usize,

    // Data properties
    data: HashMap<usize, DataStore>,
    // Computed properties
    computed: HashMap<usize, ComputedStore>,
    computed_data: HashMap<usize, DataStore>,
}

impl Context {
    pub fn new() -> Self {
        Context {
            next_id: 0,
            data: HashMap::new(),
            computed: HashMap::new(),
            computed_data: HashMap::new(),
        }
    }

    pub fn next_id(&mut self) -> usize {
        let id = self.next_id;
        self.next_id += 1;
        id
    }

    pub fn ref_is_data<D: Data>(&self, ref_: &DataRef<D>) -> bool {
        self.data.contains_key(&ref_.id)
    }
    pub fn ref_is_computed<D: Data>(&self, ref_: &DataRef<D>) -> bool {
        self.computed.contains_key(&ref_.id)
    }

    // Data management
    pub fn data<D: Data>(&mut self, data: D) -> DataRef<D> {
        let id = self.next_id();
        let data_store = DataStore::new(data);
        self.data.insert(id, data_store);
        DataRef {
            id,
            phantom: PhantomData,
        }
    }
    pub fn get_data<D: Data>(&self, data_ref: DataRef<D>) -> Option<D> {
        self.data
            .get(&data_ref.id)
            .and_then(|data_store| data_store.get_cloned_ref())
    }
    pub fn set_data<D: Data>(&mut self, data_ref: DataRef<D>, data: D) -> D {
        let old_data = self
            .data
            .insert(data_ref.id, DataStore::new(data))
            .expect("Old data not found on set");
        old_data.get().unwrap()
    }

    // Data management (computed)
    pub fn computed<D: Data, C: Computed<D> + 'static>(
        &mut self,
        context_ref: ContextRef,
        computed: C,
    ) -> DataRef<D> {
        let id = self.next_id();
        let computed_store = ComputedStore::new(computed);
        self.computed.insert(id, computed_store);
        let data_ref = DataRef {
            id,
            phantom: PhantomData,
        };
        self.computed_execute_getter(context_ref, data_ref);
        data_ref
    }
    pub fn get_computed<D: Data>(&self, data_ref: DataRef<D>) -> Option<D> {
        self.computed_data
            .get(&data_ref.id)
            .and_then(|computed_store| computed_store.get_cloned_ref())
    }
    pub fn set_computed<D: Data>(
        &mut self,
        context_ref: ContextRef,
        data_ref: DataRef<D>,
        data: D,
    ) -> D {
        let new_data = DataStore::new(data);
        self.computed_execute_setter(context_ref, data_ref, &new_data);
        let old_data = self
            .computed_data
            .insert(data_ref.id, new_data)
            .expect("Old data not found on set");
        old_data.get().unwrap()
    }
    fn computed_execute_getter<D: Data>(&mut self, context_ref: ContextRef, data_ref: DataRef<D>) {
        let computed_store = self
            .computed
            .get_mut(&data_ref.id)
            .expect("Computed data not found on populate");
        let computed_data = computed_store.get(context_ref);
        self.computed_data.insert(data_ref.id, computed_data);
    }
    fn computed_execute_setter<D: Data>(
        &mut self,
        context_ref: ContextRef,
        data_ref: DataRef<D>,
        data: &DataStore,
    ) {
        let computed_store = self
            .computed
            .get_mut(&data_ref.id)
            .expect("Computed data not found on populate");
        computed_store.set(context_ref, &data);
    }
}
