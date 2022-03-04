use std::{
    collections::{HashMap, HashSet},
    marker::PhantomData,
};

use crate::{
    computed::{Computed, ComputedStore},
    context_ref::ContextRef,
    data::Data,
    reference::Ref,
    value::ValueStore,
};

pub struct Context {
    pub next_id: usize,

    // Data properties
    values: HashMap<usize, ValueStore>,
    // Computed properties
    computed: HashMap<usize, ComputedStore>,
    computed_data: HashMap<usize, ValueStore>,
    currently_computing: Vec<usize>,
    // Dependencies
    dependencies: HashMap<usize, HashSet<usize>>,
}

impl Context {
    pub fn new() -> Self {
        Context {
            next_id: 0,
            values: HashMap::new(),
            computed: HashMap::new(),
            computed_data: HashMap::new(),
            currently_computing: Vec::new(),
            dependencies: HashMap::new(),
        }
    }

    pub fn next_id(&mut self) -> usize {
        let id = self.next_id;
        self.next_id += 1;
        id
    }

    pub fn id_is_value(&self, id: usize) -> bool {
        self.values.contains_key(&id)
    }
    pub fn id_is_computed(&self, id: usize) -> bool {
        self.computed.contains_key(&id)
    }

    pub fn add_dependency_if_computing(&mut self, dependency: usize) {
        // Check if currently computing something
        if self.currently_computing.len() > 0 {
            // Add dependency
            let currently_computing_id =
                self.currently_computing[self.currently_computing.len() - 1];
            self.dependencies
                .entry(currently_computing_id)
                .or_insert_with(HashSet::new)
                .insert(dependency);
        }
    }
    pub fn trigger_compute_if_dependency_is_set(
        &mut self,
        context_ref: ContextRef,
        dependency: usize,
    ) {
        // Go through each compute and check if it depends on the dependency
        let mut to_compute = Vec::new();
        for (id, dependencies) in self.dependencies.iter() {
            if dependencies.contains(&dependency) {
                to_compute.push(*id);
            }
        }

        // Compute
        for id in to_compute {
            self.computed_execute_getter(context_ref.clone(), id);
        }
    }

    // Value management
    pub fn value<D: Data>(&mut self, data: D) -> Ref<D> {
        let id = self.next_id();
        let data_store = ValueStore::new(data);
        self.values.insert(id, data_store);
        Ref::new(id)
    }
    pub fn get_value<D: Data>(&mut self, id: Ref<D>) -> Option<D> {
        self.add_dependency_if_computing(id.id);
        self.values
            .get(&id.id)
            .and_then(|data_store| data_store.get_cloned_ref())
    }
    pub fn set_value<D: Data>(&mut self, context_ref: ContextRef, id: Ref<D>, data: D) -> D {
        let old_data = self
            .values
            .insert(id.id, ValueStore::new(data))
            .expect("Old data not found on set");
        self.trigger_compute_if_dependency_is_set(context_ref, id.id);
        old_data.get().unwrap()
    }

    // Data management (computed)
    pub fn computed<D: Data, C: Computed<D> + 'static>(
        &mut self,
        context_ref: ContextRef,
        computed: C,
    ) -> Ref<D> {
        let id = self.next_id();
        let computed_store = ComputedStore::new(computed);
        self.computed.insert(id, computed_store);
        let id = Ref::new(id);
        self.computed_execute_getter(context_ref, id.id);
        id
    }
    pub fn get_computed<D: Data>(&mut self, id: Ref<D>) -> Option<D> {
        self.add_dependency_if_computing(id.id);
        self.computed_data
            .get(&id.id)
            .and_then(|computed_store| computed_store.get_cloned_ref())
    }
    pub fn set_computed<D: Data>(&mut self, context_ref: ContextRef, id: Ref<D>, data: D) -> D {
        let new_data = ValueStore::new(data);
        self.computed_execute_setter::<D>(context_ref, id.id, &new_data);
        let old_data = self
            .computed_data
            .insert(id.id, new_data)
            .expect("Old data not found on set");
        old_data.get().unwrap()
    }
    fn computed_execute_getter(&mut self, context_ref: ContextRef, id: usize) {
        let computed_store = self
            .computed
            .get_mut(&id)
            .expect("Computed data not found on populate");
        self.currently_computing.push(id);
        let computed_data = computed_store.get(context_ref.clone());
        self.currently_computing.pop();
        self.computed_data.insert(id, computed_data);
        self.trigger_compute_if_dependency_is_set(context_ref, id);
    }
    fn computed_execute_setter<D: Data>(
        &mut self,
        context_ref: ContextRef,
        id: usize,
        data: &ValueStore,
    ) {
        let computed_store = self
            .computed
            .get_mut(&id)
            .expect("Computed data not found on populate");
        computed_store.set(context_ref, &data);
    }
}
