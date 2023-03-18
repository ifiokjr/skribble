pub struct SkribbleRoot(String);

impl SkribbleValue for SkribbleRoot {
  fn from_ref(value: impl AsRef<str>) -> Self {
    Self(value.as_ref().to_string())
  }

  fn get_skribble_value(&self) -> &String {
    &self.0
  }
}
impl MediaQueryDeviceCategories for SkribbleRoot {}
impl MediaQueryBreakpoints for SkribbleRoot {}
impl ParentModifiers for SkribbleRoot {}

pub struct MediaQueryDeviceCategoriesChild(String);

impl SkribbleValue for MediaQueryDeviceCategoriesChild {
  fn from_ref(value: impl AsRef<str>) -> Self {
    Self(value.as_ref().to_string())
  }

  fn get_skribble_value(&self) -> &String {
    &self.0
  }
}

impl MediaQueryBreakpoints for MediaQueryDeviceCategoriesChild {}
impl ParentModifiers for MediaQueryDeviceCategoriesChild {}

pub trait MediaQueryDeviceCategories: SkribbleValue {
  fn screen(&self) -> MediaQueryDeviceCategoriesChild {
    MediaQueryDeviceCategoriesChild::from_ref(self.append_to_skribble_value("screen"))
  }
  fn print(&self) -> MediaQueryDeviceCategoriesChild {
    MediaQueryDeviceCategoriesChild::from_ref(self.append_to_skribble_value("print"))
  }
}

pub struct MediaQueryBreakpointsChild(String);
impl SkribbleValue for MediaQueryBreakpointsChild {
  fn from_ref(value: impl AsRef<str>) -> Self {
    Self(value.as_ref().to_string())
  }

  fn get_skribble_value(&self) -> &String {
    &self.0
  }
}
impl ParentModifiers for MediaQueryBreakpointsChild {}

pub trait MediaQueryBreakpoints: SkribbleValue {
  fn sm(&self) -> MediaQueryBreakpointsChild {
    MediaQueryBreakpointsChild::from_ref(self.append_to_skribble_value("sm"))
  }
  fn md(&self) -> MediaQueryBreakpointsChild {
    MediaQueryBreakpointsChild::from_ref(self.append_to_skribble_value("md"))
  }
  fn lg(&self) -> MediaQueryBreakpointsChild {
    MediaQueryBreakpointsChild::from_ref(self.append_to_skribble_value("lg"))
  }
  fn xl(&self) -> MediaQueryBreakpointsChild {
    MediaQueryBreakpointsChild::from_ref(self.append_to_skribble_value("xl"))
  }
  fn xxl(&self) -> MediaQueryBreakpointsChild {
    MediaQueryBreakpointsChild::from_ref(self.append_to_skribble_value("xxl"))
  }
}

pub struct ParentModifiersChild(String);
impl SkribbleValue for ParentModifiersChild {
  fn from_ref(value: impl AsRef<str>) -> Self {
    Self(value.as_ref().to_string())
  }

  fn get_skribble_value(&self) -> &String {
    &self.0
  }
}
pub trait ParentModifiers: SkribbleValue {
  fn rtl(&self) -> ParentModifiersChild {
    ParentModifiersChild::from_ref(self.append_to_skribble_value("rtl"))
  }

  fn group_hover(&self) -> ParentModifiersChild {
    ParentModifiersChild::from_ref(self.append_to_skribble_value("groupHover"))
  }

  fn group_focus(&self) -> ParentModifiersChild {
    ParentModifiersChild::from_ref(self.append_to_skribble_value("groupFocus"))
  }

  fn group_active(&self) -> ParentModifiersChild {
    ParentModifiersChild::from_ref(self.append_to_skribble_value("groupActive"))
  }

  fn group_visited(&self) -> ParentModifiersChild {
    ParentModifiersChild::from_ref(self.append_to_skribble_value("groupVisited"))
  }

  fn light(&self) -> ParentModifiersChild {
    ParentModifiersChild::from_ref(self.append_to_skribble_value("light"))
  }

  fn dark(&self) -> ParentModifiersChild {
    ParentModifiersChild::from_ref(self.append_to_skribble_value("dark"))
  }
}

pub trait SkribbleValue {
  fn from_ref(value: impl AsRef<str>) -> Self;
  fn get_skribble_value(&self) -> &String;
  fn append_to_skribble_value(&self, value: impl AsRef<str>) -> String {
    format!("{}{}", self.get_skribble_value(), value.as_ref())
  }
}

pub fn sk() -> SkribbleRoot {
  SkribbleRoot::from_ref("")
}

fn main() {
  // let a =
  sk().screen().md().group_hover();
}
