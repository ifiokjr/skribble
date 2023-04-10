#![deny(clippy::all)]
#![forbid(clippy::indexing_slicing)]

doc_comment::doctest!("../readme.md");

use std::fmt::Display;
use std::str::FromStr;

pub use palette; // Re-export palette
pub use palette::rgb::Rgba;
pub use palette::FromColor;
pub use palette::Hsla;
pub use palette::Hsva;
pub use palette::Hwba;
pub use palette::LabHue;
pub use palette::Laba;
pub use palette::Lcha;
pub use palette::OklabHue;
pub use palette::Oklaba;
pub use palette::Oklcha;
pub use palette::RgbHue;

/// This enum represents a color in any of the supported css color formats.
///
/// The currently supported formats are:
///
/// - `hex` which is a 6 digit hex value with an optional alpha channel
/// - [`rgb`](https://developer.mozilla.org/en-US/docs/Web/CSS/color_value/rgb)
/// - [`hsl`](https://developer.mozilla.org/en-US/docs/Web/CSS/color_value/hsl)
/// - [`hwb`](https://developer.mozilla.org/en-US/docs/Web/CSS/color_value/hwb)
/// - [`lch`](https://developer.mozilla.org/en-US/docs/Web/CSS/color_value/lch)
/// - [`oklch`](https://developer.mozilla.org/en-US/docs/Web/CSS/color_value/oklch)
/// - [`lab`](https://developer.mozilla.org/en-US/docs/Web/CSS/color_value/lab)
/// - [`oklab`](https://bottosson.github.io/posts/oklab/)
#[cfg_attr(
  feature = "serde",
  derive(::serde::Serialize, ::serde::Deserialize),
  serde(tag = "type")
)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Color {
  Hex(Rgba),
  Rgb(Rgba),
  Hsl(Hsla),
  Hwb(Hwba),
  Hsv(Hsva),
  Lch(Lcha),
  Oklch(Oklcha),
  Lab(Laba),
  Oklab(Oklaba),
}

impl Color {
  /// Create a color from the rgb values.
  ///
  /// # Arguments
  ///
  /// - red - The amount of red light, where `0.0` is no red light and `1.0` is
  ///   the highest displayable amount.
  /// - green - The amount of green light, where `0.0` is no green light and
  ///   `1.0` is the highest displayable amount.
  /// - blue - The amount of blue light, where `0.0` is no blue light and `1.0`
  ///   is the highest displayable amount.
  /// - `alpha` - The transparency component. `0.0` is fully transparent and
  ///   `1.0` is fully opaque.
  pub fn rgb(red: f32, green: f32, blue: f32, alpha: f32) -> Self {
    Self::Rgb(Rgba::new(red, green, blue, alpha))
  }

  /// Create a color from the u8 rgb values where the maximum is 255. You can
  /// write the numbers hexadecimally by prefixing them with `0x`.
  ///
  /// # Arguments
  ///
  /// - red - The amount of red light, where 0u8 is no red light and 255u8
  /// is the highest displayable amount.
  /// - green - The amount of green light, where 0u8 is no green light and 255u8
  ///   is the highest displayable amount.
  /// - blue - The amount of blue light, where 0u8 is no blue light and 255u8 is
  ///   the highest displayable amount.
  /// - alpha - The transparency component. 0u8 is fully transparent and 255u8
  ///   is fully
  /// opaque.
  pub fn hex(red: u8, green: u8, blue: u8, alpha: u8) -> Self {
    Self::Hex(Rgba::new(
      red as f32 / 255.0,
      green as f32 / 255.0,
      blue as f32 / 255.0,
      alpha as f32 / 255.0,
    ))
  }

  /// Create a color from the hsl values.
  ///
  /// # Arguments
  ///
  /// - `hue` - The hue of the color, in degrees. Decides if it's red, blue,
  ///   purple, etc.
  /// - `saturation` - The colorfulness of the color. `0.0` gives gray scale
  ///   colors and `1.0` will give absolutely clear colors.
  /// - `lightness` - Decides how light the color will look. `0.0` will be
  ///   black, `0.5` will give a clear color, and `1.0` will give white.
  /// - `alpha` - The transparency component. `0.0` is fully transparent and
  ///   `1.0` is fully opaque.
  pub fn hsl(hue: f32, saturation: f32, lightness: f32, alpha: f32) -> Self {
    Self::Hsl(Hsla::new(hue, saturation, lightness, alpha))
  }

  /// Create a color from the hwb values.
  ///
  /// # Arguments
  ///
  /// - `hue` - The hue of the color, in degrees. Decides if it's red, blue,
  ///   purple, etc. Same as the hue for HSL and HSV.
  /// - `whiteness` - The whiteness of the color. It specifies the amount white
  ///   to mix into the hue. It varies from `0.0` to `1.0`, with `1.0` being
  ///   always full white and `0.0` always being the color shade (a mixture of a
  ///   pure hue with black) chosen with the other two controls.
  /// - `blackness` - The blackness of the color. It specifies the amount black
  ///   to mix into the hue. It varies from 0 to 1, with 1 being always full
  ///   black and 0 always being the color tint (a mixture of a pure hue with
  ///   white) chosen with the other two controls.
  /// - `alpha` - The transparency component. 0.0 is fully transparent and 1.0
  ///   is fully opaque.
  pub fn hwb(hue: f32, whiteness: f32, blackness: f32, alpha: f32) -> Self {
    Self::Hwb(Hwba::new(hue, whiteness, blackness, alpha))
  }

