use lazy_static::lazy_static;
use skribble_core::CssVariable;

lazy_static! {
  pub(crate) static ref CSS_VARIABLES: Vec<CssVariable> = vec![
    CssVariable::builder()
      .name("containerMaxWidth")
      .variable("--container-max-width")
      .value("container",)
      .description("")
      .build(),
    CssVariable::builder()
      .name("filterBlur")
      .variable("--filter-blur")
      .description("")
      .build(),
    CssVariable::builder()
      .name("filterBrightness")
      .variable("--filter-brightness")
      .description("")
      .build(),
    CssVariable::builder()
      .name("filterContrast")
      .variable("--filter-contrast")
      .description("")
      .build(),
    CssVariable::builder()
      .name("filterCustom")
      .variable("--filter-custom")
      .description("")
      .build(),
    CssVariable::builder()
      .name("filterGrayscale")
      .variable("--filter-grayscale")
      .description("")
      .build(),
    CssVariable::builder()
      .name("filterHueRotate")
      .variable("--filter-hue-rotate")
      .description("")
      .build(),
    CssVariable::builder()
      .name("filterInvert")
      .variable("--filter-invert")
      .description("")
      .build(),
    CssVariable::builder()
      .name("filterSaturate")
      .variable("--filter-saturate")
      .description("")
      .build(),
    CssVariable::builder()
      .name("filterSepia")
      .variable("--filter-sepia")
      .description("")
      .build(),
    CssVariable::builder()
      .name("filterDropShadow")
      .variable("--filter-drop-shadow")
      .description("")
      .build(),
    CssVariable::builder()
      .name("groupNestedFilter")
      .variable("--group-nested-filter")
      .value({
        let filter_blur = CssVariable::placeholder("filterBlur");
        let filter_brightness = CssVariable::placeholder("filterBrightness");
        let filter_contrast = CssVariable::placeholder("filterContrast");
        let filter_grayscale = CssVariable::placeholder("filterGrayscale");
        let filter_hue_rotate = CssVariable::placeholder("filterHueRotate");
        let filter_invert = CssVariable::placeholder("filterInvert");
        let filter_saturate = CssVariable::placeholder("filterSaturate");
        let filter_sepia = CssVariable::placeholder("filterSepia");
        let filter_drop_shadow = CssVariable::placeholder("filterDropShadow");
        let filter_custom = CssVariable::placeholder("filterCustom");
        format!(
          "var({filter_blur}) var({filter_brightness}) var({filter_contrast}) \
           var({filter_grayscale}) var({filter_hue_rotate}) var({filter_invert}) \
           var({filter_saturate}) var({filter_sepia}) var({filter_drop_shadow}) \
           var({filter_custom})",
        )
      })
      .description("")
      .build(),
    CssVariable::builder()
      .name("backdropBlur")
      .variable("--backdrop-blur")
      .description("")
      .build(),
    CssVariable::builder()
      .name("backdropBrightness")
      .variable("--backdrop-brightness")
      .description("")
      .build(),
    CssVariable::builder()
      .name("backdropContrast")
      .variable("--backdrop-contrast")
      .description("")
      .build(),
    CssVariable::builder()
      .name("backdropCustom")
      .variable("--backdrop-custom")
      .description("")
      .build(),
    CssVariable::builder()
      .name("backdropGrayscale")
      .variable("--backdrop-grayscale")
      .description("")
      .build(),
    CssVariable::builder()
      .name("backdropHueRotate")
      .variable("--backdrop-hue-rotate")
      .description("")
      .build(),
    CssVariable::builder()
      .name("backdropInvert")
      .variable("--backdrop-invert")
      .description("")
      .build(),
    CssVariable::builder()
      .name("backdropSaturate")
      .variable("--backdrop-saturate")
      .description("")
      .build(),
    CssVariable::builder()
      .name("backdropSepia")
      .variable("--backdrop-sepia")
      .description("")
      .build(),
    CssVariable::builder()
      .name("backdropDropShadow")
      .variable("--backdrop-drop-shadow")
      .description("")
      .build(),
    CssVariable::builder()
      .name("groupNestedBackdrop")
      .variable("--group-nested-backdrop")
      .value({
        let backdrop_blur = CssVariable::placeholder("backdropBlur");
        let backdrop_brightness = CssVariable::placeholder("backdropBrightness");
        let backdrop_contrast = CssVariable::placeholder("backdropContrast");
        let backdrop_grayscale = CssVariable::placeholder("backdropGrayscale");
        let backdrop_hue_rotate = CssVariable::placeholder("backdropHueRotate");
        let backdrop_invert = CssVariable::placeholder("backdropInvert");
        let backdrop_saturate = CssVariable::placeholder("backdropSaturate");
        let backdrop_sepia = CssVariable::placeholder("backdropSepia");
        let backdrop_drop_shadow = CssVariable::placeholder("backdropDropShadow");
        let backdrop_custom = CssVariable::placeholder("backdropCustom");
        format!(
          "var({backdrop_blur}) var({backdrop_brightness}) var({backdrop_contrast}) \
           var({backdrop_grayscale}) var({backdrop_hue_rotate}) var({backdrop_invert}) \
           var({backdrop_saturate}) var({backdrop_sepia}) var({backdrop_drop_shadow}) \
           var({backdrop_custom})",
        )
      })
      .description("")
      .build(),
    CssVariable::builder()
      .name("defaultTransitionDuration")
      .variable("--default-transition-duration")
      .value("150ms",)
      .description("")
      .build(),
    CssVariable::builder()
      .name("defaultAnimationDuration")
      .variable("--default-animation-duration")
      .value("1s")
      .description("")
      .build(),
  ];
}
