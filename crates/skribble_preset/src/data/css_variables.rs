use lazy_static::lazy_static;
use skribble_core::map;
use skribble_core::CssVariable;
use skribble_core::Placeholder;
use skribble_core::PropertySyntaxValue;
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
      .value(Placeholder::palette("white"))
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
      .value(Placeholder::palette("white"))
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
      .value(Placeholder::palette("white"))
      .description("The neutral content color.")
      .syntax(Color)
      .build(),
    CssVariable::builder()
      .name("base100")
      .variable("--b1")
      .value(Placeholder::palette("white"))
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
      .value(Placeholder::palette("white"))
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
      .value(Placeholder::palette("white"))
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
      .value(Placeholder::palette("white"))
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
      .value(Placeholder::palette("white"))
      .description("The error content color. Useful for text within error buttons and alerts.")
      .syntax(Color)
      .build(),
  ];
  pub(crate) static ref CSS_VARIABLES: Vec<CssVariable> = vec![
    CssVariable::builder()
      .name("containedMaxWidth")
      .variable("--cmw")
      .value("inherit")
      .media_queries(map! {
        Placeholder::media_query("sm") => map! { "" => "640px" },
        Placeholder::media_query("md") => map! { "" => "768px" },
        Placeholder::media_query("lg") => map! { "" => "1024px" },
        Placeholder::media_query("xl") => map! { "" => "1280px" },
        Placeholder::media_query("xxl") => map! { "" => "1536px" },
      })
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
        let filter_blur = Placeholder::wrapped_variable("filterBlur", None);
        let filter_brightness = Placeholder::wrapped_variable("filterBrightness", None);
        let filter_contrast = Placeholder::wrapped_variable("filterContrast", None);
        let filter_grayscale = Placeholder::wrapped_variable("filterGrayscale", None);
        let filter_hue_rotate = Placeholder::wrapped_variable("filterHueRotate", None);
        let filter_invert = Placeholder::wrapped_variable("filterInvert", None);
        let filter_saturate = Placeholder::wrapped_variable("filterSaturate", None);
        let filter_sepia = Placeholder::wrapped_variable("filterSepia", None);
        let filter_drop_shadow = Placeholder::wrapped_variable("filterDropShadow", None);
        let filter_custom = Placeholder::wrapped_variable("filterCustom", None);
        format!(
          "{filter_blur} {filter_brightness} {filter_contrast} {filter_grayscale} \
           {filter_hue_rotate} {filter_invert} {filter_saturate} {filter_sepia} \
           {filter_drop_shadow} {filter_custom}",
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
      .variable("--gnb")
      .value({
        let backdrop_blur = Placeholder::wrapped_variable("backdropBlur", None);
        let backdrop_brightness = Placeholder::wrapped_variable("backdropBrightness", None);
        let backdrop_contrast = Placeholder::wrapped_variable("backdropContrast", None);
        let backdrop_grayscale = Placeholder::wrapped_variable("backdropGrayscale", None);
        let backdrop_hue_rotate = Placeholder::wrapped_variable("backdropHueRotate", None);
        let backdrop_invert = Placeholder::wrapped_variable("backdropInvert", None);
        let backdrop_saturate = Placeholder::wrapped_variable("backdropSaturate", None);
        let backdrop_sepia = Placeholder::wrapped_variable("backdropSepia", None);
        let backdrop_drop_shadow = Placeholder::wrapped_variable("backdropDropShadow", None);
        let backdrop_custom = Placeholder::wrapped_variable("backdropCustom", None);
        format!(
          "{backdrop_blur} {backdrop_brightness} {backdrop_contrast} {backdrop_grayscale} \
           {backdrop_hue_rotate} {backdrop_invert} {backdrop_saturate} {backdrop_sepia} \
           {backdrop_drop_shadow} {backdrop_custom}",
        )
      })
      .build(),
    CssVariable::builder()
      .name("transitionDuration")
      .variable("--transition-duration")
      .syntax(PropertySyntaxValue::Time)
      .value("150ms")
      .build(),
    CssVariable::builder()
      .name("animationDuration")
      .variable("--animation-duration")
      .syntax(PropertySyntaxValue::Time)
      .value("1s")
      .build(),
    CssVariable::builder()
      .name("enterOpacity")
      .variable("--enter-opacity")
      .syntax(vec![
        PropertySyntaxValue::Number,
        PropertySyntaxValue::Percentage
      ])
      .value("1")
      .build(),
    CssVariable::builder()
      .name("enterTranslateX")
      .variable("--enter-tx")
      .syntax(PropertySyntaxValue::LengthPercentage)
      .value("0")
      .build(),
    CssVariable::builder()
      .name("enterTranslateY")
      .variable("--enter-ty")
      .syntax(PropertySyntaxValue::LengthPercentage)
      .value("0")
      .build(),
    CssVariable::builder()
      .name("enterScaleX")
      .variable("--enter-sx")
      .syntax(vec![
        PropertySyntaxValue::Number,
        PropertySyntaxValue::Percentage
      ])
      .value("1")
      .build(),
    CssVariable::builder()
      .name("enterScaleY")
      .variable("--enter-sy")
      .syntax(vec![
        PropertySyntaxValue::Number,
        PropertySyntaxValue::Percentage
      ])
      .value("1")
      .build(),
    CssVariable::builder()
      .name("enterRotate")
      .variable("--enter-rotate")
      .syntax(PropertySyntaxValue::Angle)
      .value("0")
      .build(),
    CssVariable::builder()
      .name("exitOpacity")
      .variable("--exit-opacity")
      .syntax(vec![
        PropertySyntaxValue::Number,
        PropertySyntaxValue::Percentage
      ])
      .value("1")
      .build(),
    CssVariable::builder()
      .name("exitTranslateX")
      .variable("--exit-tx")
      .syntax(PropertySyntaxValue::LengthPercentage)
      .value("0")
      .build(),
    CssVariable::builder()
      .name("exitTranslateY")
      .variable("--exit-ty")
      .syntax(PropertySyntaxValue::LengthPercentage)
      .value("0")
      .build(),
    CssVariable::builder()
      .name("exitScaleX")
      .variable("--exit-sx")
      .syntax(vec![
        PropertySyntaxValue::Number,
        PropertySyntaxValue::Percentage
      ])
      .value("1")
      .build(),
    CssVariable::builder()
      .name("exitScaleY")
      .variable("--exit-sy")
      .syntax(vec![
        PropertySyntaxValue::Number,
        PropertySyntaxValue::Percentage
      ])
      .value("1")
      .build(),
    CssVariable::builder()
      .name("exitRotate")
      .variable("--exit-rotate")
      .syntax(PropertySyntaxValue::Angle)
      .value("0")
      .build(),
  ];
}