  /// Create a color from the hsv values.
  ///
  /// # Arguments
  ///
  /// - `hue` - The hue of the color, in degrees. Decides if it's red, blue,
  ///   purple, etc.
  /// - saturation - The colorfulness of the color. `0.0` gives gray scale
  ///   colors and `1.0` will give absolutely clear colors.
  /// - `value` - Decides how bright the color will look. `0.0` will be black,
  ///   and `1.0` will give a bright an clear color that goes towards white when
  ///   `saturation` goes towards `0.0`.
  /// - `alpha` - The transparency component. `0.0` is fully transparent and
  ///   `1.0` is fully opaque.
  pub fn hsv(hue: f32, saturation: f32, value: f32, alpha: f32) -> Self {
    Self::Hsv(Hsva::new(hue, saturation, value, alpha))
  }

  /// Create a color from the lab values.
  ///
  /// # Arguments
  ///
  /// - `l`- the lightness of the color. `0.0` gives absolute black and `100.0`
  /// give the brightest white.
  /// - `a` - goes from red at `-128.0` to green at `127.0`.
  /// - `b` - goes from yellow at `-128.0` to blue at `127.0`.
  /// - `alpha` - The transparency component. 0.0 is fully transparent and 1.0
  ///   is fully
  /// opaque.
  pub fn lab(l: f32, a: f32, b: f32, alpha: f32) -> Self {
    Self::Lab(Laba::new(l, a, b, alpha))
  }

  /// Create a color from the oklab values.
  ///
  /// # Arguments
  ///
  /// - `l` - the lightness of the color. `0.0` gives absolute black and `1.0`
  ///   gives the brightest white.
  /// - `a` - goes from red at `-1.0` to green at `1.0`.
  /// - `b` - goes from yellow at `-1.0` to blue at `1.0`.
  pub fn oklab(l: f32, a: f32, b: f32, alpha: f32) -> Self {
    Self::Oklab(Oklaba::new(l, a, b, alpha))
  }

  /// Create a color from the lch values.
  ///
  /// # Arguments
  ///
  /// - `l` is the lightness of the color. 0.0 gives absolute black and 100.0
  ///   gives the brightest white.
  /// - `c` is the colorfulness of the color. It's similar to saturation. 0.0
  /// gives gray scale colors, and numbers around 128-181 gives fully saturated
  /// colors. The upper limit of 128 should include the whole L\*a\*b\* space
  /// and some more.
  /// - `hue` of the color, in degrees. Decides if it's red, blue, purple, etc.
  pub fn lch(l: f32, c: f32, h: f32, alpha: f32) -> Self {
    Self::Lch(Lcha::new(l, c, h, alpha))
  }

  /// Create a color from the oklch values.
  ///
  /// # Arguments
  ///
  /// - `l` is the lightness of the color. `0.0` gives absolute black and `1.0`
  ///   gives the brightest white.
  /// - `C` is the colorfulness of the color, from greyscale at 0 to the most
  ///   colorful at `1.0`.
  /// - `h` is the hue of the color, in degrees. Decides if it's red, blue,
  ///   purple, etc.
  pub fn oklch(l: f32, c: f32, h: f32, alpha: f32) -> Self {
    Self::Oklch(Oklcha::new(l, c, h, alpha))
  }

  /// Returns the color as a HEX value. Will remain unchanged if the color is
  /// already in the HEX format.
  pub fn into_hex(self) -> Self {
    match self {
      Self::Hex(_) => self,
      Self::Rgb(rgba) => Self::Hex(rgba),
      Self::Hsl(hsla) => Self::Hex(Rgba::from_color(hsla)),
      Self::Hwb(hwba) => Self::Hex(Rgba::from_color(hwba)),
      Self::Hsv(hsva) => Self::Hex(Rgba::from_color(hsva)),
      Self::Lch(lcha) => Self::Hex(Rgba::from_color(lcha)),
      Self::Oklch(oklcha) => Self::Hex(Rgba::from_color(oklcha)),
      Self::Lab(laba) => Self::Hex(Rgba::from_color(laba)),
      Self::Oklab(oklaba) => Self::Hex(Rgba::from_color(oklaba)),
    }
  }

  /// Returns the color as an RGB value. Will remain unchanged if the color is
  /// already in the RGB format.
  pub fn into_rgb(self) -> Self {
    match self {
      Self::Hex(rgba) => Self::Rgb(rgba),
      Self::Rgb(_) => self,
      Self::Hsl(hsla) => Self::Rgb(Rgba::from_color(hsla)),
      Self::Hwb(hwba) => Self::Rgb(Rgba::from_color(hwba)),
      Self::Hsv(hsva) => Self::Rgb(Rgba::from_color(hsva)),
      Self::Lch(lcha) => Self::Rgb(Rgba::from_color(lcha)),
      Self::Oklch(oklcha) => Self::Rgb(Rgba::from_color(oklcha)),
      Self::Lab(laba) => Self::Rgb(Rgba::from_color(laba)),
      Self::Oklab(oklaba) => Self::Rgb(Rgba::from_color(oklaba)),
    }
  }

  /// Returns the color as an HSL value. Will remain unchanged if the color is
  /// already in the HSL format.
  pub fn into_hsl(self) -> Self {
    match self {
      Self::Hex(rgb) => Self::Hsl(Hsla::from_color(rgb)),
      Self::Rgb(rgb) => Self::Hsl(Hsla::from_color(rgb)),
      Self::Hsl(_) => self,
      Self::Hwb(hwba) => Self::Hsl(Hsla::from_color(hwba)),
      Self::Hsv(hsva) => Self::Hsl(Hsla::from_color(hsva)),
      Self::Lch(lch) => Self::Hsl(Hsla::from_color(lch)),
      Self::Oklch(oklch) => Self::Hsl(Hsla::from_color(oklch)),
      Self::Lab(lab) => Self::Hsl(Hsla::from_color(lab)),
      Self::Oklab(oklab) => Self::Hsl(Hsla::from_color(oklab)),
    }
  }

