use indexmap::indexmap;
use lazy_static::lazy_static;
use skribble_core::Keyframe;
use skribble_core::Placeholder;

lazy_static! {
  pub(crate) static ref KEYFRAMES: Vec<Keyframe> = {
    let enter_opacity = Placeholder::wrapped_variable("enter-opacity", Some("1".into()));
    let enter_translate_x = Placeholder::wrapped_variable("enter-translate-x", Some("0".into()));
    let enter_translate_y = Placeholder::wrapped_variable("enter-translate-y", Some("0".into()));
    let enter_scale_x = Placeholder::wrapped_variable("enter-scale-x", Some("1".into()));
    let enter_scale_y = Placeholder::wrapped_variable("enter-scale-y", Some("1".into()));
    let enter_rotate = Placeholder::wrapped_variable("enter-rotate", Some("0deg".into()));
    let exit_opacity = Placeholder::wrapped_variable("exit-opacity", Some("1".into()));
    let exit_translate_x = Placeholder::wrapped_variable("exit-translate-x", Some("0".into()));
    let exit_translate_y = Placeholder::wrapped_variable("exit-translate-y", Some("0".into()));
    let exit_scale_x = Placeholder::wrapped_variable("exit-scale-x", Some("1".into()));
    let exit_scale_y = Placeholder::wrapped_variable("exit-scale-y", Some("1".into()));
    let exit_rotate = Placeholder::wrapped_variable("exit-rotate", Some("0deg".into()));

    vec![
      Keyframe::builder()
        .name("in")
        .rules(indexmap! {
          "from" => indexmap! {
            "opacity" => enter_opacity,
            "transform" => format!("translate3d({enter_translate_x}, {enter_translate_y}, 0) scale3d({enter_scale_x}, {enter_scale_y}, 1) rotate({enter_rotate})")
          }
        })
        .description("Manages the keyframes for the entry animation")
        .build(),
      Keyframe::builder()
        .name("out")
        .rules(indexmap! {
          "to" => indexmap! {
            "opacity" => exit_opacity,
            "transform" => format!("translate3d({exit_translate_x}, {exit_translate_y}, 0) scale3d({exit_scale_x}, {exit_scale_y}, 1) rotate({exit_rotate})")
          }
        })
        .description("Manages the keyframes for the exit animation.")
        .build(),
      Keyframe::builder()
        .name("spin")
        .rules(indexmap! {
          "from" => indexmap! { "transform" => "rotate(0deg)" },
          "to" => indexmap! { "transform" => "rotate(360deg)" }
        })
        .description("Manages the keyframes for the spin animation.")
        .build(),
      Keyframe::builder()
        .name("ping")
        .rules(indexmap! { "75%, 100%" => indexmap! { "transform" => "scale(2)", "opacity" => "0" } })
        .description("Manages the keyframes for the ping animation.")
        .build(),
      Keyframe::builder()
        .name("pulse")
        .description("Manages the keyframes for the pulse animation.")
        .rules(indexmap! { "0%, 100%" => indexmap! { "opacity" => "1" }, "50%" => indexmap! { "opacity" => "0.5" } })
        .build(),
      Keyframe::builder()
        .name("bounce")
        .rules(indexmap! {
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
    ]
  };
}
