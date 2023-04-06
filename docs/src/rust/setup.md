# Setup

Add the default skribble configuration to the `build.rs` file.

```rust,no_run
use skribble::core::Result;
use skribble::create_config;
use skribble::run;

fn main() -> Result<()> {
  let config = create_config();
  run(config)?;
  Ok(())
}
```
