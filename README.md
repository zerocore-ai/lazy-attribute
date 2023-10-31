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

## Outline

- [Usage](#usage)
- [Contributing](#contributing)
- [Getting Help](#getting-help)
- [External Resources](#external-resources)
- [License](#license)

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

## Contributing

:balloon: We're thankful for any feedback and help in improving our project!
We have a [contributing guide](./CONTRIBUTING.md) to help you get involved. We
also adhere to our [Code of Conduct](./CODE_OF_CONDUCT.md).

### Testing the Project

- Run tests

  ```console
  cargo test --all-features
  ```

### Formatting

For formatting Rust in particular, please use `cargo +nightly fmt` as it uses
specific nightly features we recommend. **Make sure you have nightly
installed**.

### Pre-commit Hook

This project recommends using [pre-commit][pre-commit] for running pre-commit
hooks. Please run this before every commit and/or push.

- Once installed, Run `pre-commit install` and `pre-commit install --hook-type commit-msg`
  to setup the pre-commit hooks locally. This will reduce failed CI builds.

- If you are doing interim commits locally, and for some reason if you _don't_
  want pre-commit hooks to fire, you can run
  `git commit -a -m "Your message here" --no-verify`.

### Recommended Development Flow

- We recommend installing and leveraging [cargo-watch][cargo-watch],
  [cargo-expand][cargo-expand] and [irust][irust] for Rust development.

### Conventional Commits

This project _lightly_ follows the [Conventional Commits
convention][commit-spec-site] to help explain
commit history and tie in with our release process. The full specification
can be found [here][commit-spec]. We recommend prefixing your commits with
a type of `fix`, `feat`, `docs`, `ci`, `refactor`, etc..., structured like so:

```
<type>[optional scope]: <description>

[optional body]

[optional footer(s)]
```

## Getting Help

For usage questions, usecases, or issues please open an issue in our repository.

We would be happy to try to answer your question or try opening a new issue on Github.

## External Resources

These are references to specifications, talks and presentations, etc.

## License

This project is licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](./LICENSE-APACHE) or [http://www.apache.org/licenses/LICENSE-2.0][apache])
- MIT license ([LICENSE-MIT](./LICENSE-MIT) or [http://opensource.org/licenses/MIT][mit])

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.

[apache]: https://www.apache.org/licenses/LICENSE-2.0
[cargo-expand]: https://github.com/dtolnay/cargo-expand
[cargo-udeps]: https://github.com/est31/cargo-udeps
[cargo-watch]: https://github.com/watchexec/cargo-watch
[commit-spec]: https://www.conventionalcommits.org/en/v1.0.0/#specification
[commit-spec-site]: https://www.conventionalcommits.org/
[irust]: https://github.com/sigmaSd/IRust
[mit]: http://opensource.org/licenses/MIT
[pre-commit]: https://pre-commit.com/
