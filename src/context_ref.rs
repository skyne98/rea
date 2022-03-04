use crate::{computed::Computed, context::Context, data::Data, reference::Ref};

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
    pub fn get_context_mut(&self) -> &mut Context {
        unsafe { &mut *(self.context as *mut Context) }
    }
    pub fn get_counter(&self) -> u64 {
        unsafe { *(self.counter as *mut u64) }
    }

    pub fn id_is_value(&self, id: usize) -> bool {
        self.get_context().id_is_value(id)
    }
    pub fn id_is_computed(&self, id: usize) -> bool {
        self.get_context().id_is_computed(id)
    }

    // Data management
    pub fn value<D: Data>(&self, data: D) -> Ref<D> {
        self.get_context_mut().value(data)
    }
    pub fn computed<D: Data, C: Computed<D> + 'static>(&self, computed: C) -> Ref<D> {
        let self_clone = self.clone();
        self.get_context_mut().computed(self_clone, computed)
    }
    // Generic get and set
    pub fn get<D: Data>(&self, id: Ref<D>) -> Option<D> {
        if self.id_is_value(id.id) {
            self.get_context_mut().get_value(id)
        } else if self.id_is_computed(id.id) {
            self.get_context_mut().get_computed(id)
        } else {
            None
        }
    }
    pub fn set<D: Data>(&self, id: Ref<D>, data: D) -> D {
        let self_clone = self.clone();
        if self.id_is_value(id.id) {
            self.get_context_mut().set_value(self_clone, id, data)
        } else if self.id_is_computed(id.id) {
            self.get_context_mut().set_computed(self_clone, id, data)
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
                Box::from_raw(self.context as *mut Context);
                Box::from_raw(self.counter as *mut u64);
            }
        }
    }
}

impl AsRef<Context> for ContextRef {
    fn as_ref(&self) -> &Context {
        self.get_context()
    }
}

impl AsRef<ContextRef> for ContextRef {
    fn as_ref(&self) -> &ContextRef {
        self
    }
}
