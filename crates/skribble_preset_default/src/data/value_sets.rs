use indexmap::indexmap;
use lazy_static::lazy_static;
use skribble_core::Placeholder;
use skribble_core::ValueSet;

lazy_static! {
  pub(crate) static ref ATOM_VALUE_SETS: Vec<ValueSet> = {
    vec![
      ValueSet::builder()
        .name("screenReader")
        .values(indexmap! {
          "only" => indexmap! {
            "position" => "absolute",
            "width" => "1px",
            "height" => "1px",
            "padding" => "0",
            "margin" => "-1px",
            "overflow" => "hidden",
            "clip" => "rect(0, 0, 0, 0)",
            "whiteSpace" => "nowrap",
            "borderWidth" => "0"
          },
          "exclude" => indexmap! {
            "position" => "static",
            "width" => "auto",
            "height" => "auto",
            "padding" => "0",
            "margin" => "0",
            "overflow" => "visible",
            "clip" => "auto",
            "whiteSpace" => "normal"
          }
        })
        .build(),
      ValueSet::builder()
        .name("transitions")
        .values(indexmap! {
          "" => indexmap! {
            "transition-property" => "color, background-color, border-color, text-decoration-color, fill, stroke, opacity, box-shadow, transform, filter, backdrop-filter",
            "transition-timing-function" => "cubic-bezier(0.4, 0, 0.2, 1)",
            "transition-duration" => "var(--default-transition-duration)"
          },
          "none" => indexmap! { "transitionProperty" => "none" },
          "all" => indexmap! {
            "transition-property" => "all",
            "transition-timing-function" => "cubic-bezier(0.4, 0, 0.2, 1)",
            "transition-duration" => "var(--default-transition-duration)"
          },
          "colors" => indexmap! {
            "transition-property" => "color, background-color, border-color, text-decoration-color, fill, stroke",
            "transition-timing-function" => "cubic-bezier(0.4, 0, 0.2, 1)",
            "transition-duration" => "var(--default-transition-duration)"
          },
          "opacity" => indexmap! {
            "transition-property" => "opacity",
            "transition-timing-function" => "cubic-bezier(0.4, 0, 0.2, 1)",
            "transition-duration" => "var(--default-transition-duration)"
          },
          "shadow" => indexmap! {
            "transition-property" => "box-shadow",
            "transition-timing-function" => "cubic-bezier(0.4, 0, 0.2, 1)",
            "transition-duration" => "var(--default-transition-duration)"
          },
          "transform" => indexmap! {
            "transition-property" => "transform",
            "transition-timing-function" => "cubic-bezier(0.4, 0, 0.2, 1)",
            "transition-duration" => "var(--default-transition-duration)"
          }
        })
        .description("The built-in transition class values.")
        .build(),
      ValueSet::builder()
        .name("transitionProperties")
        .values(indexmap!{
          "" => "color, background-color, border-color, text-decoration-color, fill, stroke, opacity, box-shadow, transform, filter, backdrop-filter",
          "none" => "none",
          "all" => "all",
          "colors" => "color, background-color, border-color, text-decoration-color, fill, stroke",
          "opacity" => "opacity",
          "shadow" => "box-shadow",
          "transform" => "transform",
        })
        .build(),
      ValueSet::builder()
        .name("durations")
        .values(indexmap! {
          "0" => "0ms",
          "75" => "75ms",
          "100" => "100ms",
          "150" => "150ms",
          "200" => "200ms",
          "300" => "300ms",
          "500" => "500ms",
          "700" => "700ms",
          "1000" => "1000ms",
          "1500" => "1500ms",
          "2000" => "2000ms",
        })
        .build(),
      ValueSet::builder()
        .name("easing")
        .values(indexmap! {
          "linear" => "linear",
          "in" => "cubic-bezier(0.4, 0, 1, 1)",
          "out" => "cubic-bezier(0, 0, 0.2, 1)",
          "inOut" => "cubic-bezier(0.4, 0, 0.2, 1)",
        })
        .build(),
      ValueSet::builder()
        .name("animationRepetitions")
        .values(indexmap! {
          "0" => "0",
          "0.5" => "0.5",
          "1" => "1",
          "2" => "2",
          "3" => "3",
          "infinite" => "infinite",
          "in" => "cubic-bezier(0.4, 0, 1, 1)",
          "out" => "cubic-bezier(0, 0, 0.2, 1)",
          "inOut" => "cubic-bezier(0.4, 0, 0.2, 1)",
        })
        .build(),
      ValueSet::builder()
        .name("animationDirection")
        .values(indexmap! {
          "normal" => "normal",
          "reverse" => "reverse",
          "alt" => "alternate",
          "altReverse" => "alternate-reverse"
        })
        .build(),
      ValueSet::builder()
        .name("animationFillMode")
        .values(indexmap! {
          "none" => "none",
          "forwards" => "forwards",
          "backwards" => "backwards",
          "both" => "both",
        })
        .description("These are the default animation fill mode values.")
        .build(),
      ValueSet::builder()
        .name("animationState")
        .values(indexmap! { "running" => "running", "paused" => "paused" })
        .description("These are the default animation state values.")
        .build(),
      ValueSet::builder()
        .name("opacity")
        .values(indexmap! {
          "0" => "0",
          "5" => "5",
          "10" => "10",
          "20" => "20",
          "30" => "30",
          "40" => "40",
          "50" => "50",
          "60" => "60",
          "70" => "70",
          "75" => "75",
          "80" => "80",
          "90" => "90",
          "95" => "95",
          "100" => "100",
         })
        .description("These are the default opacity values.")
        .build(),
      ValueSet::builder()
        .name("spacing")
        .values(indexmap! {
          "0" => "0px",
          "1" => "0.25rem",
          "2" => "0.5rem",
          "3" => "0.75rem",
          "4" => "1rem",
          "5" => "1.25rem",
          "6" => "1.5rem",
          "7" => "1.75rem",
          "8" => "2rem",
          "9" => "2.25rem",
          "10" => "2.5rem",
          "11" => "2.75rem",
          "12" => "3rem",
          "14" => "3.5rem",
          "16" => "4rem",
          "20" => "5rem",
          "24" => "6rem",
          "28" => "7rem",
          "32" => "8rem",
          "36" => "9rem",
          "40" => "10rem",
          "44" => "11rem",
          "48" => "12rem",
          "52" => "13rem",
          "56" => "14rem",
          "60" => "15rem",
          "64" => "16rem",
          "72" => "18rem",
          "80" => "20rem",
          "96" => "24rem",
          "px" => "1px",
          "0.5" => "0.125rem",
          "1.5" => "0.375rem",
          "2.5" => "0.625rem",
          "3.5" => "0.875rem",
         })
        .build(),
      ValueSet::builder()
        .name("zIndex")
        .values(indexmap! {
          "0" => "0",
          "1" => "1",
          "2" => "2",
          "3" => "3",
          "4" => "4",
          "5" => "5",
          "10" => "10",
          "20" => "20",
          "30" => "30",
          "40" => "40",
          "50" => "50",
          "60" => "60",
          "auto" => "auto",
          "-1" => "-1",
          "-2" => "-2",
          "-3" => "-3",
          "-5" => "-5",
         })
        .build(),
      ValueSet::builder()
        .name("direction")
        .values(indexmap! { "ltr" => "ltr", "rtl" => "rtl" })
        .build(),
      ValueSet::builder()
        .name("zoom")
        .values(indexmap! {
          "0" => "0",
          "50" => ".5",
          "75" => ".75",
          "90" => ".9",
          "95" => ".95",
          "100" => "1",
          "105" => "1.05",
          "110" => "1.1",
          "125" => "1.25",
          "150" => "1.5"
        })
        .build(),
      ValueSet::builder()
        .name("zoom")
        .values(indexmap! {
          "0" => "0",
          "50" => ".5",
          "75" => ".75",
          "90" => ".9",
          "95" => ".95",
          "100" => "1",
          "105" => "1.05",
          "110" => "1.1",
          "125" => "1.25",
          "150" => "1.5"
        })
        .build(),
      ValueSet::builder()
        .name("fonts")
        .values(indexmap! {
          "sans" => "ui-sans-serif,system-ui,-apple-system,BlinkMacSystemFont,\"Segoe UI\",Roboto,\"Helvetica Neue\",Arial,\"Noto Sans\",sans-serif,\"Apple Color Emoji\",\"Segoe UI Emoji\",\"Segoe UI Symbol\",\"Noto Color Emoji\"",
          "serif" => "ui-serif,Georgia,Cambria,\"Times New Roman\",Times,serif",
          "mono" => "ui-monospace,SFMono-Regular,Menlo,Monaco,Consolas,\"Liberation Mono\",\"Courier New\",monospace",
        })
        .build(),

    ]
  };
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
