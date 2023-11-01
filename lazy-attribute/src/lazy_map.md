There are times when you want to lazily evaluate a function, but you want to map the result of the function to a different type.

For example, you may want to lazily evaluate a function that returns a `Result<String, std::io::Error>`, but you want to map the result to a `String`.

`lazy_map` lets you provide a closure that will be used to map the result of the function to a different type.

## Example

```rust
use lazy_attribute::lazy_map;

#[lazy_map(String, |result| result.unwrap_or_default())]
fn get_string() -> Result<String, std::io::Error> {
    println!("Called once!");
    Ok(String::from("Hello, world!"))
}

fn main() {
    println!("{}", get_string());  // Outputs: Called once! Hello, world!
    println!("{}", get_string());  // Outputs: Hello, world!
}
```

With `async` feature enabled, `lazy_map` can also be used with async functions:

```rust
use lazy_attribute::lazy_map;

#[lazy_map(String, |result| result.unwrap_or_default())]
async fn get_string() -> Result<String, std::io::Error> {
    println!("Called once!");
    Ok(String::from("Hello, world!"))
}

#[tokio::main]
async fn main() {
    println!("{}", get_string().await);  // Outputs: Called once! Hello, world!
    println!("{}", get_string().await);  // Outputs: Hello, world!
}
```
