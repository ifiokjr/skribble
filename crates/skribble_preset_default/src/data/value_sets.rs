use indexmap::indexmap;
use lazy_static::lazy_static;
use skribble_core::Placeholder;
use skribble_core::ValueSet;

lazy_static! {
  pub(crate) static ref ANIMATION_VALUE_SETS: Vec<ValueSet> = {
    let default_animation_duration = Placeholder::variable("defaultAnimationDuration");
    let enter_opacity = Placeholder::variable("enterOpacity");
    let enter_scale = Placeholder::variable("enterScale");
    let enter_rotate = Placeholder::variable("enterRotate");
    let enter_translate_x = Placeholder::variable("enterTranslateX");
    let enter_translate_y = Placeholder::variable("enterTranslateY");
    let exit_opacity = Placeholder::variable("exitOpacity");
    let exit_scale = Placeholder::variable("exitScale");
    let exit_rotate = Placeholder::variable("exitRotate");
    let exit_translate_x = Placeholder::variable("exitTranslateX");
    let exit_translate_y = Placeholder::variable("exitTranslateY");

    vec![
      ValueSet::builder()
        .name("spinAnimation")
        .values(indexmap! { "spin".to_string() => format!("spin var({default_animation_duration}) linear infinite") })
        .description("The spin animation.")
        .build(),
      ValueSet::builder()
        .name("pingAnimation")
        .values(indexmap! { "ping".to_string() => format!("ping var({default_animation_duration}) cubic-bezier(0, 0, 0.2, 1) infinite") })
        .description("The ping animation.")
        .build(),
      ValueSet::builder()
        .name("pulseAnimation")
        .values(indexmap! { "pulse".to_string() => format!("pulse calc(2 * var({default_animation_duration})) cubic-bezier(0.4, 0, 0.6, 1) infinite") })
        .description("The pulse animation.")
        .build(),
      ValueSet::builder()
        .name("bounceAnimation")
        .values(indexmap! { "bounce".to_string() => format!("bounce var({default_animation_duration}) infinite") })
        .description("The bounce animation.")
        .build(),
      ValueSet::builder()
        .name("enterAnimation")
        .values(indexmap! {
          "in".to_string() => indexmap! {
            "animation".to_string() => format!("enter var({default_animation_duration})"),
            enter_opacity => "initial".to_string(),
            enter_scale => "initial".to_string(),
            enter_rotate => "initial".to_string(),
            enter_translate_x => "initial".to_string(),
            enter_translate_y => "initial".to_string()
          }
        })
        .description("The enter animation.")
        .build(),
      ValueSet::builder()
        .name("exitAnimation")
        .values(indexmap! {
          "out".to_string() => indexmap! {
            "animation".to_string() => format!("exit var({default_animation_duration})"),
            exit_opacity => "initial".to_string(),
            exit_scale => "initial".to_string(),
            exit_rotate => "initial".to_string(),
            exit_translate_x => "initial".to_string(),
            exit_translate_y => "initial".to_string()
          }
        })
        .description("The exit animation.")
        .build(),
    ]
  };
}
