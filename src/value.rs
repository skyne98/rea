use std::any::{Any, TypeId};

use crate::data::Data;

pub(crate) struct ValueStore {
    // Stores type id and mutable pointer to data
    type_id: TypeId,
    data: *mut usize,
    empty: bool,
}

impl ValueStore {
    pub fn new<D: Data>(data: D) -> Self {
        let type_id = TypeId::of::<D>();
        let data = Box::new(data);
        let data_ptr = Box::into_raw(data) as *mut usize;
        ValueStore {
            type_id,
            data: data_ptr,
            empty: false,
        }
    }

    pub fn get_ref<D: Data>(&self) -> Option<&D> {
        if self.type_id == TypeId::of::<D>() {
            unsafe { Some(&*(self.data as *const D)) }
        } else {
            None
        }
    }
    pub fn get_cloned_ref<D: Data>(&self) -> Option<D> {
        if self.type_id == TypeId::of::<D>() {
            unsafe { Some((*(self.data as *const D)).clone()) }
        } else {
            None
        }
    }
    pub fn get<D: Data>(mut self) -> Option<D> {
        if self.type_id == TypeId::of::<D>() {
            self.empty = true;
            unsafe { Some(*Box::from_raw(self.data as *mut D)) }
        } else {
            None
        }
    }

    pub fn type_id(&self) -> TypeId {
        self.type_id
    }

    pub fn is(&self, type_id: TypeId) -> bool {
        self.type_id == type_id
    }
}

impl Drop for ValueStore {
    fn drop(&mut self) {
        if self.empty == false {
            unsafe {
                Box::from_raw(self.data);
            }
        }
    }
}
