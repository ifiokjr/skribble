use indexmap::indexmap;
use lazy_static::lazy_static;
use skribble_core::Atom;
use skribble_core::ColorField;
use skribble_core::LinkedValues;
use skribble_core::OptionalStringMap;
use skribble_core::Placeholder;

lazy_static! {
  pub(crate) static ref ATOMS: Vec<Atom> = {
    let none: Option<&String> = None;
    let enter_scale = Placeholder::variable("enter-scale");
    let animation_duration = Placeholder::variable("animation-duration");
    let enter_opacity = Placeholder::variable("enter-opacity");
    let enter_translate_x = Placeholder::variable("enter-translate-x");
    let enter_translate_y = Placeholder::variable("enter-translate-y");
    let enter_scale_x = Placeholder::variable("enter-scale-x");
    let enter_scale_y = Placeholder::variable("enter-scale-y");
    let enter_rotate = Placeholder::variable("enter-rotate");
    let exit_opacity = Placeholder::variable("exit-opacity");
    let exit_translate_x = Placeholder::variable("exit-translate-x");
    let exit_translate_y = Placeholder::variable("exit-translate-y");
    let exit_scale_x = Placeholder::variable("exit-scale-x");
    let exit_scale_y = Placeholder::variable("exit-scale-y");
    let exit_rotate = Placeholder::variable("exit-rotate");
    let wrapped_space_x_reverse =
      Placeholder::wrapped_variable("space-x-reverse", Some("0".into()));
    let wrapped_space_y_reverse =
      Placeholder::wrapped_variable("space-y-reverse", Some("0".into()));
    let placeholder_value = Placeholder::value("");
    let space_margin_right = format!("calc({placeholder_value} * {wrapped_space_x_reverse})");
    let space_margin_left =
      format!("calc({placeholder_value} * calc(1 - {wrapped_space_x_reverse}))");
    let space_margin_top =
      format!("calc({placeholder_value} * calc(1 - {wrapped_space_y_reverse}))");
    let space_margin_bottom = format!("calc({placeholder_value} * {wrapped_space_y_reverse})");
    let filter_blur = Placeholder::variable("filter-blur");
    let filter_brightness = Placeholder::variable("filter-brightness");
    let filter_contrast = Placeholder::variable("filter-contrast");
    let filter_grayscale = Placeholder::variable("filter-grayscale");
    let filter_hue_rotate = Placeholder::variable("filter-hue-rotate");
    let filter_invert = Placeholder::variable("filter-invert");
    let filter_saturate = Placeholder::variable("filter-saturate");
    let filter_sepia = Placeholder::variable("filter-sepia");
    let filter_drop_shadow = Placeholder::variable("filter-drop-shadow");
    let filter_custom = Placeholder::variable("filter-custom");
    let backdrop_blur = Placeholder::variable("backdrop-blur");
    let backdrop_brightness = Placeholder::variable("backdrop-brightness");
    let backdrop_contrast = Placeholder::variable("backdrop-contrast");
    let backdrop_grayscale = Placeholder::variable("backdrop-grayscale");
    let backdrop_hue_rotate = Placeholder::variable("backdrop-hue-rotate");
    let backdrop_invert = Placeholder::variable("backdrop-invert");
    let backdrop_saturate = Placeholder::variable("backdrop-saturate");
    let backdrop_sepia = Placeholder::variable("backdrop-sepia");
    let backdrop_drop_shadow = Placeholder::variable("backdrop-drop-shadow");
    let backdrop_custom = Placeholder::variable("backdrop-custom");

    let gradient_from = Placeholder::variable("gradient-from");
    let gradient_to = Placeholder::variable("gradient-to");
    let gradient_stops = Placeholder::variable("gradient-stops");
    let gradient_to_position = Placeholder::variable("gradient-to-position");
    let gradient_from_position = Placeholder::variable("gradient-from-position");
    let gradient_via_position = Placeholder::variable("gradient-via-position");
    let wrapped_gradient_to_position = Placeholder::wrapped_variable("gradient-to-position", None);
    let wrapped_gradient_from_position = Placeholder::wrapped_variable("gradient-from-position", None);
    let wrapped_gradient_via_position = Placeholder::wrapped_variable("gradient-via-position", None);
    let wrapped_gradient_from = Placeholder::wrapped_variable("gradient-from", None);
    let wrapped_gradient_to = Placeholder::wrapped_variable("gradient-to", None);
    // Transforms
    let translate_x = Placeholder::variable("translate-x");
    let translate_y = Placeholder::variable("translate-y");
    let rotate = Placeholder::variable("rotate");
    let skew_x = Placeholder::variable("skew-x");
    let skew_y = Placeholder::variable("skew-y");
    let scale_x = Placeholder::variable("scale-x");
    let scale_y = Placeholder::variable("scale-y");
    // Box shadow
    let ring_offset_shadow = Placeholder::variable("ring-offset-shadow");
    let ring_shadow = Placeholder::variable("ring-shadow");
    let wrapped_ring_offset_width = Placeholder::wrapped_variable("ring-offset-width", None);
    let wrapped_ring_offset_color = Placeholder::wrapped_variable("ring-offset-color", None);
    let wrapped_ring_color = Placeholder::wrapped_variable("ring-color", None);
    let ring_offset_width = Placeholder::variable("ring-offset-width");
    let ring_offset_color = Placeholder::variable("ring-offset-color");
    let ring_color = Placeholder::variable("ring-color");
    let wrapped_ring_inset = Placeholder::wrapped_variable("ring-inset", None);
    let shadow = Placeholder::variable("shadow");
    let wrapped_shadow = Placeholder::wrapped_variable("shadow", Some("0 0 #0000".into()));
    let shadow_colored = Placeholder::variable("shadow-colored");
    let wrapped_shadow_colored = Placeholder::wrapped_variable("shadow-colored", None);
    let shadow_color = Placeholder::variable("shadow-color");

    vec![
      Atom::builder()
        .name("sr")
        .description("Screen reader only")
        .styles(OptionalStringMap::default())
        .values(vec!["screen-reader"])
        .build(),
      Atom::builder()
        .name("transition")
        .description("A class to apply transition effects.")
        .styles(OptionalStringMap::default())
        .values(vec!["transition"])
        .build(),
      Atom::builder()
        .name("transition-property")
        .styles(indexmap! { "transition-property" => none })
        .values(vec!["transition-properties"])
        .build(),
      Atom::builder()
        .name("duration")
        .styles(indexmap! { "transition-duration" => none })
        .values(vec!["duration"])
        .build(),
      Atom::builder()
        .name("ease")
        .styles(indexmap! { "transition-timing-function" => none })
        .values(vec!["easing"])
        .build(),
      Atom::builder()
        .name("delay")
        .styles(indexmap! { "transition-delay" => none })
        .values(vec!["duration"])
        .build(),
      Atom::builder()
        .name("animate")
        .values(LinkedValues::Keyframes)
        .styles(indexmap! {
          "animation-name" => none,
          "animation-duration" => Some(&animation_duration)
        })
        .build(),
      Atom::builder()
        .name("fade-in")
        .description(
          "Fade in an element. Requires the `animate:$in` class to be applied to the element.",
        )
        .styles(indexmap! { &enter_opacity => none })
        .values(vec!["opacity"])
        .build(),
      Atom::builder()
        .name("fade-out")
        .description(
          "Fade out an element. Requires the `animate:$out` class to be applied to the element.",
        )
        .styles(indexmap! { &exit_opacity => none })
        .values(vec!["opacity"])
        .build(),
      Atom::builder()
        .name("spin-in")
        .description(
          "Rotate an element to enter. Requires the `animate:$in` class to be applied to the \
           element.",
        )
        .styles(indexmap! { &enter_rotate => none })
        .values(vec!["rotation"])
        .build(),
      Atom::builder()
        .name("spin-out")
        .description(
          "Rotate an element to exit. Requires the `animate:$out` class to be applied to the \
           element.",
        )
        .styles(indexmap! { &exit_rotate => none })
        .values(vec!["rotation"])
        .build(),
      Atom::builder()
        .name("slide-in-left")
        .description(
          "Entry animation from the left. Requires the `animate:$in` class to be applied to the \
           element.",
        )
        .styles(indexmap! { &enter_translate_x => none })
        .values(vec!["negative-translation"])
        .build(),
      Atom::builder()
        .name("slide-in-right")
        .description(
          "Entry animation from the right. Requires the `animate:$in` class to be applied to the \
           element.",
        )
        .styles(indexmap! { &enter_translate_x => none })
        .values(vec!["positive-translation"])
        .build(),
      Atom::builder()
        .name("slide-out-left")
        .description(
          "Exit animation to the left. Requires the `animate:$out` class to be applied to the \
           element.",
        )
        .styles(indexmap! { &exit_translate_x => none })
        .values(vec!["negative-translation"])
        .build(),
      Atom::builder()
        .name("slide-out-right")
        .description(
          "Exit animation to the right. Requires the `animate:$out` class to be applied to the \
           element.",
        )
        .styles(indexmap! { &exit_translate_x => none })
        .values(vec!["positive-translation"])
        .build(),
      Atom::builder()
        .name("slide-in-top")
        .description(
          "Entry animation from the top. Requires the `animate:$in` class to be applied to the \
           element.",
        )
        .styles(indexmap! { &enter_translate_y => none })
        .values(vec!["negative-translation"])
        .build(),
      Atom::builder()
        .name("slide-in-bottom")
        .description(
          "Entry animation from the bottom. Requires the `animate:$in` class to be applied to the \
           element.",
        )
        .styles(indexmap! { &enter_translate_y => none })
        .values(vec!["positive-translation"])
        .build(),
      Atom::builder()
        .name("slide-out-top")
        .description(
          "Exit animation to the top. Requires the `animate:$out` class to be applied to the \
           element.",
        )
        .styles(indexmap! { &exit_translate_y => none })
        .values(vec!["negative-translation"])
        .build(),
      Atom::builder()
        .name("slide-out-bottom")
        .description(
          "Exit animation to the bottom. Requires the `animate:$out` class to be applied to the \
           element.",
        )
        .styles(indexmap! { &exit_translate_y => none })
        .values(vec!["positive-translation"])
        .build(),
      Atom::builder()
        .name("zoom-in")
        .description(
          "Entry animation from this zoom level. Requires the `animate:$in` class to be applied \
           to the element.",
        )
        .styles(indexmap! { &enter_scale_x => none, &enter_scale_y => none })
        .values(vec!["zoom"])
        .build(),
      Atom::builder()
        .name("zoom-out")
        .description(
          "Exit animation from this zoom level. Requires the `animate:$out` class to be applied \
           to the element.",
        )
        .styles(indexmap! { &exit_scale_x => none, &exit_scale_y => none })
        .values(vec!["zoom"])
        .build(),
      Atom::builder()
        .name("animate-duration")
        .styles(indexmap! { "animation-duration" => none })
        .values(vec!["duration"])
        .build(),
      Atom::builder()
        .name("animate-easing")
        .styles(indexmap! { "animation-timing-function" => none })
        .values(vec!["easing"])
        .build(),
      Atom::builder()
        .name("animate-delay")
        .styles(indexmap! { "animation-delay" => none })
        .values(vec!["duration"])
        .build(),
      Atom::builder()
        .name("animate-repeat")
        .styles(indexmap! { "animation-iteration-count" => none })
        .values(vec!["animation-repetitions"])
        .build(),
      Atom::builder()
        .name("animate-direction")
        .styles(indexmap! { "animation-direction" => none })
        .values(vec!["animation-direction"])
        .build(),
      Atom::builder()
        .name("animate-fill-mode")
        .values(vec!["animation-fill-mode"])
        .styles(indexmap! { "animation-fill-mode" => none })
        .build(),
      Atom::builder()
        .name("animate-state")
        .values(vec!["animation-state"])
        .styles(indexmap! { "animation-play-state" => none })
        .build(),
      Atom::builder()
        .name("zoom")
        .values(vec!["zoom"])
        .styles(indexmap! {enter_scale => none })
        .build(),
      Atom::builder()
        .name("p")
        .values(vec!["spacing", "negative-spacing"])
        .styles(indexmap! { "padding" => none })
        .build(),
      Atom::builder()
        .name("py")
        .values(vec!["spacing", "negative-spacing"])
        .styles(indexmap! { "padding-top" => none, "padding-bottom" => none })
        .build(),
      Atom::builder()
        .name("px")
        .values(vec!["spacing", "negative-spacing"])
        .styles(indexmap! { "padding-right" => none, "padding-left" => none })
        .build(),
      Atom::builder()
        .name("p-block")
        .values(vec!["spacing", "negative-spacing"])
        .styles(indexmap! { "padding-block" => none })
        .build(),
      Atom::builder()
        .name("pbs")
        .values(vec!["spacing", "negative-spacing"])
        .styles(indexmap! { "padding-block-start" => none })
        .build(),
      Atom::builder()
        .name("pbe")
        .values(vec!["spacing", "negative-spacing"])
        .styles(indexmap! { "padding-block-end" => none })
        .build(),
      Atom::builder()
        .name("p-inline")
        .values(vec!["spacing", "negative-spacing"])
        .styles(indexmap! { "padding-inline" => none })
        .build(),
      Atom::builder()
        .name("ps")
        .values(vec!["spacing", "negative-spacing"])
        .styles(indexmap! { "padding-inline-start" => none })
        .build(),
      Atom::builder()
        .name("pe")
        .values(vec!["spacing", "negative-spacing"])
        .styles(indexmap! { "padding-inline-end" => none })
        .build(),
      Atom::builder()
        .name("pt")
        .values(vec!["spacing", "negative-spacing"])
        .styles(indexmap! { "padding-top" => none })
        .build(),
      Atom::builder()
        .name("pr")
        .values(vec!["spacing", "negative-spacing"])
        .styles(indexmap! { "padding-right" => none })
        .build(),
      Atom::builder()
        .name("pb")
        .values(vec!["spacing", "negative-spacing"])
        .styles(indexmap! { "padding-bottom" => none })
        .build(),
      Atom::builder()
        .name("pl")
        .values(vec!["spacing", "negative-spacing"])
        .styles(indexmap! { "padding-left" => none })
        .build(),
      Atom::builder()
        .name("m")
        .values(vec!["spacing", "negative-spacing"])
        .styles(indexmap! { "margin" => none })
        .build(),
      Atom::builder()
        .name("my")
        .values(vec!["spacing", "negative-spacing"])
        .styles(indexmap! { "margin-top" => none, "margin-bottom" => none })
        .build(),
      Atom::builder()
        .name("mx")
        .values(vec!["spacing", "negative-spacing"])
        .styles(indexmap! { "margin-right" => none, "margin-left" => none })
        .build(),
      Atom::builder()
        .name("m-block")
        .values(vec!["spacing", "negative-spacing"])
        .styles(indexmap! { "margin-block" => none })
        .build(),
      Atom::builder()
        .name("mbs")
        .values(vec!["spacing", "negative-spacing"])
        .styles(indexmap! { "margin-block-start" => none })
        .build(),
      Atom::builder()
        .name("mbe")
        .values(vec!["spacing", "negative-spacing"])
        .styles(indexmap! { "margin-block-end" => none })
        .build(),
      Atom::builder()
        .name("m-inline")
        .values(vec!["spacing", "negative-spacing"])
        .styles(indexmap! { "margin-inline" => none })
        .build(),
      Atom::builder()
        .name("ms")
        .values(vec!["spacing", "negative-spacing"])
        .styles(indexmap! { "margin-inline-start" => none })
        .build(),
      Atom::builder()
        .name("me")
        .values(vec!["spacing", "negative-spacing"])
        .styles(indexmap! { "margin-inline-end" => none })
        .build(),
      Atom::builder()
        .name("mt")
        .values(vec!["spacing", "negative-spacing"])
        .styles(indexmap! { "margin-top" => none })
        .build(),
      Atom::builder()
        .name("mr")
        .values(vec!["spacing", "negative-spacing"])
        .styles(indexmap! { "margin-right" => none })
        .build(),
      Atom::builder()
        .name("mb")
        .values(vec!["spacing", "negative-spacing"])
        .styles(indexmap! { "margin-bottom" => none })
        .build(),
      Atom::builder()
        .name("ml")
        .values(vec!["spacing", "negative-spacing"])
        .styles(indexmap! { "margin-left" => none })
        .build(),
      Atom::builder()
        .name("space-x")
        .description("Control the horizontal space between child elements.")
        .modifier("&>:not([hidden])~:not([hidden])")
        .values(vec!["spacing", "negative-spacing"])
        .styles(indexmap! {
          "margin-right" => Some(&space_margin_right),
          "margin-left" => Some(&space_margin_left),
        })
        .build(),
      Atom::builder()
        .name("space-y")
        .description("Control the vertical space between child elements.")
        .modifier("&>:not([hidden])~:not([hidden])")
        .values(vec!["spacing", "negative-spacing"])
        .styles(indexmap! {
          "margin-top" => Some(&space_margin_top),
          "margin-bottom" => Some(&space_margin_bottom),
        })
        .build(),
      Atom::builder()
        .name("z")
        .values(vec!["z-index"])
        .styles(indexmap! { "z-index" => none })
        .build(),
      Atom::builder()
        .name("dir")
        .values(vec!["direction"])
        .styles(indexmap! { "direction" => none })
        .build(),
      Atom::builder()
        .name("aspect")
        .values(vec!["ratio"])
        .description("Control the aspect ratio of an element.")
        .styles(indexmap! { "aspect-ratio" => none })
        .build(),
      Atom::builder()
        .name("columns")
        .values(vec!["grid-count", "grid-size"])
        .description("Control the number of columns within an element.")
        .styles(indexmap! { "columns" => none })
        .build(),
      Atom::builder()
        .name("break-after")
        .values(vec!["break"])
        .description("Control how a column or page should break after an element.")
        .styles(indexmap! { "break-after" => none })
        .build(),
      Atom::builder()
        .name("break-before")
        .values(vec!["break"])
        .description("Control how a column or page should break before an element.")
        .styles(indexmap! { "break-before" => none })
        .build(),
      Atom::builder()
        .name("break-inside")
        .values(vec!["break-inside"])
        .description("Control how a column or page should break within an element.")
        .styles(indexmap! { "break-inside" => none })
        .build(),
      Atom::builder()
        .name("box-decoration")
        .values(vec!["box-decoration"])
        .description(
          "Use the box-decoration-slice and box-decoration-clone utilities to control whether \
           properties like background, border, border-image, box-shadow, clip-page, margin, and \
           padding should be rendered as if the element were one continuous fragment, or distinct \
           blocks.
        ",
        )
        .styles(indexmap! { "box-decoration-break" => none })
        .build(),
      Atom::builder()
        .name("box")
        .values(vec!["box"])
        .description("Control how the browser should calculate an element's total size.")
        .styles(indexmap! { "box-sizing" => none })
        .build(),
      Atom::builder()
        .name("display")
        .values(vec!["display"])
        .styles(indexmap! { "display" => none })
        .build(),
      Atom::builder()
        .name("visibility")
        .values(vec!["visibility"])
        .styles(indexmap! { "visibility" => none })
        .build(),
      Atom::builder()
        .name("float")
        .values(vec!["float"])
        .styles(indexmap! { "float" => none })
        .build(),
      Atom::builder()
        .name("clear")
        .values(vec!["float", "clear"])
        .styles(indexmap! { "clear" => none })
        .build(),
      Atom::builder()
        .name("isolate")
        .description("Control whether an element should explicitly create a new stacking context. More details: https://developer.mozilla.org/en-US/docs/Web/CSS/isolation")
        .values(vec!["isolation"])
        .styles(indexmap! { "isolation" => none })
        .build(),
      Atom::builder()
        .name("object-fit")
        .description("Control how a replaced element's content should be resized.")
        .values(vec!["object-fit"])
        .styles(indexmap! { "object-fit" => none })
        .build(),
      Atom::builder()
        .name("object-position")
        .description("Control how a replaced element's content should be positioned within its container.")
        .values(vec!["object-position"])
        .styles(indexmap! { "object-position" => none })
        .build(),
      Atom::builder()
        .name("overflow")
        .description("Control how an element handles content that is too large for the container.")
        .values(vec!["overflow"])
        .styles(indexmap! { "overflow" => none })
        .build(),
        Atom::builder()
        .name("overflow-x")
        .description("Control how an element handles horizontal content that is too large for the container.")
        .values(vec!["overflow"])
        .styles(indexmap! { "overflow-x" => none })
        .build(),
        Atom::builder()
        .name("overflow-y")
        .description("Control how an element handles vertical content that is too large for the container.")
        .values(vec!["overflow"])
        .styles(indexmap! { "overflow-y" => none })
        .build(),
      Atom::builder()
        .name("overscroll")
        .description("Control how the browser behaves when reaching the boundary of a scrolling area.")
        .values(vec!["overscroll"])
        .styles(indexmap! { "overscroll-behavior" => none })
        .build(),
      Atom::builder()
        .name("overscroll-x")
        .description("Control how the browser behaves when reaching the horizontal boundary of a scrolling area.")
        .values(vec!["overscroll"])
        .styles(indexmap! { "overscroll-behavior-x" => none })
        .build(),
      Atom::builder()
        .name("overscroll-y")
        .description("Control how the browser behaves when reaching the vertical boundary of a scrolling area.")
        .values(vec!["overscroll"])
        .styles(indexmap! { "overscroll-behavior-y" => none })
        .build(),
      Atom::builder()
        .name("position")
        .description("Control how an element is positioned in the DOM.")
        .values(vec!["position"])
        .styles(indexmap! { "position" => none })
        .build(),
      Atom::builder()
        .name("inset")
        .description("Control the placement of positioned elements.")
        .values(vec!["spacing", "relative-spacing", "negative-spacing", "negative-relative-spacing"])
        .styles(indexmap! { "inset" => none })
        .build(),
      Atom::builder()
        .name("inset-x")
        .description("Control the placement of positioned elements.")
        .values(vec!["spacing", "relative-spacing", "negative-spacing", "negative-relative-spacing"])
        .styles(indexmap! { "left" => none, "right" => none })
        .build(),
      Atom::builder()
        .name("inset-y")
        .description("Control the placement of positioned elements.")
        .values(vec!["spacing", "relative-spacing", "negative-spacing", "negative-relative-spacing"])
        .styles(indexmap! { "top" => none, "bottom" => none })
        .build(),
      Atom::builder()
        .name("start")
        .description("Control the placement of positioned elements.")
        .values(vec!["spacing", "relative-spacing", "negative-spacing", "negative-relative-spacing"])
        .styles(indexmap! { "inset-inline-start" => none })
        .build(),
      Atom::builder()
        .name("end")
        .description("Control the placement of positioned elements.")
        .values(vec!["spacing", "relative-spacing", "negative-spacing", "negative-relative-spacing"])
        .styles(indexmap! { "inset-inline-end" => none })
        .build(),
      Atom::builder()
        .name("top")
        .description("Control the placement of positioned elements.")
        .values(vec!["spacing", "relative-spacing", "negative-spacing", "negative-relative-spacing"])
        .styles(indexmap! { "top" => none })
        .build(),
      Atom::builder()
        .name("right")
        .description("Control the placement of positioned elements.")
        .values(vec!["spacing", "relative-spacing", "negative-spacing", "negative-relative-spacing"])
        .styles(indexmap! { "right" => none })
        .build(),
      Atom::builder()
        .name("bottom")
        .description("Control the placement of positioned elements.")
        .values(vec!["spacing", "relative-spacing", "negative-spacing", "negative-relative-spacing"])
        .styles(indexmap! { "bottom" => none })
        .build(),
      Atom::builder()
        .name("left")
        .description("Control the placement of positioned elements.")
        .values(vec!["spacing", "relative-spacing", "negative-spacing", "negative-relative-spacing"])
        .styles(indexmap! { "left" => none })
        .build(),
      Atom::builder()
        .name("blur")
        .description("Control the blur filters to an element.")
        .values(vec!["blur"])
        .children(vec!["filter"])
        .styles(indexmap! { &filter_blur => none })
        .build(),
      Atom::builder()
        .name("brightness")
        .description("Control the brightness filters to an element.")
        .values(vec!["brightness"])
        .children(vec!["filter"])
        .styles(indexmap! { &filter_brightness => none })
        .build(),
      Atom::builder()
        .name("contrast")
        .description("Control the contrast filters to an element.")
        .values(vec!["contrast"])
        .children(vec!["filter"])
        .styles(indexmap! { &filter_contrast => none })
        .build(),
      Atom::builder()
        .name("grayscale")
        .description("Control the grayscale filters to an element.")
        .values(vec!["grayscale"])
        .children(vec!["filter"])
        .styles(indexmap! { &filter_grayscale => none })
        .build(),
      Atom::builder()
        .name("hue-rotate")
        .description("Control the hue-rotate filters to an element.")
        .values(vec!["hue-rotate"])
        .children(vec!["filter"])
        .styles(indexmap! { &filter_hue_rotate => none })
        .build(),
      Atom::builder()
        .name("invert")
        .description("Control the invert filters to an element.")
        .values(vec!["invert"])
        .children(vec!["filter"])
        .styles(indexmap! { &filter_invert => none })
        .build(),
      Atom::builder()
        .name("saturate")
        .description("Control the saturate filters to an element.")
        .values(vec!["saturate"])
        .children(vec!["filter"])
        .styles(indexmap! { &filter_saturate => none })
        .build(),
      Atom::builder()
        .name("sepia")
        .description("Control the sepia filters to an element.")
        .values(vec!["sepia"])
        .children(vec!["filter"])
        .styles(indexmap! { &filter_sepia => none })
        .build(),
      Atom::builder()
        .name("drop-shadow")
        .description("Control the drop-shadow filters to an element.")
        .values(vec!["drop-shadow"])
        .children(vec!["filter"])
        .styles(indexmap! { &filter_drop_shadow => none })
        .build(),

      Atom::builder()
        .name("filter")
        .description("Set the custom svg filters.")
        .values(vec!["filter"])
        .children(vec!["filter"])
        .styles(indexmap! { &filter_custom => none })
        .build(),
      Atom::builder()
        .name("backdrop-blur")
        .description("Control the backdrop-blur filters to an element.")
        .values(vec!["blur"])
        .children(vec!["backdrop-filter"])
        .styles(indexmap! { &backdrop_blur => none })
        .build(),
      Atom::builder()
        .name("backdrop-brightness")
        .description("Control the backdrop-brightness filters to an element.")
        .values(vec!["brightness"])
        .children(vec!["backdrop-filter"])
        .styles(indexmap! { &backdrop_brightness => none })
        .build(),
      Atom::builder()
        .name("backdrop-contrast")
        .description("Control the backdrop-contrast filters to an element.")
        .values(vec!["contrast"])
        .children(vec!["backdrop-filter"])
        .styles(indexmap! { &backdrop_contrast => none })
        .build(),
      Atom::builder()
        .name("backdrop-grayscale")
        .description("Control the backdrop-grayscale filters to an element.")
        .values(vec!["grayscale"])
        .children(vec!["backdrop-filter"])
        .styles(indexmap! { &backdrop_grayscale => none })
        .build(),
      Atom::builder()
        .name("backdrop-hue-rotate")
        .description("Control the backdrop-hue-rotate filters to an element.")
        .values(vec!["hue-rotate"])
        .children(vec!["backdrop-filter"])
        .styles(indexmap! { &backdrop_hue_rotate => none })
        .build(),
      Atom::builder()
        .name("backdrop-invert")
        .description("Control the backdrop-invert filters to an element.")
        .values(vec!["invert"])
        .children(vec!["backdrop-filter"])
        .styles(indexmap! { &backdrop_invert => none })
        .build(),
      Atom::builder()
        .name("backdrop-saturate")
        .description("Control the backdrop-saturate filters to an element.")
        .values(vec!["saturate"])
        .children(vec!["backdrop-filter"])
        .styles(indexmap! { &backdrop_saturate => none })
        .build(),
      Atom::builder()
        .name("backdrop-sepia")
        .description("Control the backdrop-sepia filters to an element.")
        .values(vec!["sepia"])
        .children(vec!["backdrop-filter"])
        .styles(indexmap! { &backdrop_sepia => none })
        .build(),
      Atom::builder()
        .name("backdrop-drop-shadow")
        .description("Control the backdrop-drop-shadow filters to an element.")
        .values(vec!["drop-shadow"])
        .children(vec!["backdrop-filter"])
        .styles(indexmap! { &backdrop_drop_shadow => none })
        .build(),
      Atom::builder()
        .name("backdrop-filter")
        .description("Control the backdrop-custom filters to an element.")
        .values(vec!["backdrop-filter"])
        .children(vec!["backdrop-filter"])
        .styles(indexmap! { &backdrop_custom => none })
        .build(),
      Atom::builder()
        .name("w")
        .description("Control the width of an element.")
        .values(vec!["spacing", "relative-spacing", "content-fit", "screen-width"])
        .styles(indexmap! { "width" => none })
        .build(),
      Atom::builder()
        .name("min-w")
        .description("Control the minimum width of an element.")
        .values(vec!["relative-spacing", "content-fit", "screen-width"])
        .styles(indexmap! { "min-width" => none })
        .build(),
      Atom::builder()
        .name("max-w")
        .description("Control the maximum width of an element.")
        .values(vec!["max-width", "content-fit"])
        .styles(indexmap! { "max-width" => none })
        .build(),
      Atom::builder()
        .name("h")
        .description("Control the height of an element.")
        .values(vec!["spacing", "relative-spacing", "content-fit", "screen-height"])
        .styles(indexmap! { "height" => none })
        .build(),
      Atom::builder()
        .name("min-h")
        .description("Control the minimum height of an element.")
        .values(vec!["spacing", "relative-spacing", "content-fit", "screen-height"])
        .styles(indexmap! { "min-height" => none })
        .build(),
      Atom::builder()
        .name("max-h")
        .description("Control the maximum height of an element.")
        .values(vec!["spacing", "relative-spacing", "content-fit", "screen-height"])
        .styles(indexmap! { "max-height" => none })
        .build(),

      // Flex
      Atom::builder()
        .name("basis")
        .description("Control the initial size of flex items.")
        .values(vec!["spacing", "relative-spacing"])
        .styles(indexmap! { "flex-basis" => none })
        .build(),
      Atom::builder()
        .name("flex-direction")
        .description("Control the direction of flex items.")
        .values(vec!["flex-direction"])
        .styles(indexmap! { "flex-direction" => none })
        .build(),
      Atom::builder()
        .name("flex-wrap")
        .description("Control how flex items wrap.")
        .values(vec!["flex-wrap"])
        .styles(indexmap! { "flex-wrap" => none })
        .build(),
      Atom::builder()
        .name("flex")
        .description("Control how flex items both grow and shrink.")
        .values(vec!["flex"])
        .styles(indexmap! { "flex" => none })
        .build(),
      Atom::builder()
        .name("flex-grow")
        .description("Control how flex items grow.")
        .values(vec!["flex-grow"])
        .styles(indexmap! { "flex-grow" => none })
        .build(),
      Atom::builder()
        .name("flex-shrink")
        .description("Control how flex items shrink.")
        .values(vec!["flex-shrink"])
        .styles(indexmap! { "flex-shrink" => none })
        .build(),
      Atom::builder()
        .name("order")
        .description("Control how flex items shrink.")
        .values(vec!["order", "negative-order"])
        .styles(indexmap! { "order" => none })
        .build(),

      // Grid
      Atom::builder()
        .name("grid-cols")
        .description("Specify the columns in a grid layout.")
        .values(vec!["grid-template"])
        .styles(indexmap! { "grid-template-columns" => none })
        .build(),
      Atom::builder()
        .name("grid-rows")
        .description("Specify the rows in a grid layout.")
        .values(vec!["grid-template"])
        .styles(indexmap! { "grid-template-rows" => none })
        .build(),
      Atom::builder()
        .name("col-span")
        .description("Control how elements are sized and placed across grid columns.")
        .values(vec!["grid-span"])
        .styles(indexmap! { "grid-column" => none })
        .build(),
      Atom::builder()
        .name("col-start")
        .description("Control how elements are sized and placed across grid columns.")
        .values(vec!["grid-start-end"])
        .styles(indexmap! { "grid-column-start" => none })
        .build(),
      Atom::builder()
        .name("col-end")
        .description("Control how elements are sized and placed across grid columns.")
        .values(vec!["grid-start-end"])
        .styles(indexmap! { "grid-column-end" => none })
        .build(),
      Atom::builder()
        .name("row-span")
        .description("Control how elements are sized and placed across grid rows.")
        .values(vec!["grid-span"])
        .styles(indexmap! { "grid-row" => none })
        .build(),
      Atom::builder()
        .name("row-start")
        .description("Control how elements are sized and placed across grid rows.")
        .values(vec!["grid-start-end"])
        .styles(indexmap! { "grid-row-start" => none })
        .build(),
      Atom::builder()
        .name("row-end")
        .description("Control how elements are sized and placed across grid rows.")
        .values(vec!["grid-start-end"])
        .styles(indexmap! { "grid-row-end" => none })
        .build(),
      Atom::builder()
        .name("flow")
        .description("Control how elements in a grid are auto-placed.")
        .values(vec!["auto-flow"])
        .styles(indexmap! { "grid-auto-flow" => none })
        .build(),
      Atom::builder()
        .name("auto-cols")
        .description("Control the size of implicitly-created grid columns.")
        .values(vec!["grid-auto"])
        .styles(indexmap! { "grid-auto-columns" => none })
        .build(),
      Atom::builder()
        .name("auto-rows")
        .description("Control the size of implicitly-created grid rows.")
        .values(vec!["grid-auto"])
        .styles(indexmap! { "grid-auto-rows" => none })
        .build(),
      Atom::builder()
        .name("gap")
        .description("Control gutters between grid and flexbox items.")
        .values(vec!["spacing"])
        .styles(indexmap! { "gap" => none })
        .build(),
      Atom::builder()
        .name("gap-x")
        .description("Control gutters between grid and flexbox items.")
        .values(vec!["spacing"])
        .styles(indexmap! { "column-gap" => none })
        .build(),
      Atom::builder()
        .name("gap-y")
        .description("Control gutters between grid and flexbox items.")
        .values(vec!["spacing"])
        .styles(indexmap! { "row-gap" => none })
        .build(),
      Atom::builder()
        .name("justify")
        .description("Control how flex and grid items are positioned along a container's main axis.")
        .values(vec!["justify"])
        .styles(indexmap! { "justify-content" => none })
        .build(),
      Atom::builder()
        .name("justify-items")
        .description("Control how grid items are aligned along their inline axis.")
        .values(vec!["justify-items"])
        .styles(indexmap! { "justify-items" => none })
        .build(),
      Atom::builder()
        .name("justify-self")
        .description("Control how an individual grid item is aligned along its inline axis.")
        .values(vec!["justify-items", "auto"])
        .styles(indexmap! { "justify-self" => none })
        .build(),
      Atom::builder()
        .name("content")
        .description("Control how rows are positioned in multi-row flex and grid containers.")
        .values(vec!["align-content"])
        .styles(indexmap! { "align-content" => none })
        .build(),
      Atom::builder()
        .name("items")
        .description("Control how flex and grid items are positioned along a container's cross axis.")
        .values(vec!["align-items"])
        .styles(indexmap! { "align-items" => none })
        .build(),
      Atom::builder()
        .name("self")
        .description("Control how an individual flex or grid item is positioned along its container's cross axis. The misspelling is to avoid conflicts with the `self` keyword in rust which can't be a raw identifier.")
        .values(vec!["align-self"])
        .styles(indexmap! { "align-self" => none })
        .build(),
      Atom::builder()
        .name("place-content")
        .description("Control how content is justified and aligned at the same time.")
        .values(vec!["place-content"])
        .styles(indexmap! { "place-content" => none })
        .build(),
      Atom::builder()
        .name("place-items")
        .description("Control how items are justified and aligned at the same time.")
        .values(vec!["place-items"])
        .styles(indexmap! { "place-items" => none })
        .build(),
      Atom::builder()
        .name("place-self")
        .description("Control how an individual flex or grid item is positioned along its container's cross axis.")
        .values(vec!["justify-items", "auto"])
        .styles(indexmap! { "place-self" => none })
        .build(),

        // Svg
        Atom::builder()
        .name("fill")
        .values(ColorField::default())
        .styles(indexmap! { "fill" => none })
        .build(),
        Atom::builder()
        .name("stroke")
        .values(ColorField::default())
        .styles(indexmap! { "stroke" => none })
        .build(),
      Atom::builder()
        .name("stroke-width")
        .values(vec!["stroke-width"])
        .styles(indexmap! { "stroke-width" => none })
        .build(),

      // Typography
      Atom::builder()
        .name("font-family")
        .values(vec!["font-family"])
        .styles(indexmap! { "font-family" => none })
        .build(),
      Atom::builder()
        .name("font-size")
        .values(vec!["font-size"])
        .styles(indexmap! {
          "font-size" => Some(Placeholder::value("size")),
          "line-height" => Some(Placeholder::value("height")),
        })
        .build(),
      Atom::builder()
        .name("smoothing")
        .values(vec!["smoothing"])
        .styles(indexmap! {
          "-webkit-font-smoothing" => Some(Placeholder::value("webkit")),
          "-moz-osx-font-smoothing" => Some(Placeholder::value("moz")),
        })
        .build(),
      Atom::builder()
        .name("font-style")
        .values(vec!["font-style"])
        .styles(indexmap! { "font-style" => none })
        .build(),
      Atom::builder()
        .name("font-weight")
        .values(vec!["font-weight"])
        .styles(indexmap! { "font-weight" => none })
        .build(),
      Atom::builder()
        .name("font-numeric")
        .values(vec!["font-variant-numeric"])
        .styles(indexmap! { "font-variant-numeric" => none })
        .build(),
      Atom::builder()
        .name("tracking")
        .values(vec!["letter-spacing"])
        .styles(indexmap! { "letter-spacing" => none })
        .build(),
      Atom::builder()
        .name("line-clamp")
        .values(vec!["line-clamp"])
        .styles(indexmap! {
          "overflow" => Some(Placeholder::value("overflow")),
          "display" => Some(Placeholder::value("display")),
          "-webkit-box-orient" => Some(Placeholder::value("orient")),
          "-webkit-box-clamp" => Some(Placeholder::value("clamp")),
        })
        .build(),
      Atom::builder()
        .name("leading")
        .values(vec!["line-height"])
        .styles(indexmap! { "line-height" => none })
        .build(),
      Atom::builder()
        .name("list-image")
        .values(vec!["none"])
        .styles(indexmap! { "list-style-image" => none })
        .build(),
      Atom::builder()
        .name("list-position")
        .values(vec!["list-style-position"])
        .styles(indexmap! { "list-style-position" => none })
        .build(),
      Atom::builder()
        .name("list-type")
        .values(vec!["list-style-type", "none"])
        .styles(indexmap! { "list-style-type" => none })
        .build(),
      Atom::builder()
        .name("text-align")
        .values(vec!["text-align"])
        .styles(indexmap! { "text-align" => none })
        .build(),
      Atom::builder()
        .name("text")
        .values(ColorField::default())
        .styles(indexmap! { "color" => none })
        .build(),
      Atom::builder()
        .name("text-decoration")
        .values(vec!["text-decoration-line", "none"])
        .styles(indexmap! { "text-decoration-line" => none })
        .build(),
      Atom::builder()
        .name("decoration")
        .values(ColorField::default())
        .styles(indexmap! { "text-decoration-color" => none })
        .build(),
      Atom::builder()
        .name("decoration-style")
        .values(vec!["text-decoration-style"])
        .styles(indexmap! { "text-decoration-style" => none })
        .build(),
      Atom::builder()
        .name("decoration-thickness")
        .values(vec!["text-decoration-thickness"])
        .styles(indexmap! { "text-decoration-thickness" => none })
        .build(),
      Atom::builder()
        .name("underline-offset")
        .values(vec!["text-underline-offset"])
        .styles(indexmap! { "text-underline-offset" => none })
        .build(),
      Atom::builder()
        .name("text-transform")
        .values(vec!["text-transform", "none"])
        .styles(indexmap! { "text-transform" => none })
        .build(),
      Atom::builder()
        .name("text-overflow")
        .values(vec!["text-overflow"])
        .styles(indexmap! { "text-overflow" => none })
        .build(),
      Atom::builder()
        .name("indent")
        .values(vec!["text-indent"])
        .styles(indexmap! { "text-indent" => none })
        .build(),
      Atom::builder()
        .name("align")
        .values(vec!["vertical-align"])
        .styles(indexmap! { "vertical-align" => none })
        .build(),
      Atom::builder()
        .name("whitespace")
        .values(vec!["whitespace"])
        .styles(indexmap! { "white-space" => none })
        .build(),
      Atom::builder()
        .name("break")
        .values(vec!["break"])
        .styles(OptionalStringMap::default())
        .build(),
      Atom::builder()
        .name("hyphens")
        .values(vec!["none", "auto", "hyphens"])
        .styles(indexmap! { "hyphens" => none })
        .build(),
      Atom::builder()
        .name("content")
        .values(vec!["none"])
        .styles(indexmap! { "content" => none })
        .build(),

      // Backgrounds
      Atom::builder()
        .name("bg-attachment")
        .values(vec!["background-attachment"])
        .styles(indexmap! { "background-attachment" => none })
        .build(),
      Atom::builder()
        .name("bg")
        .values(ColorField::default())
        .styles(indexmap! { "background-color" => none })
        .build(),
      Atom::builder()
        .name("bg-clip")
        .values(vec!["background-origin", "background-text"])
        .styles(indexmap! { "background-clip" => none })
        .build(),
      Atom::builder()
        .name("bg-origin")
        .values(vec!["background-origin"])
        .styles(indexmap! { "background-origin" => none })
        .build(),
      Atom::builder()
        .name("bg-position")
        .values(vec!["background-position"])
        .styles(indexmap! { "background-position" => none })
        .build(),
      Atom::builder()
        .name("bg-repeat")
        .values(vec!["background-repeat"])
        .styles(indexmap! { "background-repeat" => none })
        .build(),
      Atom::builder()
        .name("bg-size")
        .values(vec!["background-size"])
        .styles(indexmap! { "background-size" => none })
        .build(),
      Atom::builder()
        .name("bg-gradient")
        .values(vec!["none", "background-gradient"])
        .styles(indexmap! { "background-image" => none })
        .build(),
      Atom::builder()
        .name("from-color")
        .values(ColorField::default())
        .children(vec!["gradient-reference"])
        .styles({
          let transparent_value = Placeholder::value("transparent");
          let gradient_from_value = format!("{placeholder_value} {wrapped_gradient_from_position}");
          let gradient_to_value = format!("{transparent_value} {wrapped_gradient_to_position}");
          let gradient_stops_value = format!("{wrapped_gradient_from}, {wrapped_gradient_to}");
          indexmap! {
            &gradient_from => Some(gradient_from_value),
            &gradient_to => Some(gradient_to_value),
            &gradient_stops => Some(gradient_stops_value),
          }
        })
        .build(),
      Atom::builder()
        .name("from-position")
        .values(vec!["gradient-position"])
        .children(vec!["gradient-reference"])
        .styles(indexmap! { &gradient_from_position => none })
        .build(),
      Atom::builder()
        .name("via-position")
        .values(ColorField::default())
        .children(vec!["gradient-reference"])
        .styles({
          let transparent_value = Placeholder::value("transparent");
          let gradient_to_value = format!("{transparent_value} {wrapped_gradient_to_position}");
          let gradient_stops_value = format!("{wrapped_gradient_from}, {placeholder_value} {wrapped_gradient_via_position}, {wrapped_gradient_to}");
          indexmap! {
            &gradient_to => Some(gradient_to_value),
            &gradient_stops => Some(gradient_stops_value),
          }
        })
        .build(),
      Atom::builder()
        .name("via-position")
        .values(vec!["gradient-position"])
        .children(vec!["gradient-reference"])
        .styles(indexmap! { &gradient_via_position => none })
        .build(),
      Atom::builder()
        .name("to-color")
        .values(ColorField::default())
        .children(vec!["gradient-reference"])
        .styles(indexmap! { &gradient_to => Some(format!("{placeholder_value} {wrapped_gradient_to_position}")) })
        .build(),
      Atom::builder()
        .name("to-position")
        .values(vec!["gradient-position"])
        .children(vec!["gradient-reference"])
        .styles(indexmap! { &gradient_to_position => none })
        .build(),
      // Borders
      Atom::builder()
        .name("rounded")
        .description("Control the border radius of an element.")
        .values(vec!["border-radius"])
        .styles(indexmap! { "border-radius" => none })
        .build(),
      Atom::builder()
        .name("rounded-start")
        .description("Control the `start` border radius of an element.")
        .values(vec!["border-radius"])
        .styles(indexmap! {
          "border-start-start-radius" => none,
          "border-end-start-radius" => none,
        })
        .build(),
      Atom::builder()
        .name("rounded-end")
        .description("Control the `end` border radius of an element.")
        .values(vec!["border-radius"])
        .styles(indexmap! {
          "border-start-end-radius" => none,
          "border-end-end-radius" => none,
        })
        .build(),
      Atom::builder()
        .name("rounded-top")
        .description("Control the `top` border radius of an element.")
        .values(vec!["border-radius"])
        .styles(indexmap! {
          "border-top-left-radius" => none,
          "border-top-right-radius" => none,
        })
        .build(),
      Atom::builder()
        .name("rounded-top")
        .description("Control the `top` border radius of an element.")
        .values(vec!["border-radius"])
        .styles(indexmap! {
          "border-top-left-radius" => none,
          "border-top-right-radius" => none,
        })
        .build(),
      Atom::builder()
        .name("rounded-right")
        .description("Control the `right` border radius of an element.")
        .values(vec!["border-radius"])
        .styles(indexmap! {
          "border-top-right-radius" => none,
          "border-bottom-right-radius" => none,
        })
        .build(),
      Atom::builder()
        .name("rounded-bottom")
        .description("Control the `bottom` border radius of an element.")
        .values(vec!["border-radius"])
        .styles(indexmap! {
          "border-bottom-right-radius" => none,
          "border-bottom-left-radius" => none,
        })
        .build(),
      Atom::builder()
        .name("rounded-left")
        .description("Control the `left` border radius of an element.")
        .values(vec!["border-radius"])
        .styles(indexmap! {
          "border-top-left-radius" => none,
          "border-bottom-left-radius" => none,
        })
        .build(),
      Atom::builder()
        .name("rounded-start-start")
        .description("Control the `start-start` border radius of an element.")
        .values(vec!["border-radius"])
        .styles(indexmap! { "border-start-start-radius" => none })
        .build(),
      Atom::builder()
        .name("rounded-start-end")
        .description("Control the `start-end` border radius of an element.")
        .values(vec!["border-radius"])
        .styles(indexmap! { "border-start-end-radius" => none })
        .build(),
      Atom::builder()
        .name("rounded-end-end")
        .description("Control the `end-end` border radius of an element.")
        .values(vec!["border-radius"])
        .styles(indexmap! { "border-end-end-radius" => none })
        .build(),
      Atom::builder()
        .name("rounded-end-start")
        .description("Control the `end-start` border radius of an element.")
        .values(vec!["border-radius"])
        .styles(indexmap! { "border-end-start-radius" => none })
        .build(),
      Atom::builder()
        .name("rounded-top-left")
        .description("Control the `top-left` border radius of an element.")
        .values(vec!["border-radius"])
        .styles(indexmap! { "border-top-left-radius" => none })
        .build(),
      Atom::builder()
        .name("rounded-top-right")
        .description("Control the `top-right` border radius of an element.")
        .values(vec!["border-radius"])
        .styles(indexmap! { "border-top-right-radius" => none })
        .build(),
      Atom::builder()
        .name("rounded-bottom-left")
        .description("Control the `bottom-left` border radius of an element.")
        .values(vec!["border-radius"])
        .styles(indexmap! { "border-bottom-left-radius" => none })
        .build(),
      Atom::builder()
        .name("rounded-bottom-right")
        .description("Control the `bottom-right` border radius of an element.")
        .values(vec!["border-radius"])
        .styles(indexmap! { "border-bottom-right-radius" => none })
        .build(),
      Atom::builder()
        .name("border")
        .description("Control the border width of an element.")
        .values(vec!["border-width"])
        .styles(indexmap! { "border" => none })
        .build(),
      Atom::builder()
        .name("border-x")
        .description("Control the left and right border width of an element.")
        .values(vec!["border-width"])
        .styles(indexmap! {
          "border-left-width" => none,
          "border-right-width" => none,
        })
        .build(),
      Atom::builder()
        .name("border-y")
        .description("Control the top and bottom border width of an element.")
        .values(vec!["border-width"])
        .styles(indexmap! {
          "border-top-width" => none,
          "border-bottom-width" => none,
        })
        .build(),
      Atom::builder()
        .name("border-inline")
        .values(vec!["border-width"])
        .styles(indexmap! { "border-inline-width" => none })
        .build(),
      Atom::builder()
        .name("border-inline-start")
        .values(vec!["border-width"])
        .styles(indexmap! { "border-inline-start-width" => none })
        .build(),
      Atom::builder()
        .name("border-inline-end")
        .values(vec!["border-width"])
        .styles(indexmap! { "border-inline-end-width" => none })
        .build(),
      Atom::builder()
        .name("border-block")
        .values(vec!["border-width"])
        .styles(indexmap! { "border-block-width" => none })
        .build(),
      Atom::builder()
        .name("border-block-start")
        .values(vec!["border-width"])
        .styles(indexmap! { "border-block-start-width" => none })
        .build(),
      Atom::builder()
        .name("border-block-end")
        .values(vec!["border-width"])
        .styles(indexmap! { "border-block-end-width" => none })
        .build(),
      Atom::builder()
        .name("border-top")
        .values(vec!["border-width"])
        .styles(indexmap! { "border-top-width" => none })
        .build(),
      Atom::builder()
        .name("border-right")
        .values(vec!["border-width"])
        .styles(indexmap! { "border-right-width" => none })
        .build(),
      Atom::builder()
        .name("border-bottom")
        .values(vec!["border-width"])
        .styles(indexmap! { "border-bottom-width" => none })
        .build(),
      Atom::builder()
        .name("border-left")
        .values(vec!["border-width"])
        .styles(indexmap! { "border-left-width" => none })
        .build(),
      Atom::builder()
        .name("border-color")
        .values(ColorField::default())
        .styles(indexmap! { "border-color" => none })
        .build(),
      Atom::builder()
        .name("border-color-x")
        .values(ColorField::default())
        .styles(indexmap! {
          "border-left-color" => none,
          "border-right-color" => none,
        })
        .build(),
      Atom::builder()
        .name("border-color-y")
        .values(ColorField::default())
        .styles(indexmap! {
          "border-top-color" => none,
          "border-bottom-color" => none,
        })
        .build(),
      Atom::builder()
        .name("border-color-inline")
        .values(ColorField::default())
        .styles(indexmap! { "border-inline-color" => none })
        .build(),
      Atom::builder()
        .name("border-color-inline-start")
        .values(ColorField::default())
        .styles(indexmap! { "border-inline-start-color" => none })
        .build(),
      Atom::builder()
        .name("border-color-inline-end")
        .values(ColorField::default())
        .styles(indexmap! { "border-inline-end-color" => none })
        .build(),
      Atom::builder()
        .name("border-color-block")
        .values(ColorField::default())
        .styles(indexmap! { "border-block-color" => none })
        .build(),
      Atom::builder()
        .name("border-color-block-start")
        .values(ColorField::default())
        .styles(indexmap! { "border-block-start-color" => none })
        .build(),
      Atom::builder()
        .name("border-color-block-end")
        .values(ColorField::default())
        .styles(indexmap! { "border-block-end-color" => none })
        .build(),
      Atom::builder()
        .name("border-color-top")
        .values(ColorField::default())
        .styles(indexmap! { "border-top-color" => none })
        .build(),
      Atom::builder()
        .name("border-color-right")
        .values(ColorField::default())
        .styles(indexmap! { "border-right-color" => none })
        .build(),
      Atom::builder()
        .name("border-color-bottom")
        .values(ColorField::default())
        .styles(indexmap! { "border-bottom-color" => none })
        .build(),
      Atom::builder()
        .name("border-color-left")
        .values(ColorField::default())
        .styles(indexmap! { "border-left-color" => none })
        .build(),
      Atom::builder()
        .name("border-style")
        .values(vec!["border-style"])
        .styles(indexmap! { "border-style" => none })
        .build(),
        Atom::builder()
        .name("border-style-x")
        .values(vec!["border-style"])
        .styles(indexmap! {
          "border-left-style" => none,
          "border-right-style" => none,
        })
        .build(),
      Atom::builder()
        .name("border-style-y")
        .values(vec!["border-style"])
        .styles(indexmap! {
          "border-top-style" => none,
          "border-bottom-style" => none,
        })
        .build(),
      Atom::builder()
        .name("border-style-inline")
        .values(vec!["border-style"])
        .styles(indexmap! { "border-inline-style" => none })
        .build(),
      Atom::builder()
        .name("border-style-inline-start")
        .values(vec!["border-style"])
        .styles(indexmap! { "border-inline-start-style" => none })
        .build(),
      Atom::builder()
        .name("border-style-inline-end")
        .values(vec!["border-style"])
        .styles(indexmap! { "border-inline-end-style" => none })
        .build(),
      Atom::builder()
        .name("border-style-block")
        .values(vec!["border-style"])
        .styles(indexmap! { "border-block-style" => none })
        .build(),
      Atom::builder()
        .name("border-style-block-start")
        .values(vec!["border-style"])
        .styles(indexmap! { "border-block-start-style" => none })
        .build(),
      Atom::builder()
        .name("border-style-block-end")
        .values(vec!["border-style"])
        .styles(indexmap! { "border-block-end-style" => none })
        .build(),
      Atom::builder()
        .name("border-style-top")
        .values(vec!["border-style"])
        .styles(indexmap! { "border-top-style" => none })
        .build(),
      Atom::builder()
        .name("border-style-right")
        .values(vec!["border-style"])
        .styles(indexmap! { "border-right-style" => none })
        .build(),
      Atom::builder()
        .name("border-style-bottom")
        .values(vec!["border-style"])
        .styles(indexmap! { "border-bottom-style" => none })
        .build(),
      Atom::builder()
        .name("border-style-left")
        .values(vec!["border-style"])
        .styles(indexmap! { "border-left-style" => none })
        .build(),
      Atom::builder()
        .name("divide-x")
        .values(vec!["divide-x"])
        .modifier("& > * + *")
        .styles(indexmap! {
          "border-left-width" => Some(Placeholder::value("left")),
          "border-right-width" => Some(Placeholder::value("right")),
        })
        .build(),
      Atom::builder()
        .name("divide-y")
        .values(vec!["divide-y"])
        .modifier("& > * + *")
        .styles(indexmap! {
          "border-top-width" => Some(Placeholder::value("top")),
          "border-bottom-width" => Some(Placeholder::value("bottom")),
        })
        .build(),
      Atom::builder()
        .name("divide")
        .values(ColorField::default())
        .modifier("& > * + *")
        .styles(indexmap! { "border-color" => none })
        .build(),
      Atom::builder()
        .name("divide-style")
        .values(vec!["divide-style"])
        .modifier("& > * + *")
        .styles(indexmap! { "border-style" => none })
        .build(),
      Atom::builder()
        .name("outline-width")
        .values(vec!["outline-width"])
        .styles(indexmap! { "outline-width" => none })
        .build(),
        Atom::builder()
        .name("outline")
        .values(ColorField::default())
        .styles(indexmap! { "outline-color" => none })
        .build(),
      Atom::builder()
        .name("outline-style")
        .values(vec!["outline-style"])
        .styles(indexmap! { "outline-style" => none })
        .build(),
      Atom::builder()
        .name("outline-offset")
        .values(vec!["outline-offset"])
        .styles(indexmap! { "outline-offset" => none })
        .build(),
      Atom::builder()
        .name("ring")
        .values(vec!["ring-width"])
        .styles(indexmap! {
          ring_offset_shadow.as_str() => Some(format!("{wrapped_ring_inset} 0 0 0 {wrapped_ring_offset_width} {wrapped_ring_offset_color})")),
          ring_shadow.as_str() => Some(format!("{wrapped_ring_inset} 0 0 0 calc({placeholder_value} + {wrapped_ring_offset_width}) {wrapped_ring_color}")),
          "box-shadow" => Some(format!("{ring_offset_shadow}, {ring_shadow}, {wrapped_shadow}")),
        })
        .build(),
      Atom::builder()
        .name("ring-color")
        .values(ColorField::default())
        .styles(indexmap! { &ring_color => none })
        .build(),
      Atom::builder()
        .name("ring-offset")
        .values(vec!["ring-width"])
        .styles(indexmap! { &ring_offset_width => none })
        .build(),
      Atom::builder()
        .name("ring-offset-color")
        .values(ColorField::default())
        .styles(indexmap! { &ring_offset_color => none })
        .build(),

      // Effects
      Atom::builder()
        .name("shadow")
        .values(vec!["shadow"])
        .styles(indexmap! {
          shadow.as_str() => Some(Placeholder::value("default")),
          shadow_colored.as_str() => Some(Placeholder::value("colored")),
          "box-shadow" => Some(format!("{ring_offset_shadow}, {ring_shadow}, {wrapped_shadow}")),
         })
        .build(),
      Atom::builder()
        .name("shadow-color")
        .values(ColorField::default())
        .styles(indexmap! {
          &shadow_color => Some(placeholder_value),
          &shadow => Some(wrapped_shadow_colored),
        })
        .build(),
      Atom::builder()
        .name("opacity")
        .values(vec!["opacity"])
        .styles(indexmap! { "opacity" => none })
        .build(),
      Atom::builder()
        .name("mix-blend")
        .values(vec!["mix-blend"])
        .styles(indexmap! { "mix-blend-mode" => none })
        .build(),
      Atom::builder()
        .name("bg-blend")
        .values(vec!["mix-blend"])
        .styles(indexmap! { "background-blend-mode" => none })
        .build(),

      // Transforms
      Atom::builder()
        .name("scale")
        .values(vec!["scale"])
        .children(vec!["transform"])
        .styles(indexmap! { &scale_x => none, &scale_y => none })
        .build(),
      Atom::builder()
        .name("scale-x")
        .values(vec!["scale"])
        .children(vec!["transform"])
        .styles(indexmap! { &scale_x => none })
        .build(),
      Atom::builder()
        .name("scale-y")
        .values(vec!["scale"])
        .children(vec!["transform"])
        .styles(indexmap! { &scale_y => none })
        .build(),
      Atom::builder()
        .name("rotate")
        .values(vec!["rotation"])
        .children(vec!["transform"])
        .styles(indexmap! { &rotate => none })
        .build(),
      Atom::builder()
        .name("translate")
        .values(vec!["positive-translation", "negative-translation"])
        .children(vec!["transform"])
        .styles(indexmap! { &translate_x => none, &translate_y => none })
        .build(),
      Atom::builder()
        .name("translate-x")
        .values(vec!["positive-translation", "negative-translation"])
        .children(vec!["transform"])
        .styles(indexmap! { &translate_x => none })
        .build(),
      Atom::builder()
        .name("translate-y")
        .values(vec!["positive-translation", "negative-translation"])
        .children(vec!["transform"])
        .styles(indexmap! { &translate_y => none })
        .build(),
      Atom::builder()
        .name("skew")
        .values(vec!["skew"])
        .children(vec!["transform"])
        .styles(indexmap! { &skew_x => none, &skew_y => none })
        .build(),
      Atom::builder()
        .name("skew-x")
        .values(vec!["skew"])
        .children(vec!["transform"])
        .styles(indexmap! { &skew_x => none })
        .build(),
      Atom::builder()
        .name("skew-y")
        .values(vec!["skew"])
        .children(vec!["transform"])
        .styles(indexmap! { &skew_y => none })
        .build(),
      Atom::builder()
        .name("origin")
        .values(vec!["origin"])
        .styles(indexmap! { "transform-origin" => none })
        .build(),
    ]
  };
}
