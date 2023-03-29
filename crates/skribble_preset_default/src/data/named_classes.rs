use indexmap::indexmap;
use lazy_static::lazy_static;
use skribble_core::NamedClass;
use skribble_core::Placeholder;
use skribble_core::StringMap;

lazy_static! {
  pub(crate) static ref NAMED_CLASSES: Vec<NamedClass> = {
    let group_nested_filter = Placeholder::variable("groupNestedFilter");
    let group_nested_backdrop = Placeholder::variable("groupNestedBackdrop");
    let group_nested_transform_gpu = Placeholder::variable("groupNestedTransformGpu");
    let group_nested_transform = Placeholder::variable("groupNestedTransform");
    let group_nested_transform_cpu = Placeholder::variable("groupNestedTransformCpu");
    let contained_max_width = Placeholder::wrapped_variable("containedMaxWidth", None);

    vec![
      NamedClass::builder()
        .name("group")
        .styles(StringMap::default())
        .build(),
      NamedClass::builder()
        .name("contained")
        .styles(indexmap! {
         "width" => "100%".into(),
         "max-width" => contained_max_width,
        })
        .build(),
      NamedClass::builder()
        .name("italic")
        .styles(indexmap! { "font-style" => "italic" })
        .build(),
      NamedClass::builder()
        .name("nonItalic")
        .styles(indexmap! { "font-style" => "normal" })
        .build(),
      NamedClass::builder()
        .name("oblique")
        .styles(indexmap! { "font-style" => "oblique -10deg" })
        .build(),
      NamedClass::builder()
        .name("antialiased")
        .styles(indexmap! {
          "-webkit-font-smoothing" => "antialiased",
          "-moz-osx-font-smoothing" => "grayscale",
        })
        .build(),
      NamedClass::builder()
        .name("subpixelAntialiased")
        .styles(indexmap! {
          "-webkit-font-smoothing" => "auto",
          "-moz-osx-font-smoothing" => "auto",
        })
        .build(),
      NamedClass::builder()
        .name("block")
        .styles(indexmap! { "display" => "block" })
        .build(),
      NamedClass::builder()
        .name("inlineBlock")
        .styles(indexmap! { "display" => "inline-block" })
        .build(),
      NamedClass::builder()
        .name("inline")
        .styles(indexmap! { "display" => "inline" })
        .build(),
      NamedClass::builder()
        .name("flowRoot")
        .styles(indexmap! { "display" => "flow-root" })
        .build(),
      NamedClass::builder()
        .name("content")
        .styles(indexmap! { "display" => "content" })
        .build(),
      NamedClass::builder()
        .name("hidden")
        .styles(indexmap! { "display" => "hidden" })
        .build(),
      NamedClass::builder()
        .name("invisible")
        .styles(indexmap! { "visibility" => "hidden" })
        .build(),
      NamedClass::builder()
        .name("visible")
        .styles(indexmap! { "visibility" => "visible" })
        .build(),
      NamedClass::builder()
        .name("removeFilter")
        .styles(indexmap! { group_nested_filter => "none" })
        .build(),
      NamedClass::builder()
        .name("removeBackdrop")
        .styles(indexmap! { group_nested_backdrop => "none" })
        .build(),
      NamedClass::builder()
        .name("transformGpu")
        .styles(indexmap! { &group_nested_transform => group_nested_transform_gpu })
        .build(),
      NamedClass::builder()
        .name("transformCpu")
        .styles(indexmap! { &group_nested_transform => group_nested_transform_cpu })
        .build(),
    ]
  };
}
