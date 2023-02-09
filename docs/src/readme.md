# Introduction

This is the skribble book.

## skribble: rust

Using `skribble` in rust is composed of four parts:

1. First configure skribble in the `build.rs` file.

```rust
use skribble::config::load;
use skribble::config::Config;
use skribble::config::StyleRules;

fn main() {
  let mut style_rules = StyleRules::default();
  style_rules.add("p", vec!["padding"]);

  let config = Config::builder().style_rules().build();

  // This pulls in configuration from any dependencies so that projects can be
  // composed together. Any conflicts are highlighted during the build process
  // so that any component libraries can be used together with `skribble`.
  load(config);
}
```

Alternatively, you can use the default configuration.

```rust
use skribble::config::load;
use skribble::config::Config;

fn main() {
  load(Config::default());
}
```

2. The create a `skribble.rs` file in your project root and add the following.

```rust
// skribble.rs
pub use skribble::client::*;
```

In your `lib.rs` (or `main.rs`) add the following.

```rust
// lib.rs
mod skribble;
pub use skribble::*;
```

3. Import the class name creator.

```rust
use leptos::*;

use crate::skribble::c;

#[component]
pub fn MyComponent() -> Html {
  html! {
    <div class={c.md().PX()}>
      {"Hello World"}
    </div>
  }
}
```

4. Run the skribble build script to generate the css.

```bash
skribble build
```

Library authors should use following to build the css and include a json file in the root of the
crate.

```bash
skribble build --lib
```
