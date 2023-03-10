use indexmap::indexmap;
use lazy_static::lazy_static;
use skribble_core::Keyframe;

lazy_static! {
  pub(crate) static ref KEYFRAMES: Vec<Keyframe> = vec![
    Keyframe::builder()
      .name("enter")
      .values(vec!["enterAnimation"])
      .rules(indexmap!{
        "from" => indexmap! {
          "opacity" => "var(--sk-enter-opacity, 1)",
          "transform" => "translate3d(var(--sk-enter-translate-x, 0), var(--sk-enter-translate-y, 0), 0) scale3d(var(--sk-enter-scale, 1), var(--sk-enter-scale, 1), var(--sk-enter-scale, 1)) rotate(var(--sk-enter-rotate, 0))"
        }
      })
      .description("Manages the keyframes for the entry animation")
      .build(),
    Keyframe::builder()
      .name("exit")
      .values(vec!["exitAnimation"])
      .rules(indexmap! {
        "to" => indexmap! {
          "opacity" => "var(--sk-exit-opacity, 1)",
          "transform" => "translate3d(var(--sk-exit-translate-x, 0), var(--sk-exit-translate-y, 0), 0) scale3d(var(--sk-exit-scale, 1), var(--sk-exit-scale, 1), var(--sk-exit-scale, 1)) rotate(var(--sk-exit-rotate, 0))"
        }
      })
      .description("Manages the keyframes for the exit animation.")
      .build(),
    Keyframe::builder()
      .name("spin")
      .values(vec!["spinAnimation"])
      .rules(indexmap! {
        "from" => indexmap! { "transform" => "rotate(0deg)" },
        "to" => indexmap! { "transform" => "rotate(360deg)" }
      })
      .description("Manages the keyframes for the spin animation.")
      .build(),
    Keyframe::builder()
      .name("ping")
      .values(vec!["pingAnimation"])
      .rules(indexmap!{ "75%, 100%" => indexmap! { "transform" => "scale(2)", "opacity" => "0" } })
      .description("Manages the keyframes for the ping animation.")
      .build(),
    Keyframe::builder()
      .name("pulse")
      .values(vec!["pulseAnimation"])
      .description("Manages the keyframes for the pulse animation.")
      .rules(indexmap!{ "0%, 100%" => indexmap! { "opacity" => "1" }, "50%" => indexmap! { "opacity" => "0.5" } })
      .build(),
    Keyframe::builder()
      .name("bounce")
      .values(vec!["bounceAnimation"])
      .rules(indexmap!{
        "0%, 100%" => indexmap! {
          "transform" => "translateY(-25%)",
          "animationTimingFunction" => "cubic-bezier(0.8, 0, 1, 1)"
        },
        "50%" => indexmap! {
          "transform" => "translateY(0)",
          "animationTimingFunction" => "cubic-bezier(0, 0, 0.2, 1)"
        }
      })
      .description("Manages the keyframes for the bounce animation.")
      .build(),
  ];
}
