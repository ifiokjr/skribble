mod generated_code;
use generated_code::*;
use rstest::rstest;
use similar_asserts::assert_eq;

#[rstest]
#[case::breakpoint_padding(sk().md().p().px(), "md:p:$px")]
#[case::media_query_padding(sk().dark().p().px(), "dark:p:$px")]
#[case::background_color(sk().bg().accent(), "bg:$accent")]
#[case::breakpoint_padding_argument(sk().md().pt_("1px"), "md:pt:[1px]")]
#[case::breakpoint_key_value_argument(sk().md_("padding", "1px"), "md:[padding=1px]")]
#[case::chained_media_query_key_value_argument(sk().screen().md_("padding", "1px"), "screen:md:[padding=1px]")]
#[case::padding_argument(sk().p_("101px"), "p:[101px]")]
#[case::background_pallete(sk().bg().red100(), "bg:$red100")]
#[case::aspect_ratio(sk().aspect().square(), "aspect:$square")]
#[case::columns(sk().columns().n4(), "columns:$4")]
#[case::aliases(sk().md().block(), "md:display:$block")]
#[case::aliases(sk().hover().inline_block(), "hover:display:$inline-block")]
#[case::variables(vars().primary(), "--sk-p")]
fn generated_class_names(#[case] input: String, #[case] expected: &str) {
  assert_eq!(input, expected);
}