  /// Returns the color as an HWB value. Will remain unchanged if the color is
  /// already in the HWB format.
  pub fn into_hwb(self) -> Self {
    match self {
      Self::Hex(rgb) => Self::Hwb(Hwba::from_color(rgb)),
      Self::Rgb(rgb) => Self::Hwb(Hwba::from_color(rgb)),
      Self::Hsl(hsla) => Self::Hwb(Hwba::from_color(hsla)),
      Self::Hwb(_) => self,
      Self::Hsv(hsva) => Self::Hwb(Hwba::from_color(hsva)),
      Self::Lch(lch) => Self::Hwb(Hwba::from_color(lch)),
      Self::Oklch(oklch) => Self::Hwb(Hwba::from_color(oklch)),
      Self::Lab(lab) => Self::Hwb(Hwba::from_color(lab)),
      Self::Oklab(oklab) => Self::Hwb(Hwba::from_color(oklab)),
    }
  }

  /// Returns the color as an HSV value. Will remain unchanged if the color is
  /// already in the HSV format.
  pub fn into_hsv(self) -> Self {
    match self {
      Self::Hex(rgb) => Self::Hsv(Hsva::from_color(rgb)),
      Self::Rgb(rgb) => Self::Hsv(Hsva::from_color(rgb)),
      Self::Hsl(hsla) => Self::Hsv(Hsva::from_color(hsla)),
      Self::Hwb(hwba) => Self::Hsv(Hsva::from_color(hwba)),
      Self::Hsv(_) => self,
      Self::Lch(lch) => Self::Hsv(Hsva::from_color(lch)),
      Self::Oklch(oklch) => Self::Hsv(Hsva::from_color(oklch)),
      Self::Lab(lab) => Self::Hsv(Hsva::from_color(lab)),
      Self::Oklab(oklab) => Self::Hsv(Hsva::from_color(oklab)),
    }
  }

  /// Returns the color as an LCH value. Will remain unchanged if the color is
  /// already in the LCH format.
  pub fn into_lch(self) -> Self {
    match self {
      Self::Hex(rgb) => Self::Lch(Lcha::from_color(rgb)),
      Self::Rgb(rgb) => Self::Lch(Lcha::from_color(rgb)),
      Self::Hsl(hsla) => Self::Lch(Lcha::from_color(hsla)),
      Self::Hwb(hwba) => Self::Lch(Lcha::from_color(hwba)),
      Self::Hsv(hsva) => Self::Lch(Lcha::from_color(hsva)),
      Self::Lch(_) => self,
      Self::Oklch(oklch) => Self::Lch(Lcha::from_color(oklch)),
      Self::Lab(lab) => Self::Lch(Lcha::from_color(lab)),
      Self::Oklab(oklab) => Self::Lch(Lcha::from_color(oklab)),
    }
  }

  /// Returns the color as an OKLCH value. Will remain unchanged if the color is
  /// already in the OKLCH format.
  pub fn into_oklch(self) -> Self {
    match self {
      Self::Hex(rgb) => Self::Oklch(Oklcha::from_color(rgb)),
      Self::Rgb(rgb) => Self::Oklch(Oklcha::from_color(rgb)),
      Self::Hsl(hsla) => Self::Oklch(Oklcha::from_color(hsla)),
      Self::Hwb(hwba) => Self::Oklch(Oklcha::from_color(hwba)),
      Self::Hsv(hsva) => Self::Oklch(Oklcha::from_color(hsva)),
      Self::Lch(lch) => Self::Oklch(Oklcha::from_color(lch)),
      Self::Oklch(_) => self,
      Self::Lab(lab) => Self::Oklch(Oklcha::from_color(lab)),
      Self::Oklab(oklab) => Self::Oklch(Oklcha::from_color(oklab)),
    }
  }

  /// Returns the color as an LAB value. Will remain unchanged if the color is
  /// already in the LAB format.
  pub fn into_lab(self) -> Self {
    match self {
      Self::Hex(rgb) => Self::Lab(Laba::from_color(rgb)),
      Self::Rgb(rgb) => Self::Lab(Laba::from_color(rgb)),
      Self::Hsl(hsla) => Self::Lab(Laba::from_color(hsla)),
      Self::Hwb(hwba) => Self::Lab(Laba::from_color(hwba)),
      Self::Hsv(hsva) => Self::Lab(Laba::from_color(hsva)),
      Self::Lch(lch) => Self::Lab(Laba::from_color(lch)),
      Self::Oklch(oklch) => Self::Lab(Laba::from_color(oklch)),
      Self::Lab(_) => self,
      Self::Oklab(oklab) => Self::Lab(Laba::from_color(oklab)),
    }
  }

  /// Returns the color as an OKLAB value. Will remain unchanged if the color is
  /// already in the OKLAB format.
  pub fn into_oklab(self) -> Self {
    match self {
      Self::Hex(rgb) => Self::Oklab(Oklaba::from_color(rgb)),
      Self::Rgb(rgb) => Self::Oklab(Oklaba::from_color(rgb)),
      Self::Hsl(hsla) => Self::Oklab(Oklaba::from_color(hsla)),
      Self::Hwb(hwba) => Self::Oklab(Oklaba::from_color(hwba)),
      Self::Hsv(hsva) => Self::Oklab(Oklaba::from_color(hsva)),
      Self::Lch(lch) => Self::Oklab(Oklaba::from_color(lch)),
      Self::Oklch(oklch) => Self::Oklab(Oklaba::from_color(oklch)),
      Self::Lab(lab) => Self::Oklab(Oklaba::from_color(lab)),
      Self::Oklab(_) => self,
    }
  }

