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
        .values(vec!["transitions"])
        .build(),
      Atom::builder()
        .name("transitionProperty")
        .styles(indexmap! { "transition-property" => none })
        .values(vec!["transitionProperties"])
        .build(),
      Atom::builder()
        .name("duration")
        .styles(indexmap! { "transition-duration" => none })
        .values(vec!["durations"])
        .build(),
      Atom::builder()
        .name("ease")
        .styles(indexmap! { "transition-timing-function" => none })
        .values(vec!["easing"])
        .build(),
      Atom::builder()
        .name("delay")
        .styles(indexmap! { "transition-delay" => none })
        .values(vec!["durations"])
        .build(),
      Atom::builder()
        .name("animationDuration")
        .styles(indexmap! { "animation-duration" => none })
        .values(vec!["durations"])
        .build(),
      Atom::builder()
        .name("animationEasing")
        .styles(indexmap! { "animation-timing-function" => none })
        .values(vec!["easing"])
        .build(),
      Atom::builder()
        .name("animationDelay")
        .styles(indexmap! { "animation-delay" => none })
        .values(vec!["durations"])
        .build(),
      Atom::builder()
        .name("animationRepeat")
        .styles(indexmap! { "animation-iteration-count" => none })
        .values(vec!["animationRepetitions"])
        .build(),
      Atom::builder()
        .name("animationDirection")
        .styles(indexmap! { "animation-direction" => none })
        .values(vec!["animationDirection"])
        .build(),
      Atom::builder()
        .name("animationFillMode")
        .values(vec!["animationFillMode"])
        .styles(indexmap! { "animation-fill-mode" => none })
        .build(),
      Atom::builder()
        .name("animationState")
        .values(vec!["animationState"])
        .styles(indexmap! { "animation-play-state" => none })
        .build(),
      Atom::builder()
        .name("zoom")
        .values(vec!["zoom"])
        .styles(indexmap! {enter_scale.as_str() => none })
        .build(),
      Atom::builder()
        .name("font")
        .values(vec!["fonts"])
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
        .styles(indexmap! { "color" => none })
        .build(),
      Atom::builder()
        .name("bgOpacity")
        .values(vec!["opacity"])
        .styles(indexmap! { bg_opacity.as_str() => none })
        .build(),
      Atom::builder()
        .name("opacity")
        .values(vec!["opacity"])
        .styles(indexmap! { "opacity" => none })
        .build(),
      Atom::builder()
        .name("animate")
        .values(LinkedValues::Keyframes)
        .styles(indexmap! { "animation-name" => none })
        .build(),
    ]
  };
}
