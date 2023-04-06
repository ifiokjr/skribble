# skribble_color

> Parse supported css strings into the supported color types.

<br />

[![Crate][crate-image]][crate-link] [![Docs][docs-image]][docs-link] [![Status][ci-status-image]][ci-status-link] [![Unlicense][unlicense-image]][unlicense-link]

## Usage

Add the following to your `Cargo.toml`:

```toml
[dependencies]
skribble_color = "0.0.0"
```

### Color Conversions

All the color types can be converted to each other, as shown below. Each color type has a builtin alpha channel.

```rust
use skribble_color::Color;

let hex: Color = "#b1ffb0".parse().unwrap();
let rgb: Color = hex.into_rgb();
let hsl: Color = hex.into_hsl();
let hwb: Color = hex.into_hwb();
let hsv: Color = hex.into_hsv();
let lch: Color = hex.into_lch();
let lab: Color = hex.into_lab();
let oklch: Color = hex.into_oklch();
let oklab: Color = hex.into_oklab();
```

### Examples

The following will parse a css string into a `Color` type automatically detecting the type of color.

#### Hex

```rust
use skribble_color::Color;
use skribble_color::Rgba;

let hex: Color = "#b1ffb0".parse().unwrap();
assert_eq!(hex.to_string(), "#b1ffb0");
assert_eq!(hex, Color::hex(0xb1, 0xff, 0xb0, 0xff));
```

#### Rgb

```rust
use skribble_color::Color;
use skribble_color::Rgba;

let rgb: Color = "rgb(255, 0, 0)".parse().unwrap();
assert_eq!(rgb.to_string(), "rgb(255 0 0)");
assert_eq!(rgb, Color::rgb(1.0, 0.0, 0.0, 1.0));
```

#### Hsl

```rust
use skribble_color::Color;

let hsl: Color = "hsl(120, 100%, 50%)".parse().unwrap();
assert_eq!(hsl.to_string(), "hsl(120 100% 50%)");
assert_eq!(hsl, Color::hsl(120.0, 1.0, 0.5, 1.0));
```

#### Hwb

```rust
use skribble_color::Color;
use skribble_color::Hwba;

let hwb: Color = "hwb(120 0% 0%)".parse().unwrap();
assert_eq!(hwb.to_string(), "hwb(120 0% 0%)");
assert_eq!(hwb, Color::hwb(120.0, 0.0, 0.0, 1.0));
```

[crate-image]: https://img.shields.io/crates/v/skribble_color.svg
[crate-link]: https://crates.io/crates/skribble_color
[docs-image]: https://docs.rs/skribble_color/badge.svg
[docs-link]: https://docs.rs/skribble_color
[ci-status-image]: https://github.com/ifiokjr/skribble/workflows/ci/badge.svg
[ci-status-link]: https://github.com/ifiokjr/skribble/actions?query=workflow:ci
[unlicense-image]: https://img.shields.io/badge/license-Unlicence-blue.svg
[unlicense-link]: https://opensource.org/license/unlicense