  /// Get a reference to the inner hex value if the current color is in the hex
  /// format.
  pub fn get_hex(&self) -> Option<&Rgba> {
    match self {
      Self::Hex(ref rgba) => Some(rgba),
      _ => None,
    }
  }

  /// Get a mutable reference to the inner hex value if the current color is in
  /// the hex format.
  pub fn get_hex_mut(&mut self) -> Option<&mut Rgba> {
    match self {
      Self::Hex(ref mut rgba) => Some(rgba),
      _ => None,
    }
  }

  /// Get a reference to the inner rgb value if the current color is in the rgb
  /// format.
  pub fn get_rgb(&self) -> Option<&Rgba> {
    match self {
      Self::Rgb(ref rgba) => Some(rgba),
      _ => None,
    }
  }

  /// Get a mutable reference to the inner rgb value if the current color is in
  /// the rgb format.
  pub fn get_rgb_mut(&mut self) -> Option<&mut Rgba> {
    match self {
      Self::Rgb(ref mut rgba) => Some(rgba),
      _ => None,
    }
  }

  /// Get a reference to the inner hsl value if the current color is in the hsl
  /// format.
  pub fn get_hsl(&self) -> Option<&Hsla> {
    match self {
      Self::Hsl(ref hsla) => Some(hsla),
      _ => None,
    }
  }

  /// Get a mutable reference to the inner hsl value if the current color is in
  /// the hsl format.
  pub fn get_hsl_mut(&mut self) -> Option<&mut Hsla> {
    match self {
      Self::Hsl(ref mut hsla) => Some(hsla),
      _ => None,
    }
  }

  /// Get a reference to the inner hwb value if the current color is in the hwb
  /// format.
  pub fn get_hwb(&self) -> Option<&Hwba> {
    match self {
      Self::Hwb(ref hwba) => Some(hwba),
      _ => None,
    }
  }

  /// Get a mutable reference to the inner hwb value if the current color is in
  /// the hwb format.
  pub fn get_hwb_mut(&mut self) -> Option<&mut Hwba> {
    match self {
      Self::Hwb(ref mut hwba) => Some(hwba),
      _ => None,
    }
  }

  /// Get a reference to the inner lch value if the current color is in the lch
  /// format.
  pub fn get_lch(&self) -> Option<&Lcha> {
    match self {
      Self::Lch(ref lch) => Some(lch),
      _ => None,
    }
  }

  /// Get a mutable reference to the inner lch value if the current color is in
  /// the lch format.
  pub fn get_lch_mut(&mut self) -> Option<&mut Lcha> {
    match self {
      Self::Lch(ref mut lch) => Some(lch),
      _ => None,
    }
  }

  /// Get a reference to the inner oklch value if the current color is in the
  /// oklch format.
  pub fn get_oklch(&self) -> Option<&Oklcha> {
    match self {
      Self::Oklch(ref oklch) => Some(oklch),
      _ => None,
    }
  }

  /// Get a mutable reference to the inner oklch value if the current color is
  /// in the oklch format.
  pub fn get_oklch_mut(&mut self) -> Option<&mut Oklcha> {
    match self {
      Self::Oklch(ref mut oklch) => Some(oklch),
      _ => None,
    }
  }

  /// Get a reference to the inner lab value if the current color is in the lab
  /// format.
  pub fn get_lab(&self) -> Option<&Laba> {
    match self {
      Self::Lab(ref lab) => Some(lab),
      _ => None,
    }
  }

  /// Get a mutable reference to the inner lab value if the current color is in
  /// the lab format.
  pub fn get_lab_mut(&mut self) -> Option<&mut Laba> {
    match self {
      Self::Lab(ref mut lab) => Some(lab),
      _ => None,
    }
  }

  /// Get a reference to the inner oklab value if the current color is in the
  /// oklab format.
  pub fn get_oklab(&self) -> Option<&Oklaba> {
    match self {
      Self::Oklab(ref oklab) => Some(oklab),
      _ => None,
    }
  }

  /// Get a mutable reference to the inner oklab value if the current color is
  /// in the oklab format.
  pub fn get_oklab_mut(&mut self) -> Option<&mut Oklaba> {
    match self {
      Self::Oklab(ref mut oklab) => Some(oklab),
      _ => None,
    }
  }

  /// This is used in the `skribble_core` library to create a string value with
  /// a custom opacity `css_variable`
  pub fn to_string_with_opacity(&self, opacity_variable: impl AsRef<str>) -> String {
    let opacity_variable = opacity_variable.as_ref();

    match self {
      Self::Hex(ref rgba) => hex_to_css(rgba, Some(opacity_variable)),
      Self::Rgb(ref rgba) => rgb_to_css(rgba, Some(opacity_variable)),
      Self::Hsl(ref hsla) => hsl_to_css(hsla, Some(opacity_variable)),
      Self::Hwb(ref hwba) => hwb_to_css(hwba, Some(opacity_variable)),
      Self::Hsv(ref hsva) => hsv_to_css(hsva, Some(opacity_variable)),
      Self::Lch(ref lch) => lch_to_css(lch, Some(opacity_variable)),
      Self::Oklch(ref oklch) => oklch_to_css(oklch, Some(opacity_variable)),
      Self::Lab(ref lab) => lab_to_css(lab, Some(opacity_variable)),
      Self::Oklab(ref oklab) => oklab_to_css(oklab, Some(opacity_variable)),
    }
  }

