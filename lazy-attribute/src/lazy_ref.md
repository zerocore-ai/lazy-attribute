Functions decorated with `#[lazy_ref]` will only be executed the first time they are called.
On subsequent calls, the cached return value is immediately returned.

This is useful for avoiding repeating expensive computations or operations.

The first call to a `lazy_ref` function creates initializes a shared variable with a `'static` lifetime, allowing it to live for the entire duration of the program.

## Example

```rust
use lazy_attribute::lazy_ref;

#[lazy_ref]
fn get_string() -> String {
    println!("Called once!");
    String::from("Hello, world!")
}

fn main() {
    println!("{}", get_string());  // Outputs: Called once! Hello, world!
    println!("{}", get_string());  // Outputs: Hello, world!
}
```

With `async` feature enabled, `lazy_ref` can also be used with async functions:

```rust
use lazy_attribute::lazy_ref;

#[lazy_ref]
async fn get_string() -> String {
    println!("Called once!");
    String::from("Hello, world!")
}

#[tokio::main]
async fn main() {
    println!("{}", get_string().await);  // Outputs: Called once! Hello, world!
    println!("{}", get_string().await);  // Outputs: Hello, world!
}
```
