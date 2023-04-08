# Setup

Add the default skribble configuration to the `build.rs` file.

```rust,no_run
use skribble::core::Result;
use skribble::create_config;
use skribble::run_with_config;

fn main() -> Result<()> {
  let config = create_config();
  run_with_config(config)?;
  Ok(())
}
```
