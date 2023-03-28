use indexmap::indexmap;

use crate::AnyEmptyResult;
use crate::Atom;
use crate::ClassFactory;
use crate::SkribbleRunner;
use crate::StyleConfig;
use crate::ToSkribbleCss;
use crate::ValueSet;

#[test]
fn class_selector() -> AnyEmptyResult {
  let mut runner = SkribbleRunner::new(create_config());
  let runner_config = runner.initialize()?;
  let mut factory = ClassFactory::new(runner_config);
  factory.add_token("pt").add_token("0");
  let class = factory.into_class().unwrap();
  insta::assert_display_snapshot!(class.to_skribble_css(&runner_config)?, @r###"
  .pt\:\$0 {
    padding-top: 0px;
  }
  "###);

  Ok(())
}

fn create_config() -> StyleConfig {
  StyleConfig::builder()
    .atoms(vec![
      Atom::builder()
        .name("pt")
        .values(vec!["spacing"])
        .styles(indexmap! { "padding-top" => None as Option<String> })
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
    .plugins(vec![])
    .build()
}
