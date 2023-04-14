use indexmap::indexmap;
use lazy_static::lazy_static;
use skribble_core::Atom;
use skribble_core::LinkedValues;
use skribble_core::OptionalStringMap;
use skribble_core::Placeholder;

lazy_static! {
  pub(crate) static ref ATOMS: Vec<Atom> = {
    let none: Option<&String> = None;
    let enter_scale = Placeholder::variable("enterScale");
    let bg_opacity = Placeholder::variable("bgOpacity");
    let animation_duration = Placeholder::variable("animationDuration");
    let enter_opacity = Placeholder::variable("enterOpacity");
    let enter_translate_x = Placeholder::variable("enterTranslateX");
    let enter_translate_y = Placeholder::variable("enterTranslateY");
    let enter_scale_x = Placeholder::variable("enterScaleX");
    let enter_scale_y = Placeholder::variable("enterScaleY");
    let enter_rotate = Placeholder::variable("enterRotate");
    let exit_opacity = Placeholder::variable("exitOpacity");
    let exit_translate_x = Placeholder::variable("exitTranslateX");
    let exit_translate_y = Placeholder::variable("exitTranslateY");
    let exit_scale_x = Placeholder::variable("exitScaleX");
    let exit_scale_y = Placeholder::variable("exitScaleY");
    let exit_rotate = Placeholder::variable("exitRotate");

    vec![
      Atom::builder()
        .name("sr")
        .description("Screen reader only")
        .styles(OptionalStringMap::default())
        .values(vec!["screenReader"])
        .build(),
      Atom::builder()
        .name("transition")
        .description("A class to apply transition effects.")
        .styles(indexmap! { "transition" => none })
        .values(vec!["transition"])
        .build(),
      Atom::builder()
        .name("transitionProperty")
        .styles(indexmap! { "transition-property" => none })
        .values(vec!["transitionProperties"])
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
        .name("fadeIn")
        .description(
          "Fade in an element. Requires the `animate:$in` class to be applied to the element.",
        )
        .styles(indexmap! { &enter_opacity => none })
        .values(vec!["opacity"])
        .build(),
      Atom::builder()
        .name("fadeOut")
        .description(
          "Fade out an element. Requires the `animate:$out` class to be applied to the element.",
        )
        .styles(indexmap! { &exit_opacity => none })
        .values(vec!["opacity"])
        .build(),
      Atom::builder()
        .name("spinIn")
        .description(
          "Rotate an element to enter. Requires the `animate:$in` class to be applied to the \
           element.",
        )
        .styles(indexmap! { &enter_rotate => none })
        .values(vec!["rotation"])
        .build(),
      Atom::builder()
        .name("spinOut")
        .description(
          "Rotate an element to exit. Requires the `animate:$out` class to be applied to the \
           element.",
        )
        .styles(indexmap! { &exit_rotate => none })
        .values(vec!["rotation"])
        .build(),
      Atom::builder()
        .name("slideInLeft")
        .description(
          "Entry animation from the left. Requires the `animate:$in` class to be applied to the \
           element.",
        )
        .styles(indexmap! { &enter_translate_x => none })
        .values(vec!["negativeTranslation"])
        .build(),
      Atom::builder()
        .name("slideInRight")
        .description(
          "Entry animation from the right. Requires the `animate:$in` class to be applied to the \
           element.",
        )
        .styles(indexmap! { &enter_translate_x => none })
        .values(vec!["positiveTranslation"])
        .build(),
      Atom::builder()
        .name("slideOutLeft")
        .description(
          "Exit animation to the left. Requires the `animate:$out` class to be applied to the \
           element.",
        )
        .styles(indexmap! { &exit_translate_x => none })
        .values(vec!["negativeTranslation"])
        .build(),
      Atom::builder()
        .name("slideOutRight")
        .description(
          "Exit animation to the right. Requires the `animate:$out` class to be applied to the \
           element.",
        )
        .styles(indexmap! { &exit_translate_x => none })
        .values(vec!["positiveTranslation"])
        .build(),
      Atom::builder()
        .name("slideInTop")
        .description(
          "Entry animation from the top. Requires the `animate:$in` class to be applied to the \
           element.",
        )
        .styles(indexmap! { &enter_translate_y => none })
        .values(vec!["negativeTranslation"])
        .build(),
      Atom::builder()
        .name("slideInBottom")
        .description(
          "Entry animation from the bottom. Requires the `animate:$in` class to be applied to the \
           element.",
        )
        .styles(indexmap! { &enter_translate_y => none })
        .values(vec!["positiveTranslation"])
        .build(),
      Atom::builder()
        .name("slideOutTop")
        .description(
          "Exit animation to the top. Requires the `animate:$out` class to be applied to the \
           element.",
        )
        .styles(indexmap! { &exit_translate_y => none })
        .values(vec!["negativeTranslation"])
        .build(),
      Atom::builder()
        .name("slideOutBottom")
        .description(
          "Exit animation to the bottom. Requires the `animate:$out` class to be applied to the \
           element.",
        )
        .styles(indexmap! { &exit_translate_y => none })
        .values(vec!["positiveTranslation"])
        .build(),
      Atom::builder()
        .name("zoomIn")
        .description(
          "Entry animation from this zoom level. Requires the `animate:$in` class to be applied \
           to the element.",
        )
        .styles(indexmap! { &enter_scale_x => none, &enter_scale_y => none })
        .values(vec!["zoom"])
        .build(),
      Atom::builder()
        .name("zoomOut")
        .description(
          "Exit animation from this zoom level. Requires the `animate:$out` class to be applied \
           to the element.",
        )
        .styles(indexmap! { &exit_scale_x => none, &exit_scale_y => none })
        .values(vec!["zoom"])
        .build(),
      Atom::builder()
        .name("animateDuration")
        .styles(indexmap! { "animation-duration" => none })
        .values(vec!["duration"])
        .build(),
      Atom::builder()
        .name("animateEasing")
        .styles(indexmap! { "animation-timing-function" => none })
        .values(vec!["easing"])
        .build(),
      Atom::builder()
        .name("animateDelay")
        .styles(indexmap! { "animation-delay" => none })
        .values(vec!["duration"])
        .build(),
      Atom::builder()
        .name("animateRepeat")
        .styles(indexmap! { "animation-iteration-count" => none })
        .values(vec!["animationRepetitions"])
        .build(),
      Atom::builder()
        .name("animateDirection")
        .styles(indexmap! { "animation-direction" => none })
        .values(vec!["animationDirection"])
        .build(),
      Atom::builder()
        .name("animateFillMode")
        .values(vec!["animationFillMode"])
        .styles(indexmap! { "animation-fill-mode" => none })
        .build(),
      Atom::builder()
        .name("animateState")
        .values(vec!["animationState"])
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
        .values(vec!["spacing"])
        .styles(indexmap! { "padding" => none })
        .build(),
      Atom::builder()
        .name("py")
        .values(vec!["spacing"])
        .styles(indexmap! { "padding-top" => none, "padding-bottom" => none })
        .build(),
      Atom::builder()
        .name("px")
        .values(vec!["spacing"])
        .styles(indexmap! { "padding-right" => none, "padding-left" => none })
        .build(),
      Atom::builder()
        .name("pt")
        .values(vec!["spacing"])
        .styles(indexmap! { "padding-top" => none })
        .build(),
      Atom::builder()
        .name("pr")
        .values(vec!["spacing"])
        .styles(indexmap! { "padding-right" => none })
        .build(),
      Atom::builder()
        .name("pb")
        .values(vec!["spacing"])
        .styles(indexmap! { "padding-bottom" => none })
        .build(),
      Atom::builder()
        .name("pl")
        .values(vec!["spacing"])
        .styles(indexmap! { "padding-left" => none })
        .build(),
      Atom::builder()
        .name("pbl")
        .values(vec!["spacing"])
        .styles(indexmap! { "padding-block" => none })
        .build(),
      Atom::builder()
        .name("pbls")
        .values(vec!["spacing"])
        .styles(indexmap! { "padding-block-start" => none })
        .build(),
      Atom::builder()
        .name("pble")
        .values(vec!["spacing"])
        .styles(indexmap! { "padding-block-end" => none })
        .build(),
      Atom::builder()
        .name("pin")
        .values(vec!["spacing"])
        .styles(indexmap! { "padding-inline" => none })
        .build(),
      Atom::builder()
        .name("pins")
        .values(vec!["spacing"])
        .styles(indexmap! { "padding-inline-start" => none })
        .build(),
      Atom::builder()
        .name("pine")
        .values(vec!["spacing"])
        .styles(indexmap! { "padding-inline-end" => none })
        .build(),
      Atom::builder()
        .name("m")
        .values(vec!["spacing"])
        .styles(indexmap! { "margin" => none })
        .build(),
      Atom::builder()
        .name("my")
        .values(vec!["spacing"])
        .styles(indexmap! { "margin-top" => none, "margin-bottom" => none })
        .build(),
      Atom::builder()
        .name("mx")
        .values(vec!["spacing"])
        .styles(indexmap! { "margin-right" => none, "margin-left" => none })
        .build(),
      Atom::builder()
        .name("mt")
        .values(vec!["spacing"])
        .styles(indexmap! { "margin-top" => none })
        .build(),
      Atom::builder()
        .name("mr")
        .values(vec!["spacing"])
        .styles(indexmap! { "margin-right" => none })
        .build(),
      Atom::builder()
        .name("mb")
        .values(vec!["spacing"])
        .styles(indexmap! { "margin-bottom" => none })
        .build(),
      Atom::builder()
        .name("ml")
        .values(vec!["spacing"])
        .styles(indexmap! { "margin-left" => none })
        .build(),
      Atom::builder()
        .name("z")
        .values(vec!["zIndex"])
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
        .name("bgOpacity")
        .values(vec!["opacity"])
        .styles(indexmap! { bg_opacity => none })
        .build(),
      Atom::builder()
        .name("opacity")
        .values(vec!["opacity"])
        .styles(indexmap! { "opacity" => none })
        .build(),
    ]
  };
}
