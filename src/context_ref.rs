use std::{cell::RefCell, rc::Rc};

use crate::{
    computed::Computed,
    context::Context,
    data::{Data, DataRef},
};

#[derive(Clone)]
pub struct ContextRef {
    context: *mut usize,
}

impl ContextRef {
    pub fn new() -> Self {
        ContextRef {
            context: Box::into_raw(Box::new(Context::new())) as *mut usize,
        }
    }

    pub fn get_context(&self) -> &Context {
        unsafe { &*(self.context as *mut Context) }
    }
    pub fn get_context_mut(&mut self) -> &mut Context {
        unsafe { &mut *(self.context as *mut Context) }
    }

    // Data management
    pub fn data<D: Data>(&mut self, data: D) -> DataRef<D> {
        self.get_context_mut().data(data)
    }
    pub fn computed<D: Data, C: Computed<D> + 'static>(&mut self, computed: C) -> DataRef<D> {
        let self_clone = self.clone();
        self.get_context_mut().computed(self_clone, computed)
    }
    // Generic get and set
    pub fn get<D: Data>(&self, data_ref: DataRef<D>) -> Option<D> {
        if self.get_context().ref_is_data(&data_ref) {
            self.get_context().get_data(data_ref)
        } else if self.get_context().ref_is_computed(&data_ref) {
            self.get_context().get_computed(data_ref)
        } else {
            None
        }
    }
    pub fn set<D: Data>(&mut self, data_ref: DataRef<D>, data: D) -> D {
        if self.get_context().ref_is_data(&data_ref) {
            self.get_context_mut().set_data(data_ref, data)
        } else if self.get_context().ref_is_computed(&data_ref) {
            let self_clone = self.clone();
            self.get_context_mut()
                .set_computed(self_clone, data_ref, data)
        } else {
            data
        }
    }
}

// WARNING: Currently context is never dropped
