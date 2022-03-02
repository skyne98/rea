use std::any::Any;

use rea::context::Context;

fn main() {
    let mut context = Context::new();
}

#[derive(Debug)]
pub struct TestStructWithString(String);
