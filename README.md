# Rea 🔥
> Library that brings simplicity of state management in Vue.js into the world of Rust 🦀

## Features
* Fully statically typed
* Automatic dependency detection and reactivity
* Value properties
* Cached computed properties (*with optional setters*)

## Quirks
* Values need to implement `Clone`
* Not thread-safe

```rust
// from bin/playground.rs

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

/*
Outputs:
    Full name is: World
    Full name is: Hello World
    Full name is: Rea World
    Full name is: World
*/
```

## Goal
Main goal of this little project is to build the foundation required to create a simple-to-use, user-friendly and low-boilerplate reactive GUI library similar to `Vue.js`.