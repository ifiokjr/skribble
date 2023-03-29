use indexmap::indexmap;

use crate::AnyEmptyResult;
use crate::Atom;
use crate::ClassFactory;
use crate::Classes;
use crate::CssVariable;
use crate::Group;
use crate::Keyframe;
use crate::LinkedValues;
use crate::MediaQuery;
use crate::Placeholder;
use crate::PropertySyntaxValue;
use crate::SkribbleRunner;
use crate::StyleConfig;
use crate::ToSkribbleCss;
use crate::ValueSet;

#[test]
fn class_selector() -> AnyEmptyResult {
  let mut runner = SkribbleRunner::new(create_config());
  let runner_config = runner.initialize()?;
  let factory = ClassFactory::class(runner_config, &["pt", "0"]);
  let class = factory.into_class().unwrap();
  insta::assert_display_snapshot!(class.to_skribble_css(runner_config)?);

  Ok(())
}

#[test]
fn classes_css() -> AnyEmptyResult {
  let mut runner = SkribbleRunner::new(create_config());
  let runner_config = runner.initialize()?;
  let mut classes = Classes::default();
  classes.insert_factories(vec![
    ClassFactory::class(runner_config, &["pt", "0"]),
    ClassFactory::class(runner_config, &["sm", "pt", "10"]),
    ClassFactory::class(runner_config, &["md", "pt", "px"]),
    ClassFactory::class(runner_config, &["screen", "lg", "pt", "px"]),
  ]);
  classes.sort_by_class();
  insta::assert_display_snapshot!(classes.to_skribble_css(runner_config)?);
  Ok(())
}

#[test]
fn classes_with_color_properties() -> AnyEmptyResult {
  let mut runner = SkribbleRunner::new(create_config());
  let runner_config = runner.initialize()?;
  let mut classes = Classes::default();
  classes.insert_factories(vec![
    ClassFactory::class(runner_config, &["bg", "secondary"]),
    ClassFactory::class(runner_config, &["sm", "bg", "primary"]),
  ]);
  classes.sort_by_class();
  insta::assert_display_snapshot!(classes.to_skribble_css(runner_config)?);

  Ok(())
}

#[test]
fn classes_with_keyframes() -> AnyEmptyResult {
  let mut runner = SkribbleRunner::new(create_config());
  let runner_config = runner.initialize()?;
  let mut classes = Classes::default();
  classes.insert_factories(vec![
    ClassFactory::class(runner_config, &["animate", "spin"]),
    ClassFactory::class(runner_config, &["screen", "animate", "spin"]),
  ]);
  classes.sort_by_class();
  insta::assert_display_snapshot!(classes.to_skribble_css(runner_config)?);

  Ok(())
}

fn create_config() -> StyleConfig {
  StyleConfig::builder()
    .keyframes(vec![
      Keyframe::builder()
        .name("spin")
        .rules(indexmap! {
          "from" => indexmap! { "transform" => "rotate(0deg)" },
          "to" => indexmap! { "transform" => "rotate(360deg)" }
        })
        .build(),
    ])
    .atoms(vec![
      Atom::builder()
        .name("bg")
        .values(LinkedValues::Color)
        .styles(indexmap! { "color" => None as Option<String> })
        .build(),
      Atom::builder()
        .name("pt")
        .values(vec!["spacing"])
        .styles(indexmap! { "padding-top" => None as Option<String> })
        .build(),
      Atom::builder()
        .name("animate")
        .values(LinkedValues::Keyframes)
        .styles(indexmap! { "animation-name" => None as Option<String> })
        .build(),
    ])
    .media_queries(vec![
      Group::builder()
        .name("deviceCategories")
        .description("The device categories for the media query.")
        .items(vec![
          MediaQuery::builder()
            .name("screen")
            .query("screen")
            .description("The media query for devices with a screen.")
            .build(),
          MediaQuery::builder()
            .name("print")
            .query("print")
            .description("The media query for devices with a printer.")
            .build(),
        ])
        .build(),
      Group::builder()
        .name("breakpoints")
        .description("The breakpoints for the application.")
        .items(vec![
          MediaQuery::builder()
            .name("sm")
            .query("(min-width: 640px)")
            .description("The breakpoint for devices with screen size greater than tiny.")
            .build(),
          MediaQuery::builder()
            .name("md")
            .query("(min-width: 768px)")
            .description("The breakpoint for devices screen size greater than medium")
            .build(),
          MediaQuery::builder()
            .name("lg")
            .query("(min-width: 1024px)")
            .description("The breakpoint for devices screen size greater than large")
            .build(),
          MediaQuery::builder()
            .name("xl")
            .query("(min-width: 1280px)")
            .description("The breakpoint for devices screen size greater than extra large")
            .build(),
          MediaQuery::builder()
            .name("xxl")
            .query("(min-width: 1536px)")
            .description("The breakpoint for devices screen size greater than xxl")
            .build(),
        ])
        .build(),
      Group::builder()
        .name("darkMode")
        .description("The dark mode media query.")
        .items(vec![
          MediaQuery::builder()
            .name("dark")
            .query("(prefers-color-scheme: dark)")
            .description("The media query for devices with a dark color scheme.")
            .build(),
          MediaQuery::builder()
            .name("light")
            .query("(prefers-color-scheme: light)")
            .description("The media query for devices with a light color scheme.")
            .build(),
        ])
        .build(),
    ])
    .value_sets(vec![
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
    ])
    .variables(vec![
      CssVariable::builder()
        .name("primary")
        .variable("--p")
        .value("#570df8")
        .description("The primary color. Useful for primary buttons.")
        .syntax(PropertySyntaxValue::Color)
        .build(),
      CssVariable::builder()
        .name("secondary")
        .variable("--s")
        .value("#f000b8")
        .description("The secondary color. Useful for secondary buttons.")
        .syntax(PropertySyntaxValue::Color)
        .media_queries(indexmap! {
          Placeholder::media_query("print") => indexmap! { "" => "#0000b8", ".dark" => "#ff00ff" },
          Placeholder::media_query("dark") => indexmap! { "" => "#ffffee" },
        })
        .build(),
    ])
    .build()
}
