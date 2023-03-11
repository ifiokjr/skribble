use lazy_static::lazy_static;
use skribble_core::CssVariable;
use skribble_core::Placeholder;
use skribble_core::PropertySyntaxValue::Color;

lazy_static! {
  pub(crate) static ref COLOR_CSS_VARIABLES: Vec<CssVariable> = vec![
    CssVariable::builder()
      .name("primary")
      .variable("--p")
      .value("#570df8")
      .description("The primary color. Useful for primary buttons.")
      .syntax(Color)
      .build(),
    CssVariable::builder()
      .name("primaryContent")
      .variable("--pc")
      .value("#ffffff")
      .description("The primary content color")
      .syntax(Color)
      .build(),
    CssVariable::builder()
      .name("secondary")
      .variable("--s")
      .value("#f000b8")
      .description("The secondary color. Useful for secondary buttons.")
      .syntax(Color)
      .build(),
    CssVariable::builder()
      .name("secondaryContent")
      .variable("--sc")
      .value("#ffffff")
      .description("The secondary content color. Useful for text within secondary buttons.")
      .syntax(Color)
      .build(),
    CssVariable::builder()
      .name("accent")
      .variable("--a")
      .value("#37cdbe")
      .description("Color for accents.")
      .syntax(Color)
      .build(),
    CssVariable::builder()
      .name("accentContent")
      .variable("--ac")
      .value("#163835")
      .description("Color for content within accents.")
      .syntax(Color)
      .build(),
    CssVariable::builder()
      .name("neutral")
      .variable("--n")
      .value("#3d4451")
      .description("The neutral color.")
      .syntax(Color)
      .build(),
    CssVariable::builder()
      .name("neutralContent")
      .variable("--nc")
      .value("#ffffff")
      .description("The neutral content color.")
      .syntax(Color)
      .build(),
    CssVariable::builder()
      .name("base100")
      .variable("--b1")
      .value("#ffffff")
      .description("The base color.")
      .syntax(Color)
      .build(),
    CssVariable::builder()
      .name("base200")
      .variable("--b2")
      .value("#F2F2F2")
      .description("The secondary base color.")
      .syntax(Color)
      .build(),
    CssVariable::builder()
      .name("base300")
      .variable("--b3")
      .value("#E5E6E6")
      .description("The tertiary base color.")
      .syntax(Color)
      .build(),
    CssVariable::builder()
      .name("baseContent")
      .variable("--bc")
      .value("#1f2937")
      .description("The base content color. This is useful for text.")
      .syntax(Color)
      .build(),
    CssVariable::builder()
      .name("info")
      .variable("--in")
      .value("#0070F3")
      .description("The info color. Useful for info buttons and alerts.")
      .syntax(Color)
      .build(),
    CssVariable::builder()
      .name("infoContent")
      .variable("--inc")
      .value("#ffffff")
      .description("The info content color. Useful for text within info buttons and alerts.")
      .syntax(Color)
      .build(),
    CssVariable::builder()
      .name("success")
      .variable("--su")
      .value("#21CC51")
      .description("The success color. Useful for success buttons and alerts.")
      .syntax(Color)
      .build(),
    CssVariable::builder()
      .name("successContent")
      .variable("--suc")
      .value("#ffffff")
      .description("The success content color. Useful for text within success buttons and alerts.")
      .syntax(Color)
      .build(),
    CssVariable::builder()
      .name("warning")
      .variable("--wa")
      .value("#FF6154")
      .description("The warning color. Useful for warning buttons and alerts.")
      .syntax(Color)
      .build(),
    CssVariable::builder()
      .name("warningContent")
      .variable("--wac")
      .value("#ffffff")
      .description("The warning content color. Useful for text within warning buttons and alerts.")
      .syntax(Color)
      .build(),
    CssVariable::builder()
      .name("error")
      .variable("--er")
      .value("#DE1C8D")
      .description("The error color. Useful for error buttons and alerts.")
      .syntax(Color)
      .build(),
    CssVariable::builder()
      .name("errorContent")
      .variable("--erc")
      .value("#ffffff")
      .description("The error content color. Useful for text within error buttons and alerts.")
      .syntax(Color)
      .build(),
  ];
  pub(crate) static ref CSS_VARIABLES: Vec<CssVariable> = vec![
    CssVariable::builder()
      .name("containerMaxWidth")
      .variable("--container-max-width")
      .value("container",)
      .build(),
    CssVariable::builder()
      .name("empty")
      .variable("--empty")
      .value("/* */",)
      .build(),
    CssVariable::builder()
      .name("filterBlur")
      .variable("--filter-blur")
      .build(),
    CssVariable::builder()
      .name("filterBrightness")
      .variable("--filter-brightness")
      .build(),
    CssVariable::builder()
      .name("filterContrast")
      .variable("--filter-contrast")
      .build(),
    CssVariable::builder()
      .name("filterCustom")
      .variable("--filter-custom")
      .build(),
    CssVariable::builder()
      .name("filterGrayscale")
      .variable("--filter-grayscale")
      .build(),
    CssVariable::builder()
      .name("filterHueRotate")
      .variable("--filter-hue-rotate")
      .build(),
    CssVariable::builder()
      .name("filterInvert")
      .variable("--filter-invert")
      .build(),
    CssVariable::builder()
      .name("filterSaturate")
      .variable("--filter-saturate")
      .build(),
    CssVariable::builder()
      .name("filterSepia")
      .variable("--filter-sepia")
      .build(),
    CssVariable::builder()
      .name("filterDropShadow")
      .variable("--filter-drop-shadow")
      .build(),
    CssVariable::builder()
      .name("groupNestedFilter")
      .variable("--group-nested-filter")
      .value({
        let filter_blur = Placeholder::variable("filterBlur");
        let filter_brightness = Placeholder::variable("filterBrightness");
        let filter_contrast = Placeholder::variable("filterContrast");
        let filter_grayscale = Placeholder::variable("filterGrayscale");
        let filter_hue_rotate = Placeholder::variable("filterHueRotate");
        let filter_invert = Placeholder::variable("filterInvert");
        let filter_saturate = Placeholder::variable("filterSaturate");
        let filter_sepia = Placeholder::variable("filterSepia");
        let filter_drop_shadow = Placeholder::variable("filterDropShadow");
        let filter_custom = Placeholder::variable("filterCustom");
        format!(
          "var({filter_blur}) var({filter_brightness}) var({filter_contrast}) \
           var({filter_grayscale}) var({filter_hue_rotate}) var({filter_invert}) \
           var({filter_saturate}) var({filter_sepia}) var({filter_drop_shadow}) \
           var({filter_custom})",
        )
      })
      .build(),
    CssVariable::builder()
      .name("backdropBlur")
      .variable("--backdrop-blur")
      .build(),
    CssVariable::builder()
      .name("backdropBrightness")
      .variable("--backdrop-brightness")
      .build(),
    CssVariable::builder()
      .name("backdropContrast")
      .variable("--backdrop-contrast")
      .build(),
    CssVariable::builder()
      .name("backdropCustom")
      .variable("--backdrop-custom")
      .build(),
    CssVariable::builder()
      .name("backdropGrayscale")
      .variable("--backdrop-grayscale")
      .build(),
    CssVariable::builder()
      .name("backdropHueRotate")
      .variable("--backdrop-hue-rotate")
      .build(),
    CssVariable::builder()
      .name("backdropInvert")
      .variable("--backdrop-invert")
      .build(),
    CssVariable::builder()
      .name("backdropSaturate")
      .variable("--backdrop-saturate")
      .build(),
    CssVariable::builder()
      .name("backdropSepia")
      .variable("--backdrop-sepia")
      .build(),
    CssVariable::builder()
      .name("backdropDropShadow")
      .variable("--backdrop-drop-shadow")
      .build(),
    CssVariable::builder()
      .name("groupNestedBackdrop")
      .variable("--group-nested-backdrop")
      .value({
        let backdrop_blur = Placeholder::variable("backdropBlur");
        let backdrop_brightness = Placeholder::variable("backdropBrightness");
        let backdrop_contrast = Placeholder::variable("backdropContrast");
        let backdrop_grayscale = Placeholder::variable("backdropGrayscale");
        let backdrop_hue_rotate = Placeholder::variable("backdropHueRotate");
        let backdrop_invert = Placeholder::variable("backdropInvert");
        let backdrop_saturate = Placeholder::variable("backdropSaturate");
        let backdrop_sepia = Placeholder::variable("backdropSepia");
        let backdrop_drop_shadow = Placeholder::variable("backdropDropShadow");
        let backdrop_custom = Placeholder::variable("backdropCustom");
        format!(
          "var({backdrop_blur}) var({backdrop_brightness}) var({backdrop_contrast}) \
           var({backdrop_grayscale}) var({backdrop_hue_rotate}) var({backdrop_invert}) \
           var({backdrop_saturate}) var({backdrop_sepia}) var({backdrop_drop_shadow}) \
           var({backdrop_custom})",
        )
      })
      .build(),
    CssVariable::builder()
      .name("defaultTransitionDuration")
      .variable("--default-transition-duration")
      .value("150ms",)
      .build(),
    CssVariable::builder()
      .name("defaultAnimationDuration")
      .variable("--default-animation-duration")
      .value("1s")
      .build(),
    CssVariable::builder()
      .name("enterTranslateX")
      .variable("--etx")
      .build(),
    CssVariable::builder()
      .name("enterTranslateY")
      .variable("--ety")
      .build(),
  ];
}
