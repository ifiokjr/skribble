use lazy_static::lazy_static;
use skribble_core::Alias;

lazy_static! {
  pub(crate) static ref ALIASES: Vec<Alias> = vec![
    Alias::builder()
      .name("block")
      .classes("display:$block")
      .build(),
    Alias::builder()
      .name("inline-block")
      .classes("display:$inline-block")
      .build(),
    Alias::builder()
      .name("inline")
      .classes("display:$inline")
      .build(),
    Alias::builder()
      .name("flex")
      .classes("display:$flex")
      .build(),
    Alias::builder()
      .name("inline-flex")
      .classes("display:$inline-flex")
      .build(),
    Alias::builder()
      .name("table")
      .classes("display:$table")
      .build(),
    Alias::builder()
      .name("inline-table")
      .classes("display:$inline-table")
      .build(),
    Alias::builder()
      .name("table-caption")
      .classes("display:$table-caption")
      .build(),
    Alias::builder()
      .name("table-cell")
      .classes("display:$table-cell")
      .build(),
    Alias::builder()
      .name("table-column")
      .classes("display:$table-column")
      .build(),
    Alias::builder()
      .name("table-column-group")
      .classes("display:$table-column-group")
      .build(),
    Alias::builder()
      .name("table-footer-group")
      .classes("display:$table-footer-group")
      .build(),
    Alias::builder()
      .name("table-header-group")
      .classes("display:$table-header-group")
      .build(),
    Alias::builder()
      .name("table-row-group")
      .classes("display:$table-row-group")
      .build(),
    Alias::builder()
      .name("table-row")
      .classes("display:$table-row")
      .build(),
    Alias::builder()
      .name("flow-root")
      .classes("display:$flow-root")
      .build(),
    Alias::builder()
      .name("grid")
      .classes("display:$grid")
      .build(),
    Alias::builder()
      .name("inline-grid")
      .classes("display:$inline-grid")
      .build(),
    Alias::builder()
      .name("contents")
      .classes("display:$contents")
      .build(),
    Alias::builder()
      .name("list-item")
      .classes("display:$list-item")
      .build(),
    Alias::builder()
      .name("hidden")
      .classes("display:$none")
      .build(),
  ];
}
