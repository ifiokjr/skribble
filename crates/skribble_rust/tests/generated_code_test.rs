mod generated_code;
use generated_code::*;
use rstest::rstest;
use similar_asserts::assert_eq;

#[rstest]
#[case(sk().md().p().px(), "md:p:$px")]
#[case(sk().dark().p().px(), "dark:p:$px")]
#[case(sk().bg().accent(), "bg:$accent")]
#[case(sk().md().pt_("1px"), "md:pt:[1px]")]
#[case(sk().md_("padding", "1px"), "md:[padding=1px]")]
#[case(sk().screen().md_("padding", "1px"), "screen:md:[padding=1px]")]
#[case(sk().p_("101px"), "p:[101px]")]
#[case(sk().bg().red100(), "bg:$red100")]
#[case(vars().primary(), "--sk-p")]
fn generated_class_names(#[case] input: String, #[case] expected: &str) {
  assert_eq!(input, expected);
}
