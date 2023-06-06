use indexmap::indexmap;
use lazy_static::lazy_static;
use skribble_core::NamedClass;
use skribble_core::Placeholder;
use skribble_core::StringMap;

lazy_static! {
  pub(crate) static ref NAMED_CLASSES: Vec<NamedClass> = {
    let gradient_from_position = Placeholder::variable("gradient-from-position");
    let gradient_via_position = Placeholder::variable("gradient-via-position");
    let gradient_to_position = Placeholder::variable("gradient-to-position");
    let group_nested_transform_gpu = Placeholder::variable("group-nested-transform-gpu");
    let group_nested_transform = Placeholder::variable("group-nested-transform");
    let group_nested_transform_cpu = Placeholder::variable("group-nested-transform-cpu");
    let contained_max_width = Placeholder::wrapped_variable("contained-max-width", None);
    let space_x_reverse = Placeholder::variable("space-x-reverse");
    let space_y_reverse = Placeholder::variable("space-y-reverse");

    // Transforms
    let translate_x = Placeholder::wrapped_variable("translate-x", None);
    let translate_y = Placeholder::wrapped_variable("translate-y", None);
    let rotate = Placeholder::wrapped_variable("rotate", None);
    let skew_x = Placeholder::wrapped_variable("skew-x", None);
    let skew_y = Placeholder::wrapped_variable("skew-y", None);
    let scale_x = Placeholder::wrapped_variable("scale-x", None);
    let scale_y = Placeholder::wrapped_variable("scale-y", None);
    let transform_gpu = format!("translate3d({translate_x}, {translate_y}, 0) rotate({rotate} skewX({skew_x}) skewY({skew_y}) scaleX({scale_x}) scaleY({scale_y})");
    let transform_cpu = format!("translate({translate_x}, {translate_y}) rotate({rotate}) skewX({skew_x}) skewY({skew_y}) scaleX({scale_x}) scaleY({scale_y})");
    let filter = {
      let filter_blur = Placeholder::wrapped_variable("filter-blur", None);
      let filter_brightness = Placeholder::wrapped_variable("filter-brightness", None);
      let filter_contrast = Placeholder::wrapped_variable("filter-contrast", None);
      let filter_grayscale = Placeholder::wrapped_variable("filter-grayscale", None);
      let filter_hue_rotate = Placeholder::wrapped_variable("filter-hue-rotate", None);
      let filter_invert = Placeholder::wrapped_variable("filter-invert", None);
      let filter_saturate = Placeholder::wrapped_variable("filter-saturate", None);
      let filter_sepia = Placeholder::wrapped_variable("filter-sepia", None);
      let filter_drop_shadow = Placeholder::wrapped_variable("filter-drop-shadow", None);
      let filter_custom = Placeholder::wrapped_variable("filter-custom", None);
      format!(
        "{filter_blur} {filter_brightness} {filter_contrast} {filter_grayscale} \
         {filter_hue_rotate} {filter_invert} {filter_saturate} {filter_sepia} \
         {filter_drop_shadow} {filter_custom}",
      )
    };
    let backdrop_filter = {
      let backdrop_blur = Placeholder::wrapped_variable("backdrop-blur", None);
      let backdrop_brightness = Placeholder::wrapped_variable("backdrop-brightness", None);
      let backdrop_contrast = Placeholder::wrapped_variable("backdrop-contrast", None);
      let backdrop_grayscale = Placeholder::wrapped_variable("backdrop-grayscale", None);
      let backdrop_hue_rotate = Placeholder::wrapped_variable("backdrop-hue-rotate", None);
      let backdrop_invert = Placeholder::wrapped_variable("backdrop-invert", None);
      let backdrop_saturate = Placeholder::wrapped_variable("backdrop-saturate", None);
      let backdrop_sepia = Placeholder::wrapped_variable("backdrop-sepia", None);
      let backdrop_drop_shadow = Placeholder::wrapped_variable("backdrop-drop-shadow", None);
      let backdrop_custom = Placeholder::wrapped_variable("backdrop-custom", None);
      format!(
        "{backdrop_blur} {backdrop_brightness} {backdrop_contrast} {backdrop_grayscale} \
         {backdrop_hue_rotate} {backdrop_invert} {backdrop_saturate} {backdrop_sepia} \
         {backdrop_drop_shadow} {backdrop_custom}",
      )
    };

    let scroll_snap_strictness = Placeholder::variable("scroll-snap-strictness");

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
      NamedClass::builder()
        .name("sr-only")
        .styles(indexmap! {
         "position" => "absolute",
         "width" => "1px",
         "height" => "1px",
         "padding" => "0",
         "margin" => "-1px",
         "overflow" => "hidden",
         "clip" => "rect(0, 0, 0, 0)",
         "white-space" => "nowrap",
         "border-width" => "0",
        })
        .build(),
      NamedClass::builder()
        .name("sr-exclude")
        .styles(indexmap! {
          "position" => "static",
          "width" => "auto",
          "height" => "auto",
          "padding" => "0",
          "margin" => "0",
          "overflow" => "visible",
          "clip" => "auto",
          "white-space" => "normal",
        })
        .build(),
      NamedClass::builder()
        .name("gradient-reference")
        .reference(true)
        .styles(indexmap! {
            &gradient_from_position => "",
            &gradient_via_position => "",
            &gradient_to_position => "",
        })
        .build(),
      NamedClass::builder()
        .name("transform")
        .layer("base")
        .reference(true)
        .styles(indexmap! { "transform" => &transform_cpu })
        .build(),
      NamedClass::builder()
        .name("transform-gpu")
        .layer("priority-class")
        .styles(indexmap! { "transform" => &transform_gpu })
        .build(),
      NamedClass::builder()
        .name("transform-cpu")
        .layer("priority-class")
        .styles(indexmap! { "transform" => &transform_cpu })
        .build(),
      NamedClass::builder()
        .name("outline-none")
        .styles(indexmap! {
          "outline" => "2px solid transparent",
          "outline-offset" => "2px",
         })
        .build(),
      NamedClass::builder()
        .name("shadow-none")
        .styles(indexmap! {
          "box-shadow" => "0 0 #000",
         })
        .build(),
      NamedClass::builder()
        .name("filter")
        .layer("base")
        .reference(true)
        .styles(indexmap! { "filter" => filter })
        .build(),
      NamedClass::builder()
        .name("filter-none")
        .layer("priority-class")
        .styles(indexmap! { "filter" => "none" })
        .build(),
      NamedClass::builder()
        .name("backdrop-filter")
        .layer("base")
        .reference(true)
        .styles(indexmap! { "backdrop-filter" => backdrop_filter })
        .build(),
      NamedClass::builder()
        .name("backdrop-filter-none")
        .layer("priority-class")
        .styles(indexmap! { "backdrop-filter" => "none" })
        .build(),
      NamedClass::builder()
        .name("appearance-none")
        .description("Suppress native form control styling.")
        .styles(indexmap! { "appearance" => "none" })
        .build(),
      NamedClass::builder()
        .name("snap-mandatory")
        .description("Use the `snap-mandatory` utility to force a snap container to always come to rest on a snap point.")
        .styles(indexmap! { &scroll_snap_strictness => "mandatory" })
        .build(),
      NamedClass::builder()
        .name("snap-proximity")
        .description("Use the `snap-proximity` utility to make a snap container come to rest on snap points that are close in proximity. This is the browser default.")
        .styles(indexmap! { &scroll_snap_strictness => "proximity" })
        .build(),
    ]
  };
}
