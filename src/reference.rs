use crate::{context_ref::ContextRef, data::Data};
use std::marker::PhantomData;

#[derive(Clone)]
pub struct Ref<D: Data> {
    pub id: usize,
    pub(crate) phantom: PhantomData<D>,
}

impl<D: Data> Ref<D> {
    pub fn new(id: usize) -> Self {
        Ref {
            id,
            phantom: PhantomData,
        }
    }
    pub fn id(&self) -> usize {
        self.id
    }

    pub fn get<C: AsRef<ContextRef>>(&self, context: C) -> Option<D> {
        context.as_ref().get(*self)
    }
    pub fn set<C: AsRef<ContextRef>>(&self, context: C, data: D) -> D {
        context.as_ref().set(*self, data)
    }

    pub fn is_value<C: AsRef<ContextRef>>(&self, context: C) -> bool {
        context.as_ref().id_is_value(self.id)
    }
    pub fn is_computed<C: AsRef<ContextRef>>(&self, context: C) -> bool {
        context.as_ref().id_is_computed(self.id)
    }
}

impl<D: Data> Copy for Ref<D> {}
