use rea::context_ref::ContextRef;

fn main() {
    let context = ContextRef::new();
    let skip_first_name = context.value(true);
    let first_name = context.value("Hello".to_string());
    let last_name = context.value("World".to_string());
    let full_name = context.computed(move |context: ContextRef| {
        if skip_first_name.get(&context).unwrap() {
            last_name.get(&context).unwrap()
        } else {
            let first_name = first_name.get(&context).unwrap();
            let last_name = last_name.get(&context).unwrap();
            format!("{} {}", first_name, last_name)
        }
    });
    println!("Full name is: {}", full_name.get(&context).unwrap());
    skip_first_name.set(&context, false);
    println!("Full name is: {}", full_name.get(&context).unwrap());
    first_name.set(&context, "Rea".to_string());
    println!("Full name is: {}", full_name.get(&context).unwrap());
    skip_first_name.set(&context, true);
    println!("Full name is: {}", full_name.get(&context).unwrap());
}
