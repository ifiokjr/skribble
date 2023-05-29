use indexmap::indexmap;
use rstest::rstest;
use similar_asserts::assert_eq;
use skribble_test::set_snapshot_suffix;

use crate::Alias;
use crate::AnyEmptyResult;
use crate::Atom;
use crate::ClassFactory;
use crate::Classes;
use crate::CssVariable;
use crate::Group;
use crate::Keyframe;
use crate::LinkedValues;
use crate::MediaQuery;
use crate::Modifier;
use crate::NamedClass;
use crate::Placeholder;
use crate::PropertySyntaxValue;
use crate::SkribbleRunner;
use crate::StyleConfig;
use crate::ToSkribbleCss;
use crate::ValueSet;

#[rstest]
#[case("pt:$0", "pt:$1")]
#[case("sm:pt:$10", "md:pt:$10")]
#[case("bg:$primary", "bg:$secondary")]
fn class_order(#[case] a: &str, #[case] z: &str) -> AnyEmptyResult {
  let mut runner = SkribbleRunner::try_new(create_config())?;
  let config = runner.initialize()?;
  let a = ClassFactory::from_string(config, a).into_classes();
  let z = ClassFactory::from_string(config, z).into_classes();
  assert_eq!(a.len(), z.len());
  assert!(a < z);

  Ok(())
}

#[rstest]
#[case("normal", &["pt:$0"])]
#[case("alias", &["$yo"])]
#[case("media_query_alias", &["md:$yo"])]
#[case("mixed", &["pt:$0", "sm:pt:$10", "md:pt:$px", "screen:lg:pt:$px"])]
#[case("colors", &["bg:$secondary", "sm:bg:$primary"])]
#[case("keyframes", &["animate:$spin", "screen:animate:$spin"])]
#[case("atom_arguments", &["pt:[1px]", "md:pt:[1vh]"])]
#[case("references", &["needs-custom:$px", "needs-custom:$10", "$custom"])]
#[case("modifier_arguments", &["[padding=1px]", "md:[padding=1vh]", "hover:[--something=red]", "aria-hidden:[--something=red]"])]
fn css(#[case] id: &str, #[case] names: &[&str]) -> AnyEmptyResult {
  set_snapshot_suffix!("{id}");

  let mut runner = SkribbleRunner::try_new(create_config())?;
  let runner_config = runner.initialize()?;
  let mut classes = Classes::default();
  let mut factories = vec![];

  for name in names {
    factories.push(ClassFactory::from_string(runner_config, name));
  }

  classes.insert_factories(factories);
  let css = classes.to_skribble_css(runner_config)?;
  insta::assert_display_snapshot!(css);

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
    .aliases(vec![
      Alias::builder()
        .name("yo")
        .classes(vec!["pt:$0", "bg:$secondary"])
        .build(),
    ])
    .atoms(vec![
      Atom::builder()
        .name("needs-custom")
        .values(vec!["spacing"])
        .children(vec!["custom-reference"])
        .styles(indexmap! {
          "--some-variable" => Some(Placeholder::wrapped_variable("custom", None))
        })
        .build(),
      Atom::builder()
        .name("bg")
        .values(LinkedValues::Color(Default::default()))
        .styles(indexmap! { "background-color" => None as Option<String> })
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
    .modifiers(vec![
      Group::builder()
        .name("aria")
        .description("The aria attributes for the application.")
        .items(vec![
          Modifier::builder()
            .name("aria-hidden")
            .description("The aria-hidden attribute.")
            .values(vec!["&[aria-hidden=\"true\"]"])
            .build(),
        ])
        .build(),
      Group::builder()
        .name("other")
        .description("Other stuff.")
        .items(vec![
          Modifier::builder()
            .name("hover")
            .description("The aria-hidden attribute.")
            .values(vec!["&:hover"])
            .build(),
        ])
        .build(),
    ])
    .media_queries(vec![
      Group::builder()
        .name("device-categories")
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
        .name("dark-mode")
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
        .name("custom")
        .variable("--c")
        .value("inherit")
        .description("A custom variable")
        .build(),
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
    .classes(vec![
      NamedClass::builder()
        .name("custom-reference")
        .description("for testing reference types")
        .reference(true)
        .styles(indexmap! { Placeholder::variable("custom") => "" })
        .build(),
    ])
    .build()
}
