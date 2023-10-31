<div align="center">
  <h1 align="center">lazy-attribute</h1>

  <p>
    <a href="https://crates.io/crates/lazy-attribute">
      <img src="https://img.shields.io/crates/v/lazy-attribute?label=crates" alt="Crate">
    </a>
    <a href="https://codecov.io/gh/zerocore-ai/lazy-attribute">
      <img src="https://codecov.io/gh/zerocore-ai/lazy-attribute/branch/main/graph/badge.svg?token=SOMETOKEN" alt="Code Coverage"/>
    </a>
    <a href="https://github.com/zerocore-ai/lazy-attribute/actions?query=">
      <img src="https://github.com/zerocore-ai/lazy-attribute/actions/workflows/tests_and_checks.yml/badge.svg" alt="Build Status">
    </a>
    <a href="https://github.com/zerocore-ai/lazy-attribute/blob/main/LICENSE-APACHE">
      <img src="https://img.shields.io/badge/License-Apache%202.0-blue.svg" alt="License-Apache">
    </a>
    <a href="https://github.com/zerocore-ai/lazy-attribute/blob/main/LICENSE-MIT">
      <img src="https://img.shields.io/badge/License-MIT-blue.svg" alt="License-MIT">
    </a>
    <a href="https://docs.rs/lazy-attribute">
      <img src="https://img.shields.io/static/v1?label=Docs&message=docs.rs&color=blue" alt="Docs">
    </a>
  </p>
</div>

The `lazy_attribute` crate provides a `lazy` attribute procedural macro that allows you to lazily evaluate a function and cache its result.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
lazy-attribute = "0.1"
```

### Examples

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

#[lazy]
async fn foo() -> i32 {
    println!("Called once!");
    42
}
```

### Crate Features

- `async` - Enables support for lazily evaluating async functions.
