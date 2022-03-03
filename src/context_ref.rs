use std::{cell::RefCell, rc::Rc};

use crate::{
    computed::Computed,
    context::Context,
    data::{Data, DataRef},
};

pub struct ContextRef {
    context: *mut usize,
    counter: *mut u64,
}

impl ContextRef {
    pub fn new() -> Self {
        ContextRef {
            context: Box::into_raw(Box::new(Context::new())) as *mut usize,
            counter: Box::into_raw(Box::new(1)) as *mut u64,
        }
    }

    pub fn get_context(&self) -> &Context {
        unsafe { &*(self.context as *mut Context) }
    }
    pub fn get_context_mut(&mut self) -> &mut Context {
        unsafe { &mut *(self.context as *mut Context) }
    }
    pub fn get_counter(&self) -> u64 {
        unsafe { *(self.counter as *mut u64) }
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

impl Clone for ContextRef {
    fn clone(&self) -> Self {
        let copy = ContextRef {
            context: self.context,
            counter: self.counter,
        };
        unsafe {
            *(self.counter as *mut u64) += 1;
        }
        copy
    }
}

impl Drop for ContextRef {
    fn drop(&mut self) {
        unsafe {
            *(self.counter as *mut u64) -= 1;
            if *(self.counter as *mut u64) == 0 {
                println!("Dropping context");
                Box::from_raw(self.context as *mut Context);
                Box::from_raw(self.counter as *mut u64);
            }
        }
    }
}
