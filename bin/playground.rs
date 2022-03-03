use rea::{context::Context, context_ref::ContextRef};

fn main() {
    let mut context = ContextRef::new();
    let counter = context.data(0);
    let old_counter = context.set(counter, 1);
    println!(
        "Updated counter from {} to {}",
        old_counter,
        context.get(counter).unwrap()
    );
    let old_counter = context.set(counter, 2);
    println!(
        "Updated counter from {} to {}",
        old_counter,
        context.get(counter).unwrap()
    );

    let counter_double =
        context.computed(move |context: ContextRef| context.get(counter).unwrap() * 2);
    println!("Computed created");
    println!("Counter double is {}", context.get(counter_double).unwrap());
}
