use indexmap::indexmap;
use lazy_static::lazy_static;
use skribble_core::Placeholder;
use skribble_core::VariableGroup;

lazy_static! {
  pub(crate) static ref GROUPS: Vec<VariableGroup> = {
    let group_nested_transform = Placeholder::wrapped_variable("groupNestedTransform", None);
    let group_nested_filter = Placeholder::wrapped_variable("groupNestedFilter", None);
    let group_nested_backdrop = Placeholder::wrapped_variable("groupNestedBackdrop", None);

    vec![
      VariableGroup::builder()
        .name("transform")
        .description("This class makes it possible to use the transform utilities.")
        .styles(indexmap! { "transform" => group_nested_transform })
        .build(),
      VariableGroup::builder()
        .name("filter")
        .description("This class makes it possible to use the filter utilities.")
        .styles(indexmap! { "filter" => group_nested_filter })
        .build(),
      VariableGroup::builder()
        .name("backdrop")
        .description("This class makes it possible to use the backdrop filter utilities.")
        .styles(indexmap! { "backdrop-filter" => group_nested_backdrop })
        .build(),
    ]
  };
}
