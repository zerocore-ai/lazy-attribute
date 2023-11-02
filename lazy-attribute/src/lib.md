`lazy-attributes` provides attribute macros for simplifying working with lazily evaluated functions.

Functions decorated with `#[lazy_ref]`` will only be executed the first time they are called.
On subsequent calls, the cached return value is returned.

## Usage

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

## Caveats

- `lazy_*` macros do not support functions with arguments. You will get an error telling you arguments are not supported.

## Crate Features

- `async` - Enables support for lazily evaluating async functions.
