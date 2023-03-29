use indexmap::indexmap;
use lazy_static::lazy_static;
use skribble_core::Keyframe;
use skribble_core::Placeholder;

lazy_static! {
  pub(crate) static ref KEYFRAMES: Vec<Keyframe> = {
    let enter_opacity = Placeholder::wrapped_variable("enterOpacity", Some("1".into()));
    let enter_translate_x = Placeholder::wrapped_variable("enterTranslateX", Some("0".into()));
    let enter_translate_y = Placeholder::wrapped_variable("enterTranslateY", Some("0".into()));
    let enter_scale = Placeholder::wrapped_variable("enterScale", Some("1".into()));
    let enter_rotate = Placeholder::wrapped_variable("enterRotate", Some("0deg".into()));
    let exit_opacity = Placeholder::wrapped_variable("exitOpacity", Some("1".into()));
    let exit_translate_x = Placeholder::wrapped_variable("exitTranslateX", Some("0".into()));
    let exit_translate_y = Placeholder::wrapped_variable("exitTranslateY", Some("0".into()));
    let exit_scale = Placeholder::wrapped_variable("exitScale", Some("1".into()));
    let exit_rotate = Placeholder::wrapped_variable("exitRotate", Some("0deg".into()));

    vec![
      Keyframe::builder()
        .name("enter")
        .rules(indexmap! {
          "from" => indexmap! {
            "opacity" => enter_opacity.into(),
            "transform" => format!("translate3d({enter_translate_x}, {enter_translate_y}, 0) scale3d({enter_scale}, {enter_scale}, {enter_scale}) rotate({enter_rotate})")
          }
        })
        .description("Manages the keyframes for the entry animation")
        .build(),
      Keyframe::builder()
        .name("exit")
        .rules(indexmap! {
          "to" => indexmap! {
            "opacity" => exit_opacity.into(),
            "transform" => format!("translate3d({exit_translate_x}, {exit_translate_y}, 0) scale3d({exit_scale}, {exit_scale}, {exit_scale}) rotate({exit_rotate})")
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
