mod generated_code;
use generated_code::*;
use rstest::rstest;
use similar_asserts::assert_eq;

#[rstest]
#[case::custom(sk().__("height", "40px"), "[height=40px]")]
#[case::breakpoint_padding(sk().md().p().px(), "md:p:$px")]
#[case::media_query_padding(sk().dark().p().px(), "dark:p:$px")]
#[case::media_query_padding(sk().dark().p().n1(), "dark:p:$1")]
#[case::background_color(sk().bg().accent(), "bg:$accent")]
#[case::breakpoint_padding_argument(sk().md().pt_("1px"), "md:pt:[1px]")]
#[case::breakpoint_key_value_argument(sk().md_("padding", "1px"), "md:[padding=1px]")]
#[case::chained_media_query_key_value_argument(sk().screen().md_("padding", "1px"), "screen:md:[padding=1px]")]
#[case::padding_argument(sk().p_("101px"), "p:[101px]")]
#[case::background_palette(sk().bg().red100(), "bg:$red100")]
#[case::aspect_ratio(sk().aspect().square(), "aspect:$square")]
#[case::columns(sk().columns().n4(), "columns:$4")]
#[case::aliases(sk().md().block(), "md:display:$block")]
#[case::aliases(sk().hover().inline_block(), "hover:display:$inline-block")]
#[case::important_atom(sk().hover().important().inline_block(), "hover:(important):display:$inline-block")]
#[case::important_named_class(sk().important().sr_only(), "(important):$sr-only")]
#[case::transformer_color(sk().md().darken_050().bg().red100(), "md:(darken==050):bg:$red100")]
#[case::transformer_color_args(sk().md().alpha("50%").bg().red100(), "md:(alpha=50%):bg:$red100")]
#[case::gradients(
  &[sk().bg_gradient().to_right(), sk().from_color().cyan500(), sk().to_color().blue500()].join(" "),
  "bg-gradient:$to-right from-color:$cyan500 to-color:$blue500"
)]
#[case::variables(vars().primary(), "--sk-p")]
fn generated_class_names(#[case] input: impl AsRef<str>, #[case] expected: &str) {
	assert_eq!(input.as_ref(), expected);
}
