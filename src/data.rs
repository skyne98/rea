use std::{
    any::{Any, TypeId},
    fmt::Debug,
    marker::PhantomData,
};
pub trait Data: Debug + Any + 'static {}

impl<D: Debug + 'static> Data for D {}

#[derive(Debug)]
pub struct DataRef<D: Debug + Any + 'static> {
    pub id: usize,
    pub(crate) phantom: PhantomData<D>,
}

impl<D: Debug + Any + 'static> Clone for DataRef<D> {
    fn clone(&self) -> Self {
        Self {
            id: self.id.clone(),
            phantom: self.phantom.clone(),
        }
    }
}
impl<D: Debug + Any + 'static> Copy for DataRef<D> {}

pub(crate) struct DataStore {
    // Stores type id and mutable pointer to data
    type_id: TypeId,
    data: *mut dyn Data,
}

impl DataStore {
    pub fn new<D: Data>(data: D) -> Self {
        let type_id = TypeId::of::<D>();
        let data = Box::new(data);
        let data_ptr = Box::into_raw(data) as *mut dyn Data;
        DataStore {
            type_id,
            data: data_ptr,
        }
    }

    pub fn get<D: Data>(self) -> Option<D> {
        if self.type_id == TypeId::of::<D>() {
            unsafe { Some(*Box::from_raw(self.data as *mut D)) }
        } else {
            None
        }
    }

    pub fn get_ref<D: Data>(&self) -> Option<&D> {
        if self.type_id == TypeId::of::<D>() {
            unsafe { Some(&*(self.data as *const D)) }
        } else {
            None
        }
    }

    pub fn get_mut<D: Data>(&mut self) -> Option<&mut D> {
        if self.type_id == TypeId::of::<D>() {
            unsafe { Some(&mut *(self.data as *mut D)) }
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
