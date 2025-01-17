<p align="center">
  <img src="https://raw.githubusercontent.com/viz-rs/viz.rs/main/static/logo.svg" height="200" />
</p>

<h1 align="center">
  <a href="https://docs.rs/viz">Viz</a>
</h1>

<div align="center">
  <p><strong>Macros for Viz</strong></p>
</div>

<div align="center">
  <!-- Safety -->
  <a href="/">
    <img src="https://img.shields.io/badge/-safety!-success?style=flat-square"
      alt="Safety!" /></a>
  <!-- Docs.rs docs -->
  <a href="https://docs.rs/viz-macros">
    <img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square"
      alt="Docs.rs docs" /></a>
  <!-- Crates version -->
  <a href="https://crates.io/crates/viz-macros">
    <img src="https://img.shields.io/crates/v/viz-macros.svg?style=flat-square"
    alt="Crates.io version" /></a>
  <!-- Downloads -->
  <a href="https://crates.io/crates/viz-macros">
    <img src="https://img.shields.io/crates/d/viz-macros.svg?style=flat-square"
      alt="Download" /></a>
</div>

## Macros

Macro                | Description
-------------------- | ------------
**handler**          | Extended Handler with Extractors

## Example

```rust
use viz::{IntoResponse, Result, types::{Params}};
use viz_macros::handler;

#[handler]
async fn index() -> impl IntoResponse {
    ()
}

#[handler]
async fn get_user(Params(name): Params<String>) -> Result<impl IntoResponse> {
    Ok(name)
}
```
## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or
[MIT license](LICENSE-MIT) at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted 
for inclusion in Viz by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
