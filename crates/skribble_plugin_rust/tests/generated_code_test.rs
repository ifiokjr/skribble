pub mod generated_code;
use generated_code::*;
use insta::assert_display_snapshot;

#[test]
fn media_query_class_names() {
  assert_display_snapshot!(sk().md().p().px(), @"md:p:$px");
}

#[test]
fn parent_modifier_class_names() {
  assert_display_snapshot!(sk().dark().p().px(), @"dark:p:$px");
}

#[test]
fn atom_with_colors() {
  assert_display_snapshot!(sk().bg().accent(), @"bg:$accent");
}

#[test]
fn atom_with_palette() {
  assert_display_snapshot!(sk().bg().red100(), @"bg:$red100");
}

#[test]
fn variables() {
  assert_display_snapshot!(vars().primary(), @"--sk-p");
}