  /// Get the alpha value of the current color as an [`f32`] value between `0.0`
  /// and `1.0`.
  pub fn alpha(&self) -> f32 {
    match self {
      Self::Hex(ref rgba) => rgba.alpha,
      Self::Rgb(ref rgba) => rgba.alpha,
      Self::Hsl(ref hsla) => hsla.alpha,
      Self::Hwb(ref hwba) => hwba.alpha,
      Self::Hsv(ref hsva) => hsva.alpha,
      Self::Lch(ref lch) => lch.alpha,
      Self::Oklch(ref oklch) => oklch.alpha,
      Self::Lab(ref lab) => lab.alpha,
      Self::Oklab(ref oklab) => oklab.alpha,
    }
  }
}

impl Display for Color {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::Hex(ref rgba) => write!(f, "{}", hex_to_css::<String>(rgba, None)),
      Self::Rgb(ref rgba) => write!(f, "{}", rgb_to_css::<String>(rgba, None)),
      Self::Hsl(ref hsla) => write!(f, "{}", hsl_to_css::<String>(hsla, None)),
      Self::Hwb(ref hwba) => write!(f, "{}", hwb_to_css::<String>(hwba, None)),
      Self::Hsv(ref hsva) => write!(f, "{}", hsv_to_css::<String>(hsva, None)),
      Self::Lch(ref lch) => write!(f, "{}", lch_to_css::<String>(lch, None)),
      Self::Oklch(ref oklch) => write!(f, "{}", oklch_to_css::<String>(oklch, None)),
      Self::Lab(ref lab) => write!(f, "{}", lab_to_css::<String>(lab, None)),
      Self::Oklab(ref oklab) => write!(f, "{}", oklab_to_css::<String>(oklab, None)),
    }
  }
}

impl FromStr for Color {
  type Err = ColorError;

  fn from_str(input: &str) -> Result<Self, Self::Err> {
    parse(input)
  }
}

#[derive(thiserror::Error, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ColorError {
  #[error("invalid hex format")]
  InvalidHex,
  #[error("invalid rgb format")]
  InvalidRgb,
  #[error("invalid hsl format")]
  InvalidHsl,
  #[error("invalid hwb format")]
  InvalidHwb,
  #[error("invalid hsv format")]
  InvalidHsv,
  #[error("invalid lab format")]
  InvalidLab,
  #[error("invalid lch format")]
  InvalidLch,
  #[error("invalid oklab format")]
  InvalidOklab,
  #[error("invalid oklch format")]
  InvalidOklch,
  #[error("invalid color function format")]
  InvalidFunction,
  #[error("invalid unknown format")]
  InvalidUnknown,
}

fn rgb_to_css<T: AsRef<str>>(rgba: &Rgba, opacity: Option<T>) -> String {
  let is_alpha = opacity.is_some() || rgba.alpha != 1.0;
  let red = (rgba.red * 255.0) as u8;
  let green = (rgba.green * 255.0) as u8;
  let blue = (rgba.blue * 255.0) as u8;
  let alpha = opacity
    .map(|v| v.as_ref().to_string())
    .unwrap_or(rgba.alpha.to_string());

  if !is_alpha {
    return format!("rgb({red} {green} {blue})");
  }

  format!("rgb({red} {green} {blue} / {alpha})")
}

fn hex_to_css<T: AsRef<str>>(rgba: &Rgba, opacity: Option<T>) -> String {
  let opacity_is_some = opacity.is_some();
  let is_alpha = opacity_is_some || rgba.alpha != 1.0;
  let red = (rgba.red * 255.0) as u8;
  let green = (rgba.green * 255.0) as u8;
  let blue = (rgba.blue * 255.0) as u8;
  let alpha = opacity
    .map(|v| v.as_ref().to_string())
    .unwrap_or(rgba.alpha.to_string());

  if !is_alpha {
    format!("#{red:x}{green:x}{blue:x}")
  } else if opacity_is_some {
    // No way to specify alpha channel as a variable in hex
    format!("rgb({red} {green} {blue} / {alpha})")
  } else {
    let alpha = (rgba.alpha * 255.0) as u8;
    format!("#{red:x}{green:x}{blue:x}{alpha:x}")
  }
}

fn hsl_to_css<T: AsRef<str>>(hsla: &Hsla, opacity: Option<T>) -> String {
  let is_alpha = opacity.is_some() || hsla.alpha != 1.0;
  let hue = hsla.hue.to_positive_degrees();
  let saturation = hsla.saturation * 100.0;
  let lightness = hsla.lightness * 100.0;
  let alpha = opacity
    .map(|v| v.as_ref().to_string())
    .unwrap_or(hsla.alpha.to_string());

  if !is_alpha {
    return format!("hsl({hue} {saturation}% {lightness}%)");
  }

  format!("hsl({hue} {saturation}% {lightness}% / {alpha})")
}

fn hwb_to_css<T: AsRef<str>>(hwba: &Hwba, opacity: Option<T>) -> String {
  let is_alpha = opacity.is_some() || hwba.alpha != 1.0;
  let hue = hwba.hue.to_positive_degrees();
  let whiteness = hwba.whiteness * 100.0;
  let blackness = hwba.blackness * 100.0;
  let alpha = opacity
    .map(|v| v.as_ref().to_string())
    .unwrap_or(hwba.alpha.to_string());

  if !is_alpha {
    return format!("hwb({hue} {whiteness}% {blackness}%)");
  }

  format!("hwb({hue} {whiteness}% {blackness}% / {alpha})")
}

