use indexmap::indexmap;
use lazy_static::lazy_static;
use skribble_core::NamedClass;
use skribble_core::Placeholder;
use skribble_core::StringMap;

lazy_static! {
  pub(crate) static ref NAMED_CLASSES: Vec<NamedClass> = {
    let group_nested_filter = Placeholder::variable("group-nested-filter");
    let group_nested_backdrop = Placeholder::variable("group-nested-backdrop");
    let group_nested_transform_gpu = Placeholder::variable("group-nested-transform-gpu");
    let group_nested_transform = Placeholder::variable("group-nested-transform");
    let group_nested_transform_cpu = Placeholder::variable("group-nested-transform-cpu");
    let contained_max_width = Placeholder::wrapped_variable("contained-max-width", None);
    let space_x_reverse = Placeholder::variable("space-x-reverse");
    let space_y_reverse = Placeholder::variable("space-y-reverse");

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
        .name("non-italic")
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
        .name("subpixel-antialiased")
        .styles(indexmap! {
          "-webkit-font-smoothing" => "auto",
          "-moz-osx-font-smoothing" => "auto",
        })
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
        .name("remove-filter")
        .styles(indexmap! { group_nested_filter => "none" })
        .build(),
      NamedClass::builder()
        .name("remove-backdrop")
        .styles(indexmap! { group_nested_backdrop => "none" })
        .build(),
      NamedClass::builder()
        .name("transform-gpu")
        .styles(indexmap! { &group_nested_transform => group_nested_transform_gpu })
        .build(),
      NamedClass::builder()
        .name("transform-cpu")
        .styles(indexmap! { &group_nested_transform => group_nested_transform_cpu })
        .build(),
      NamedClass::builder()
        .name("space-x-reverse")
        .styles(indexmap! { space_x_reverse => "1" })
        .build(),
      NamedClass::builder()
        .name("space-y-reverse")
        .styles(indexmap! { space_y_reverse => "1" })
        .build(),
    ]
  };
}
