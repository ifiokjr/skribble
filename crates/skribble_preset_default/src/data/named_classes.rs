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

    vec![
      NamedClass::builder()
        .name("group")
        .description("")
        .styles(StringMap::default())
        .build(),
      NamedClass::builder()
        .name("container")
        .description("")
        .styles(indexmap! {
         "width" => "100%",
         "max-width" => "var({container_max_width})",
        })
        .build(),
      NamedClass::builder()
        .name("italic")
        .description("")
        .styles(indexmap! { "font-style" => "italic" })
        .build(),
      NamedClass::builder()
        .name("nonItalic")
        .description("")
        .styles(indexmap! { "font-style" => "normal" })
        .build(),
      NamedClass::builder()
        .name("oblique")
        .description("")
        .styles(indexmap! { "font-style" => "oblique -10deg" })
        .build(),
      NamedClass::builder()
        .name("antialiased")
        .description("")
        .styles(indexmap! {
          "-webkit-font-smoothing" => "antialiased",
          "-moz-osx-font-smoothing" => "grayscale",
        })
        .build(),
      NamedClass::builder()
        .name("subpixelAntialiased")
        .description("")
        .styles(indexmap! {
          "-webkit-font-smoothing" => "auto",
          "-moz-osx-font-smoothing" => "auto",
        })
        .build(),
      NamedClass::builder()
        .name("block")
        .description("")
        .styles(indexmap! { "display" => "block" })
        .build(),
      NamedClass::builder()
        .name("inlineBlock")
        .description("")
        .styles(indexmap! { "display" => "inline-block" })
        .build(),
      NamedClass::builder()
        .name("inline")
        .description("")
        .styles(indexmap! { "display" => "inline" })
        .build(),
      NamedClass::builder()
        .name("flowRoot")
        .description("")
        .styles(indexmap! { "display" => "flow-root" })
        .build(),
      NamedClass::builder()
        .name("content")
        .description("")
        .styles(indexmap! { "display" => "content" })
        .build(),
      NamedClass::builder()
        .name("hidden")
        .description("")
        .styles(indexmap! { "display" => "hidden" })
        .build(),
      NamedClass::builder()
        .name("invisible")
        .description("")
        .styles(indexmap! { "visibility" => "hidden" })
        .build(),
      NamedClass::builder()
        .name("visible")
        .description("")
        .styles(indexmap! { "visibility" => "visible" })
        .build(),
      NamedClass::builder()
        .name("removeFilter")
        .description("")
        .styles(indexmap! { group_nested_filter => "none" })
        .build(),
      NamedClass::builder()
        .name("removeBackdrop")
        .description("")
        .styles(indexmap! { group_nested_backdrop => "none" })
        .build(),
      NamedClass::builder()
        .name("transformGpu")
        .description("")
        .styles(
          indexmap! { &group_nested_transform => format!("var({group_nested_transform_gpu})") },
        )
        .build(),
      NamedClass::builder()
        .name("transformCpu")
        .description("")
        .styles(
          indexmap! { &group_nested_transform => format!("var({group_nested_transform_cpu})") },
        )
        .build(),
    ]
  };
}
