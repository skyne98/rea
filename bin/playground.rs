use rea::{context::Context, context_ref::ContextRef};

fn main() {
    let mut context = ContextRef::new();
    let skip_first_name = context.data(true);
    let first_name = context.data("Hello".to_string());
    let last_name = context.data("World".to_string());
    let full_name = context.computed(move |mut context: ContextRef| {
        if context.get(skip_first_name).unwrap() {
            context.get(last_name).unwrap()
        } else {
            let first_name = context.get(first_name).unwrap();
            let last_name = context.get(last_name).unwrap();
            format!("{} {}", first_name, last_name)
        }
    });
    println!("Full name is: {}", context.get(full_name).unwrap());
    context.set(skip_first_name, false);
    println!("Full name is: {}", context.get(full_name).unwrap());
    context.set(first_name, "Rea".to_string());
    println!("Full name is: {}", context.get(full_name).unwrap());
    context.set(skip_first_name, true);
    println!("Full name is: {}", context.get(full_name).unwrap());
}