fn hsv_to_css<T: AsRef<str>>(hsva: &Hsva, opacity: Option<T>) -> String {
  // There is no hsv() function in CSS, so we convert it to hsl()
  let hsla = Hsla::from_color(*hsva);
  hsl_to_css(&hsla, opacity)
}

fn lch_to_css<T: AsRef<str>>(lcha: &Lcha, opacity: Option<T>) -> String {
  let is_alpha = opacity.is_some() || lcha.alpha != 1.0;
  let lightness = lcha.l;
  let chroma = lcha.chroma / 150.0 * 100.0;
  let hue = lcha.hue.to_positive_degrees();
  let alpha = opacity
    .map(|v| v.as_ref().to_string())
    .unwrap_or(lcha.alpha.to_string());

  if !is_alpha {
    return format!("lch({lightness}% {chroma}% {hue})");
  }

  format!("lch({lightness}% {chroma}% {hue} / {alpha})")
}

fn oklch_to_css<T: AsRef<str>>(oklcha: &Oklcha, opacity: Option<T>) -> String {
  let is_alpha = opacity.is_some() || oklcha.alpha != 1.0;
  let lightness = oklcha.l * 100.0;
  let chroma = oklcha.chroma * 100.0;
  let hue = oklcha.hue.to_positive_degrees();
  let alpha = opacity
    .map(|v| v.as_ref().to_string())
    .unwrap_or(oklcha.alpha.to_string());

  if !is_alpha {
    return format!("oklch({lightness}% {chroma}% {hue})");
  }

  format!("oklch({lightness}% {chroma}% {hue} / {alpha})")
}

fn lab_to_css<T: AsRef<str>>(laba: &Laba, opacity: Option<T>) -> String {
  let is_alpha = opacity.is_some() || laba.alpha != 1.0;
  let lightness = laba.l;
  let a = remap(laba.a, LAB_PALETTE_RANGE, LAB_CSS_RANGE);
  let b = remap(laba.b, LAB_PALETTE_RANGE, LAB_CSS_RANGE);
  let alpha = opacity
    .map(|v| v.as_ref().to_string())
    .unwrap_or(laba.alpha.to_string());

  if !is_alpha {
    return format!("lab({lightness}% {a} {b})");
  }

  format!("lab({lightness}% {a} {b} / {alpha})")
}

fn oklab_to_css<T: AsRef<str>>(laba: &Oklaba, opacity: Option<T>) -> String {
  let is_alpha = opacity.is_some() || laba.alpha != 1.0;
  let l = laba.l;
  let a = remap(laba.a, OKLAB_PALETTE_RANGE, OKLAB_CSS_RANGE);
  let b = remap(laba.b, OKLAB_PALETTE_RANGE, OKLAB_CSS_RANGE);
  let alpha = opacity
    .map(|v| v.as_ref().to_string())
    .unwrap_or(laba.alpha.to_string());

  if !is_alpha {
    return format!("oklab({l}% {a} {b})");
  }

  format!("oklab({l}% {a} {b} / {alpha})")
}

