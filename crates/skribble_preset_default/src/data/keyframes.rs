use indexmap::indexmap;
use lazy_static::lazy_static;
use skribble_core::Keyframe;
use skribble_core::Placeholder;

lazy_static! {
  pub(crate) static ref KEYFRAMES: Vec<Keyframe> = {
    let enter_opacity = Placeholder::variable("enterOpacity");
    let enter_translate_x = Placeholder::variable("enterTranslateX");
    let enter_translate_y = Placeholder::variable("enterTranslateY");
    let enter_scale = Placeholder::variable("enterScale");
    let enter_rotate = Placeholder::variable("enterRotate");
    let exit_opacity = Placeholder::variable("exitOpacity");
    let exit_translate_x = Placeholder::variable("exitTranslateX");
    let exit_translate_y = Placeholder::variable("exitTranslateY");
    let exit_scale = Placeholder::variable("exitScale");
    let exit_rotate = Placeholder::variable("exitRotate");

    vec![
      Keyframe::builder()
        .name("enter")
        .rules(indexmap! {
          "from" => indexmap! {
            "opacity" => format!("var({enter_opacity}, 1)"),
            "transform" => format!("translate3d(var({enter_translate_x}, 0), var({enter_translate_y}, 0), 0) scale3d(var({enter_scale}, 1), var({enter_scale}, 1), var({enter_scale}, 1)) rotate(var({enter_rotate}, 0))")
          }
        })
        .description("Manages the keyframes for the entry animation")
        .build(),
      Keyframe::builder()
        .name("exit")
        .rules(indexmap! {
          "to" => indexmap! {
            "opacity" => format!("var({exit_opacity}, 1)"),
            "transform" => format!("translate3d(var({exit_translate_x}, 0), var({exit_translate_y}, 0), 0) scale3d(var({exit_scale}, 1), var({exit_scale}, 1), var({exit_scale}, 1)) rotate(var({exit_rotate}, 0))")
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
