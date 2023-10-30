The `lazy-attribute` crate provides a `lazy` attribute procedural macro that allows you to lazily evaluate a function and cache its result.

## Examples

Lazily evaluate a function:

```rust
use lazy_attribute::lazy;

fn main() {
    println!("{}", foo());  // Outputs: Called once! 42
    println!("{}", foo());  // Outputs: 42
}

#[lazy]
fn foo() -> i32 {
    println!("Called once!");
    42
}
```

Lazily evaluate an async function (requires `async` feature):

```rust
use lazy_attribute::lazy;

#[tokio::main]
async fn main() {
    println!("{}", foo().await);  // Outputs: Called once! 42
    println!("{}", foo().await);  // Outputs: 42
}

#[lazy(async)]
async fn foo() -> i32 {
    println!("Called once!");
    42
}
```

## Crate Features

- `async` - Enables support for lazily evaluating async functions.
