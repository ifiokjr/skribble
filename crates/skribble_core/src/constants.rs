pub const INDENTATION: u8 = 2;
pub const ROOT_SELECTOR: &str = ":root";

pub struct Placeholder;

impl Placeholder {
  pub const CSS_VARIABLE: &str = "CSS_VARIABLE";
  pub const MODIFIER: &str = "MODIFIER";
  pub const PALETTE: &str = "PALETTE";
  pub const PARENT_MODIFIER: &str = "PARENT_MODIFIER";

  pub fn create(namespace: &str, name: impl AsRef<str>) -> String {
    let name = name.as_ref();
    format!("__{namespace}::{name}__")
  }

  /// Generate a placeholder for the variable by using the name. This inserts
  /// some text which will be replaced by the actual variable name when the code
  /// is generated.
  pub fn variable(name: impl AsRef<str>) -> String {
    Self::create(Self::CSS_VARIABLE, name)
  }

  /// Generate a placeholder for the palette color by using the name. This
  /// inserts some text which will be replaced by the actual palette color
  /// when the code is generated.
  pub fn palette(name: impl AsRef<str>) -> String {
    Self::create(Self::PALETTE, name)
  }

  pub fn parent_modifier(name: impl AsRef<str>) -> String {
    Self::create(Self::PARENT_MODIFIER, name)
  }

  pub fn modifier(name: impl AsRef<str>) -> String {
    Self::create(Self::MODIFIER, name)
  }
}
