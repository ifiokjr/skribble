use indexmap::indexmap;
use lazy_static::lazy_static;
use skribble_core::Atom;
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
    let placeholder_value = Placeholder::value();
    let space_margin_right = format!("calc({placeholder_value} * {wrapped_space_x_reverse})");
    let space_margin_left =
    format!("calc({placeholder_value} * calc(1 - {wrapped_space_x_reverse}))");
    let space_margin_top =
    format!("calc({placeholder_value} * calc(1 - {wrapped_space_y_reverse}))");
    let space_margin_bottom = format!("calc({placeholder_value} * {wrapped_space_y_reverse})");
    // let space_x_reverse = Placeholder::variable("space-x-reverse");
    // let space_y_reverse = Placeholder::variable("space-y-reverse");

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
        .styles(indexmap! { "transition" => none })
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
        .name("font")
        .values(vec!["font"])
        .styles(indexmap! { "font-family" => none })
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
          // space_x_reverse.as_str() => Some("0"),
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
          // space_y_reverse.as_str() => Some("0"),
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
        .name("bg")
        .values(LinkedValues::Color)
        .styles(indexmap! { "background-color" => none })
        .build(),
      Atom::builder()
        .name("opacity")
        .values(vec!["opacity"])
        .styles(indexmap! { "opacity" => none })
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
    ]
  };
}
