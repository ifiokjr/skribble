use indexmap::indexmap;
use lazy_static::lazy_static;
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
      .name("primary-content")
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
      .name("secondary-content")
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
      .name("accent-content")
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
      .name("neutral-content")
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
      .name("base-content")
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
      .name("info-content")
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
      .name("success-content")
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
      .name("warning-content")
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
      .name("error-content")
      .variable("--erc")
      .value(Placeholder::palette("white"))
      .description("The error content color. Useful for text within error buttons and alerts.")
      .syntax(Color)
      .build(),
  ];
  pub(crate) static ref CSS_VARIABLES: Vec<CssVariable> = vec![
    CssVariable::builder()
      .name("contained-max-width")
      .variable("--cmw")
      .value("inherit")
      .media_queries(indexmap! {
        Placeholder::media_query("sm") => indexmap! { "" => "640px" },
        Placeholder::media_query("md") => indexmap! { "" => "768px" },
        Placeholder::media_query("lg") => indexmap! { "" => "1024px" },
        Placeholder::media_query("xl") => indexmap! { "" => "1280px" },
        Placeholder::media_query("xxl") => indexmap! { "" => "1536px" },
      })
      .build(),
    CssVariable::builder()
      .name("empty")
      .variable("--empty")
      .value("/* */",)
      .build(),
    CssVariable::builder()
      .name("filter-blur")
      .variable("--filter-blur")
      .build(),
    CssVariable::builder()
      .name("filter-brightness")
      .variable("--filter-brightness")
      .build(),
    CssVariable::builder()
      .name("filter-contrast")
      .variable("--filter-contrast")
      .build(),
    CssVariable::builder()
      .name("filter-custom")
      .variable("--filter-custom")
      .build(),
    CssVariable::builder()
      .name("filter-grayscale")
      .variable("--filter-grayscale")
      .build(),
    CssVariable::builder()
      .name("filter-hue-rotate")
      .variable("--filter-hue-rotate")
      .build(),
    CssVariable::builder()
      .name("filter-invert")
      .variable("--filter-invert")
      .build(),
    CssVariable::builder()
      .name("filter-saturate")
      .variable("--filter-saturate")
      .build(),
    CssVariable::builder()
      .name("filter-sepia")
      .variable("--filter-sepia")
      .build(),
    CssVariable::builder()
      .name("filter-drop-shadow")
      .variable("--filter-drop-shadow")
      .build(),
    CssVariable::builder()
      .name("backdrop-blur")
      .variable("--backdrop-blur")
      .build(),
    CssVariable::builder()
      .name("backdrop-brightness")
      .variable("--backdrop-brightness")
      .build(),
    CssVariable::builder()
      .name("backdrop-contrast")
      .variable("--backdrop-contrast")
      .build(),
    CssVariable::builder()
      .name("backdrop-custom")
      .variable("--backdrop-custom")
      .build(),
    CssVariable::builder()
      .name("backdrop-grayscale")
      .variable("--backdrop-grayscale")
      .build(),
    CssVariable::builder()
      .name("backdrop-hue-rotate")
      .variable("--backdrop-hue-rotate")
      .build(),
    CssVariable::builder()
      .name("backdrop-invert")
      .variable("--backdrop-invert")
      .build(),
    CssVariable::builder()
      .name("backdrop-saturate")
      .variable("--backdrop-saturate")
      .build(),
    CssVariable::builder()
      .name("backdrop-sepia")
      .variable("--backdrop-sepia")
      .build(),
    CssVariable::builder()
      .name("backdrop-drop-shadow")
      .variable("--backdrop-drop-shadow")
      .build(),
    CssVariable::builder()
      .name("transition-duration")
      .variable("--transition-duration")
      .syntax(PropertySyntaxValue::Time)
      .value("150ms")
      .build(),
    CssVariable::builder()
      .name("animation-duration")
      .variable("--animation-duration")
      .syntax(PropertySyntaxValue::Time)
      .value("1s")
      .build(),
    CssVariable::builder()
      .name("enter-opacity")
      .variable("--enter-opacity")
      .syntax(vec![
        PropertySyntaxValue::Number,
        PropertySyntaxValue::Percentage
      ])
      .value("1")
      .build(),
    CssVariable::builder()
      .name("enter-translate-x")
      .variable("--enter-tx")
      .syntax(PropertySyntaxValue::LengthPercentage)
      .value("0")
      .build(),
    CssVariable::builder()
      .name("enter-translate-y")
      .variable("--enter-ty")
      .syntax(PropertySyntaxValue::LengthPercentage)
      .value("0")
      .build(),
    CssVariable::builder()
      .name("enter-scale-x")
      .variable("--enter-sx")
      .syntax(vec![
        PropertySyntaxValue::Number,
        PropertySyntaxValue::Percentage
      ])
      .value("1")
      .build(),
    CssVariable::builder()
      .name("enter-scale-y")
      .variable("--enter-sy")
      .syntax(vec![
        PropertySyntaxValue::Number,
        PropertySyntaxValue::Percentage
      ])
      .value("1")
      .build(),
    CssVariable::builder()
      .name("enter-rotate")
      .variable("--enter-rotate")
      .syntax(PropertySyntaxValue::Angle)
      .value("0")
      .build(),
    CssVariable::builder()
      .name("exit-opacity")
      .variable("--exit-opacity")
      .syntax(vec![
        PropertySyntaxValue::Number,
        PropertySyntaxValue::Percentage
      ])
      .value("1")
      .build(),
    CssVariable::builder()
      .name("exit-translate-x")
      .variable("--exit-tx")
      .syntax(PropertySyntaxValue::LengthPercentage)
      .value("0")
      .build(),
    CssVariable::builder()
      .name("exit-translate-y")
      .variable("--exit-ty")
      .syntax(PropertySyntaxValue::LengthPercentage)
      .value("0")
      .build(),
    CssVariable::builder()
      .name("exit-scale-x")
      .variable("--exit-sx")
      .syntax(vec![
        PropertySyntaxValue::Number,
        PropertySyntaxValue::Percentage
      ])
      .value("1")
      .build(),
    CssVariable::builder()
      .name("exit-scale-y")
      .variable("--exit-sy")
      .syntax(vec![
        PropertySyntaxValue::Number,
        PropertySyntaxValue::Percentage
      ])
      .value("1")
      .build(),
    CssVariable::builder()
      .name("exit-rotate")
      .variable("--exit-rotate")
      .syntax(PropertySyntaxValue::Angle)
      .value("0")
      .build(),
    CssVariable::builder()
      .name("space-x-reverse")
      .variable("--space-x-reverse")
      .syntax(PropertySyntaxValue::Number)
      .value("0")
      .build(),
    CssVariable::builder()
      .name("space-y-reverse")
      .variable("--space-y-reverse")
      .syntax(PropertySyntaxValue::Number)
      .value("0")
      .build(),
    CssVariable::builder()
      .name("gradient-stops")
      .variable("--gradient-stops")
      .build(),
    CssVariable::builder()
      .name("gradient-from")
      .variable("--gradient-from")
      .build(),
    CssVariable::builder()
      .name("gradient-to")
      .variable("--gradient-to")
      .build(),
    CssVariable::builder()
      .name("gradient-from-position")
      .variable("--gradient-from-position")
      .build(),
    CssVariable::builder()
      .name("gradient-to-position")
      .variable("--gradient-to-position")
      .build(),
    CssVariable::builder()
      .name("gradient-via-position")
      .variable("--gradient-via-position")
      .build(),
    CssVariable::builder()
      .name("scale-x")
      .variable("--scale-x")
      .syntax(vec![
        PropertySyntaxValue::Number,
        PropertySyntaxValue::Percentage
      ])
      .value("1")
      .build(),
    CssVariable::builder()
      .name("scale-y")
      .variable("--scale-y")
      .syntax(vec![
        PropertySyntaxValue::Number,
        PropertySyntaxValue::Percentage
      ])
      .value("1")
      .build(),
    CssVariable::builder()
      .name("translate-x")
      .variable("--translate-x")
      .syntax(PropertySyntaxValue::LengthPercentage)
      .value("0")
      .build(),
    CssVariable::builder()
      .name("translate-y")
      .variable("--translate-y")
      .syntax(PropertySyntaxValue::LengthPercentage)
      .value("0")
      .build(),
    CssVariable::builder()
      .name("skew-x")
      .variable("--skew-x")
      .syntax(PropertySyntaxValue::Angle)
      .value("0")
      .build(),
    CssVariable::builder()
      .name("skew-y")
      .variable("--skew-y")
      .syntax(PropertySyntaxValue::Angle)
      .value("0")
      .build(),
    CssVariable::builder()
      .name("rotate")
      .variable("--rotate")
      .syntax(PropertySyntaxValue::Angle)
      .value("0")
      .build(),
    CssVariable::builder()
      .name("ring-offset-shadow")
      .variable("--ring-offset-shadow")
      .build(),
    CssVariable::builder()
      .name("ring-offset-color")
      .variable("--ring-offset-color")
      .syntax(Color)
      .value(Placeholder::palette("white"))
      .build(),
    CssVariable::builder()
      .name("ring-color")
      .variable("--ring-color")
      .syntax(Color)
      .value("#3b82f680")
      .build(),
    CssVariable::builder()
      .name("ring-inset")
      .variable("--ring-inset")
      .build(),
    CssVariable::builder()
      .name("ring-offset-width")
      .variable("--ring-offset-width")
      .build(),
    CssVariable::builder()
      .name("ring-shadow")
      .variable("--ring-shadow")
      .build(),
    CssVariable::builder()
      .name("shadow")
      .variable("--shadow")
      .build(),
  ];
}