fn parse<S: AsRef<str>>(input: S) -> Result<Color, ColorError> {
  let input = input.as_ref().trim().to_lowercase();

  if input == "transparent" {
    return Ok(Color::rgb(0.0, 0.0, 0.0, 0.0));
  }

  // hex format
  if let Some(hex) = input.strip_prefix('#') {
    return parse_hex(hex);
  }

  if let (Some(index), Some(content)) = (input.find('('), input.strip_suffix(')')) {
    let prefix = content
      .get(..index)
      .map(|s| s.trim())
      .ok_or(ColorError::InvalidFunction)?;
    let extracted: ExtractedParams = content.get(index + 1..).unwrap_or("").into();

    match prefix {
      "rgb" | "rgba" => {
        let error = ColorError::InvalidRgb;

        if !extracted.is_valid() {
          return Err(error);
        }

        let red = parse_percent_or_255(extracted.params.first(), error)?;
        let green = parse_percent_or_255(extracted.params.get(1), error)?;
        let blue = parse_percent_or_255(extracted.params.get(2), error)?;

        let alpha = if extracted.is_alpha() {
          parse_percent_or_float(extracted.params.get(3), error)?
        } else {
          (1.0, true)
        };

        if red.1 == green.1 && red.1 == blue.1 {
          return Ok(Color::rgb(red.0, green.0, blue.0, alpha.0));
        }

        return Err(error);
      }
      "hsl" | "hsla" => {
        let error = ColorError::InvalidHsl;

        if !extracted.is_valid() {
          return Err(error);
        }

        let hue = parse_angle(extracted.params.first(), error)?;
        let saturation = parse_percent_or_float(extracted.params.get(1), error)?;
        let lightness = parse_percent_or_float(extracted.params.get(2), error)?;

        let alpha = if extracted.is_alpha() {
          parse_percent_or_float(extracted.params.get(3), error)?
        } else {
          (1.0, true)
        };

        if saturation.1 == lightness.1 {
          return Ok(Color::hsl(hue, saturation.0, lightness.0, alpha.0));
        }

        return Err(error);
      }
      "hwb" | "hwba" => {
        let error = ColorError::InvalidHwb;

        if !extracted.is_valid_slash() {
          return Err(error);
        }

        let hue = parse_angle(extracted.params.first(), error)?;
        let whiteness = parse_percent_or_float(extracted.params.get(1), error)?;
        let blackness = parse_percent_or_float(extracted.params.get(2), error)?;

        let alpha = if extracted.is_alpha() {
          parse_percent_or_float(extracted.params.get(3), error)?
        } else {
          (1.0, true)
        };

        if whiteness.1 == blackness.1 {
          return Ok(Color::hwb(hue, whiteness.0, blackness.0, alpha.0));
        }

        return Err(error);
      }
      "hsv" | "hsva" => {
        // This is a hack to support hsv/hsva which is not a supported css color format.
        let error = ColorError::InvalidHsv;

        if !extracted.is_valid_slash() {
          return Err(error);
        }

        let hue = parse_angle(extracted.params.first(), error)?;
        let saturation = parse_percent_or_float(extracted.params.get(1), error)?;
        let value = parse_percent_or_float(extracted.params.get(2), error)?;

        let alpha = if extracted.is_alpha() {
          parse_percent_or_float(extracted.params.get(3), error)?
        } else {
          (1.0, true)
        };

        if saturation.1 == value.1 {
          return Ok(Color::hsv(hue, saturation.0, value.0, alpha.0));
        }

        return Err(error);
      }
      prefix @ ("lab" | "oklab") => {
        let error = if prefix == "lab" {
          ColorError::InvalidLab
        } else {
          ColorError::InvalidOklab
        };

        if !extracted.is_valid_slash() {
          return Err(error);
        }

        let mut l = parse_percent_or_float(extracted.params.first(), error)?;
        let mut a = parse_percent_or_float(extracted.params.get(1), error)?;
        let mut b = parse_percent_or_float(extracted.params.get(2), error)?;

        let alpha = if extracted.is_alpha() {
          parse_percent_or_float(extracted.params.get(3), error)?
        } else {
          (1.0, true)
        };

        if prefix == "lab" {
          if l.1 {
            l.0 *= 100.0;
          }

          if a.1 {
            a.0 = remap(a.0, OKLAB_PALETTE_RANGE, LAB_PALETTE_RANGE);
          } else {
            a.0 = remap(a.0, LAB_CSS_RANGE, LAB_PALETTE_RANGE);
          }

          if b.1 {
            b.0 = remap(b.0, OKLAB_PALETTE_RANGE, LAB_PALETTE_RANGE);
          } else {
            b.0 = remap(b.0, LAB_CSS_RANGE, LAB_PALETTE_RANGE);
          }

          return Ok(Color::lab(l.0, a.0, b.0, alpha.0));
        }

        if !a.1 {
          a.0 = remap(a.0, OKLAB_CSS_RANGE, OKLAB_PALETTE_RANGE);
        }

        if !a.1 {
          a.0 = remap(a.0, OKLAB_CSS_RANGE, OKLAB_PALETTE_RANGE);
        }

        return Ok(Color::oklab(l.0, a.0, b.0, alpha.0));
      }
      prefix @ ("lch" | "oklch") => {
        let error = if prefix == "lch" {
          ColorError::InvalidLch
        } else {
          ColorError::InvalidOklch
        };

        if !extracted.is_valid_slash() {
          return Err(error);
        }

        let mut l = parse_percent_or_float(extracted.params.first(), error)?;
        let mut chroma = parse_percent_or_float(extracted.params.get(1), error)?;
        let hue = parse_angle(extracted.params.get(2), error)?;

        let alpha = if extracted.is_alpha() {
          parse_percent_or_float(extracted.params.get(3), error)?
        } else {
          (1.0, true)
        };

        if prefix == "lch" {
          if l.1 {
            l.0 *= 100.0;
          }

          if chroma.1 {
            chroma.0 *= 150.0;
          }

          return Ok(Color::lch(l.0, chroma.0, hue, alpha.0));
        }

        if !chroma.1 {
          chroma.0 /= 0.4;
        }

        return Ok(Color::oklch(l.0, chroma.0, hue, alpha.0));
      }
      _ => return Err(ColorError::InvalidFunction),
    }
  }

  if let Ok(color) = parse_hex(input) {
    return Ok(color);
  }

  Err(ColorError::InvalidUnknown)
}

fn parse_percent_or_float<S: AsRef<str>>(
  input: Option<S>,
  error: ColorError,
) -> Result<(f32, bool), ColorError> {
  let input = input.ok_or(error)?;
  let input = input.as_ref();
  input
    .strip_suffix('%')
    .and_then(|s| s.parse().ok().map(|percent: f32| (percent / 100.0, true)))
    .or_else(|| input.parse().ok().map(|t| (t, false)))
    .ok_or(error)
}

fn parse_percent_or_255<S: AsRef<str>>(
  input: Option<S>,
  error: ColorError,
) -> Result<(f32, bool), ColorError> {
  let input = input.ok_or(error)?;
  let input = input.as_ref();
  input
    .strip_suffix('%')
    .and_then(|s| s.parse().ok().map(|t: f32| (t / 100.0, true)))
    .or_else(|| input.parse().ok().map(|t: f32| (t / 255.0, false)))
    .ok_or(error)
}

fn parse_angle<S: AsRef<str>>(input: Option<S>, error: ColorError) -> Result<f32, ColorError> {
  let input = input.ok_or(error)?;
  let input = input.as_ref();
  input
    .strip_suffix("deg")
    .and_then(|s| s.parse().ok())
    .or_else(|| {
      input
        .strip_suffix("grad")
        .and_then(|s| s.parse().ok())
        .map(|t: f32| t * 360.0 / 400.0)
    })
    .or_else(|| {
      input
        .strip_suffix("rad")
        .and_then(|s| s.parse().ok())
        .map(|t: f32| t.to_degrees())
    })
    .or_else(|| {
      input
        .strip_suffix("turn")
        .and_then(|s| s.parse().ok())
        .map(|t: f32| t * 360.0)
    })
    .or_else(|| input.parse().ok())
    .ok_or(error)
}

