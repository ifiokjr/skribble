use indexmap::indexmap;
use lazy_static::lazy_static;
use skribble_core::Placeholder;
use skribble_core::VariableGroup;

lazy_static! {
  pub(crate) static ref GROUPS: Vec<VariableGroup> = {
    let group_nested_transform = Placeholder::variable("groupNestedTransform");
    let group_nested_filter = Placeholder::variable("groupNestedFilter");
    let group_nested_backdrop = Placeholder::variable("groupNestedBackdrop");

    vec![
      VariableGroup::builder()
        .name("transform")
        .description("This class makes it possible to use the transform utilities.")
        .styles(indexmap! { "transform" => format!("var({group_nested_transform})") })
        .build(),
      VariableGroup::builder()
        .name("filter")
        .description("This class makes it possible to use the filter utilities.")
        .styles(indexmap! { "filter" => format!("var({group_nested_filter})") })
        .build(),
      VariableGroup::builder()
        .name("backdrop")
        .description("This class makes it possible to use the backdrop filter utilities.")
        .styles(indexmap! { "backdrop-filter" => format!("var({group_nested_backdrop})") })
        .build(),
    ]
  };
}
