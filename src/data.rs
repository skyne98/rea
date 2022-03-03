use std::{
    any::{Any, TypeId},
    fmt::Debug,
    marker::PhantomData,
};
pub trait Data: Debug + Any + Clone + 'static {}

impl<D: Debug + Clone + 'static> Data for D {}

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
    data: *mut usize,
    empty: bool,
}

impl DataStore {
    pub fn new<D: Data>(data: D) -> Self {
        let type_id = TypeId::of::<D>();
        let data = Box::new(data);
        println!(
            "[rea] Creating data store for type {:?} with value {:?}",
            type_id, data
        );
        let data_ptr = Box::into_raw(data) as *mut usize;
        DataStore {
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
            println!(
                "[rea] Got a type mismatch of between data's {:?} and {:?}",
                self.type_id,
                TypeId::of::<D>()
            );
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

impl Drop for DataStore {
    fn drop(&mut self) {
        if self.empty == false {
            unsafe {
                Box::from_raw(self.data);
            }
        }
    }
}