fn parse_hex<S: AsRef<str>>(input: S) -> Result<Color, ColorError> {
  let input = input.as_ref();

  if !input.is_ascii() {
    return Err(ColorError::InvalidHex);
  }

  let n = input.len();

  if n == 3 || n == 4 {
    let r = u8::from_str_radix(
      input
        .get(0..1)
        .ok_or(ColorError::InvalidHex)?
        .repeat(2)
        .as_str(),
      16,
    )
    .map_err(|_| ColorError::InvalidHex)?;
    let g = u8::from_str_radix(
      input
        .get(1..2)
        .ok_or(ColorError::InvalidHex)?
        .repeat(2)
        .as_str(),
      16,
    )
    .map_err(|_| ColorError::InvalidHex)?;
    let b = u8::from_str_radix(
      input
        .get(2..3)
        .ok_or(ColorError::InvalidHex)?
        .repeat(2)
        .as_str(),
      16,
    )
    .map_err(|_| ColorError::InvalidHex)?;

    let a = if n == 4 {
      u8::from_str_radix(
        input
          .get(3..4)
          .ok_or(ColorError::InvalidHex)?
          .repeat(2)
          .as_str(),
        16,
      )
      .map_err(|_| ColorError::InvalidHex)?
    } else {
      255
    };

    Ok(Color::hex(r, g, b, a))
  } else if n == 6 || n == 8 {
    let r = u8::from_str_radix(input.get(0..2).ok_or(ColorError::InvalidHex)?, 16)
      .map_err(|_| ColorError::InvalidHex)?;
    let g = u8::from_str_radix(input.get(2..4).ok_or(ColorError::InvalidHex)?, 16)
      .map_err(|_| ColorError::InvalidHex)?;
    let b = u8::from_str_radix(input.get(4..6).ok_or(ColorError::InvalidHex)?, 16)
      .map_err(|_| ColorError::InvalidHex)?;

    let a = if n == 8 {
      u8::from_str_radix(input.get(6..8).ok_or(ColorError::InvalidHex)?, 16)
        .map_err(|_| ColorError::InvalidHex)?
    } else {
      255
    };

    Ok(Color::hex(r, g, b, a))
  } else {
    Err(ColorError::InvalidHex)
  }
}

/// Map a value from one range tuple `initial` the replacement range tuple
/// `new`.
fn remap(value: f32, initial: (f32, f32), new: (f32, f32)) -> f32 {
  (value - initial.0) * ((new.1 - new.0) / (initial.1 - initial.0)) + new.0
}

const OKLAB_CSS_RANGE: (f32, f32) = (-0.4, 0.4);
const OKLAB_PALETTE_RANGE: (f32, f32) = (-1.0, 1.0);
const LAB_CSS_RANGE: (f32, f32) = (-125.0, 125.0);
const LAB_PALETTE_RANGE: (f32, f32) = (-128.0, 127.0);

pub struct ExtractedParams {
  params: Vec<String>,
  commas: u8,
  slashes: u8,
  invalid_commas: bool,
  invalid_slashes: bool,
}

impl<S: AsRef<str>> From<S> for ExtractedParams {
  fn from(value: S) -> Self {
    let mut commas: u8 = 0;
    let mut slashes: u8 = 0;
    let mut invalid_commas = false;
    let mut invalid_slashes = false;
    let mut params = vec![];
    let mut current_param = String::new();
    let mut separator = None;

    for ch in value.as_ref().chars() {
      let prev_separator = separator;

      if !ch.is_whitespace() && ![',', '/'].contains(&ch) {
        separator = None;
        current_param.push(ch);

        continue;
      }

      if !current_param.is_empty() {
        params.push(current_param);
        current_param = String::new();
      }

      if ch == '/' {
        slashes += 1;
        separator = Some(ch);

        if
        // a slash can only appear after there are three params
        params.len() != 3
      // a maximum of one slash is allowed
      || slashes > 1
      // a slash can only follow a non separator character (ignoring whitespace)
      || prev_separator.is_some()
        {
          invalid_slashes = true;
        }

        continue;
      }

      if ch == ',' {
        commas += 1;
        separator = Some(ch);

        if prev_separator.is_some() {
          invalid_commas = true;
        }

        continue;
      }
    }

    if !current_param.is_empty() {
      params.push(current_param);
    }

    if commas > 0 && (params.len() - 1) as u8 != commas {
      invalid_commas = true;
    }

    if slashes == 1 && params.len() != 4 {
      invalid_slashes = true;
    }

    Self {
      params,
      commas,
      slashes,
      invalid_commas,
      invalid_slashes,
    }
  }
}

impl ExtractedParams {
  pub fn is_valid(&self) -> bool {
    let length = self.params.len();

    !(self.is_comma_separated() && self.invalid_commas
      || self.is_slash_separated() && self.invalid_slashes
      || self.is_comma_separated() && self.is_slash_separated())
      && (length == 3 || length == 4)
  }

  pub fn is_comma_separated(&self) -> bool {
    self.commas > 0
  }

  pub fn is_slash_separated(&self) -> bool {
    self.slashes > 0
  }

  pub fn is_valid_slash(&self) -> bool {
    !self.is_comma_separated() && self.is_valid()
  }

  pub fn is_valid_comma(&self) -> bool {
    !self.is_slash_separated() && self.is_valid()
  }

  pub fn is_alpha(&self) -> bool {
    self.params.len() == 4
  }
}

#[cfg(test)]
mod __test;
