use std::{any::Any, fmt::Debug};

pub trait Data: Debug + Any + Clone + 'static {}

impl<D: Debug + Clone + 'static> Data for D {}
