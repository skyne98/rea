use std::{borrow::Borrow, cell::RefCell, rc::Rc};

use crate::{
    context_ref::ContextRef,
    data::{Data, DataStore},
};

// Trait with getter and optional setter
pub trait Computed<D: Data> {
    fn get(&self, context: ContextRef) -> D;
    fn set(&self, context: ContextRef, data: D);
}

// Auto-implement the trait for a function as a getter only computed
impl<D: Data, G> Computed<D> for G
where
    G: Fn(ContextRef) -> D + 'static,
{
    fn get(&self, context: ContextRef) -> D {
        self(context)
    }

    fn set(&self, context: ContextRef, _data: D) {
        // Do nothing
    }
}
// Auto-implement the trait for a tuple of getter and setter
impl<D: Data, G, S> Computed<D> for (G, S)
where
    G: Fn(ContextRef) -> D + 'static,
    S: Fn(ContextRef, D) -> D + 'static,
{
    fn get(&self, context: ContextRef) -> D {
        (self.0)(context)
    }

    fn set(&self, context: ContextRef, data: D) {
        (self.1)(context, data);
    }
}

// Computed store (like DataStore)
pub(crate) struct ComputedStore {
    getter: Box<dyn Fn(ContextRef) -> DataStore>,
    setter: Option<Box<dyn Fn(ContextRef, &DataStore)>>,
}

impl ComputedStore {
    pub fn new<D: Data, C: Computed<D> + 'static>(computed: C) -> Self {
        let computed_ref = Rc::new(computed);
        let getter_ref = Rc::clone(&computed_ref);
        let setter_ref = Rc::clone(&computed_ref);

        ComputedStore {
            getter: Box::new(move |context| DataStore::new(getter_ref.get(context))),
            setter: Some(Box::new(move |context, data| {
                setter_ref.set(context, data.get_cloned_ref().unwrap());
            })),
        }
    }
    pub fn get(&self, context: ContextRef) -> DataStore {
        (self.getter)(context)
    }
    pub fn set(&self, context: ContextRef, data: &DataStore) {
        if let Some(setter) = self.setter.as_ref() {
            (setter)(context, data);
        }
    }
}
