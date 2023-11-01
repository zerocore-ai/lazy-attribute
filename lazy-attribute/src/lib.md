`lazy-attributes` provides attribute macros for simplifying working with lazily evaluated functions.

Functions decorated with `#[lazy_ref]` or `#[lazy_map]` will only be executed the first time they are called.
On subsequent calls, the cached return value is returned.

## Usage

### lazy_ref

With [`lazy_attribute::lazy_ref`][crate::lazy_ref], you can annotate a function that you want to lazily evaluate:

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

The first time the function is called, it will be evaluated and its result will be cached. Subsequent calls will return
the cached result.

`lazy_ref` macro roughly desugars the `get_string` function to:

```ignore
static __lazy_static_get_string: OnceCell<String> = OnceCell::new();

fn get_string() -> &'static String {
    __lazy_static_get_string.get_or_init(|| {
        println!("Called once!");
        String::from("Hello, world!")
    })
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

### lazy_map

There are times when you want to lazily evaluate a function, but you want to map the result of the function to a different type.

For example, you may want to lazily evaluate a function that returns a `Result<String, std::io::Error>`, but you want to map the result to a `String`.

[`lazy_attribute::lazy_map`][crate::lazy_map] lets you provide a closure that will be used to map the result of the function to a different type.

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

Just like `lazy_ref`, `lazy_map` can also be used with async functions when `async` feature is enabled

## Caveats

- `lazy_*` macros do not support functions with arguments. You will get an error telling you arguments are not supported.
- `lazy_map` only takes a closure or function identifier as attribute argument. It does not support arbitrary expressions.

## Crate Features

- `async` - Enables support for lazily evaluating async functions.
