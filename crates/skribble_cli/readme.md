# skribble

> a fully typed atomic css compiler for rust, node, deno and the browser

<br />

[![Crate][crate-image]][crate-link] [![Docs][docs-image]][docs-link] [![Status][ci-status-image]][ci-status-link] [![Unlicense][unlicense-image]][unlicense-link]

<br />

## Installation

```toml
[dependencies]
skribble = "0.0.0"
```

### Usage

Add the default skribble configuration to the `build.rs` file.

```rust,no_run
use skribble::Result;
use skribble::create_config;
use skribble::run_with_config;

fn main() -> Result<()> {
  let config = create_config();
  run_with_config(config)?;
  Ok(())
}
```

[crate-image]: https://img.shields.io/crates/v/skribble.svg
[crate-link]: https://crates.io/crates/skribble
[docs-image]: https://docs.rs/skribble/badge.svg
[docs-link]: https://docs.rs/skribble/
[ci-status-image]: https://github.com/ifiokjr/skribble/workflows/ci/badge.svg
[ci-status-link]: https://github.com/ifiokjr/skribble/actions?query=workflow:ci
[unlicense-image]: https://img.shields.io/badge/license-Unlicence-blue.svg
[unlicense-link]: https://opensource.org/license/unlicense
