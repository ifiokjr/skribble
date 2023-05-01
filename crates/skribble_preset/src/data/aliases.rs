use lazy_static::lazy_static;
use skribble_core::Alias;

lazy_static! {
  pub(crate) static ref ALIASES: Vec<Alias> = vec![
    Alias::builder()
      .name("block")
      .classes(vec!["display:$block"])
      .build(),
    Alias::builder()
      .name("inline-block")
      .classes(vec!["display:$inline-block"])
      .build(),
    Alias::builder()
      .name("inline")
      .classes(vec!["display:$inline"])
      .build(),
    Alias::builder()
      .name("flex")
      .classes(vec!["display:$flex"])
      .build(),
    Alias::builder()
      .name("inline-flex")
      .classes(vec!["display:$inline-flex"])
      .build(),
    Alias::builder()
      .name("table")
      .classes(vec!["display:$table"])
      .build(),
    Alias::builder()
      .name("inline-table")
      .classes(vec!["display:$inline-table"])
      .build(),
    Alias::builder()
      .name("table-caption")
      .classes(vec!["display:$table-caption"])
      .build(),
    Alias::builder()
      .name("table-cell")
      .classes(vec!["display:$table-cell"])
      .build(),
    Alias::builder()
      .name("table-column")
      .classes(vec!["display:$table-column"])
      .build(),
    Alias::builder()
      .name("table-column-group")
      .classes(vec!["display:$table-column-group"])
      .build(),
    Alias::builder()
      .name("table-footer-group")
      .classes(vec!["display:$table-footer-group"])
      .build(),
    Alias::builder()
      .name("table-header-group")
      .classes(vec!["display:$table-header-group"])
      .build(),
    Alias::builder()
      .name("table-row-group")
      .classes(vec!["display:$table-row-group"])
      .build(),
    Alias::builder()
      .name("table-row")
      .classes(vec!["display:$table-row"])
      .build(),
    Alias::builder()
      .name("flow-root")
      .classes(vec!["display:$flow-root"])
      .build(),
    Alias::builder()
      .name("grid")
      .classes(vec!["display:$grid"])
      .build(),
    Alias::builder()
      .name("inline-grid")
      .classes(vec!["display:$inline-grid"])
      .build(),
    Alias::builder()
      .name("contents")
      .classes(vec!["display:$contents"])
      .build(),
    Alias::builder()
      .name("list-item")
      .classes(vec!["display:$list-item"])
      .build(),
    Alias::builder()
      .name("hidden")
      .classes(vec!["display:$none"])
      .build(),
    Alias::builder()
      .name("visible")
      .classes(vec!["visibility:$visible"])
      .build(),
    Alias::builder()
      .name("invisible")
      .classes(vec!["visibility:$hidden"])
      .build(),
    Alias::builder()
      .name("collapse")
      .classes(vec!["visibility:$collapse"])
      .build(),
    Alias::builder()
      .name("isolate")
      .classes(vec!["isolate:$isolate"])
      .build(),
    Alias::builder()
      .name("static")
      .classes(vec!["position:$static"])
      .build(),
    Alias::builder()
      .name("fixed")
      .classes(vec!["position:$fixed"])
      .build(),
    Alias::builder()
      .name("absolute")
      .classes(vec!["position:$absolute"])
      .build(),
    Alias::builder()
      .name("relative")
      .classes(vec!["position:$relative"])
      .build(),
    Alias::builder()
      .name("sticky")
      .classes(vec!["position:$sticky"])
      .build(),
    Alias::builder()
      .name("blur")
      .classes(vec!["blur:$default"])
      .build(),
    Alias::builder()
      .name("drop-shadow")
      .classes(vec!["drop-shadow:$default"])
      .build(),
    Alias::builder()
      .name("grayscale")
      .classes(vec!["grayscale:$default"])
      .build(),
    Alias::builder()
      .name("invert")
      .classes(vec!["invert:$default"])
      .build(),
    Alias::builder()
      .name("sepia")
      .classes(vec!["sepia:$default"])
      .build(),
    Alias::builder()
      .name("filter-none")
      .classes(vec!["filter:$none"])
      .build(),
    Alias::builder()
      .name("backdrop-blur")
      .classes(vec!["backdrop-blur:$default"])
      .build(),
    Alias::builder()
      .name("backdrop-drop-shadow")
      .classes(vec!["backdrop-drop-shadow:$default"])
      .build(),
    Alias::builder()
      .name("backdrop-grayscale")
      .classes(vec!["backdrop-grayscale:$default"])
      .build(),
    Alias::builder()
      .name("backdrop-invert")
      .classes(vec!["backdrop-invert:$default"])
      .build(),
    Alias::builder()
      .name("backdrop-sepia")
      .classes(vec!["backdrop-sepia:$default"])
      .build(),
    Alias::builder()
      .name("backdrop-filter-none")
      .classes(vec!["backdrop-filter:$none"])
      .build(),
    Alias::builder()
      .name("flex-row")
      .classes(vec!["flex-direction:$row"])
      .build(),
    Alias::builder()
      .name("flex-row-reverse")
      .classes(vec!["flex-direction:$row-reverse"])
      .build(),
    Alias::builder()
      .name("flex-col")
      .classes(vec!["flex-direction:$col"])
      .build(),
    Alias::builder()
      .name("flex-col-reverse")
      .classes(vec!["flex-direction:$col-reverse"])
      .build(),
    Alias::builder()
      .name("flex-wrap")
      .classes(vec!["flex-direction:$wrap"])
      .build(),
    Alias::builder()
      .name("flex-wrap-reverse")
      .classes(vec!["flex-direction:$wrap-reverse"])
      .build(),
    Alias::builder()
      .name("flex-nowrap")
      .classes(vec!["flex-direction:$nowrap"])
      .build(),
    Alias::builder()
      .name("grow")
      .classes(vec!["flex-grow:$1"])
      .build(),
    Alias::builder()
      .name("grow-0")
      .classes(vec!["flex-grow:$0"])
      .build(),
    Alias::builder()
      .name("shrink")
      .classes(vec!["flex-shrink:$1"])
      .build(),
    Alias::builder()
      .name("shrink-0")
      .classes(vec!["flex-shrink:$0"])
      .build(),
    Alias::builder()
      .name("font-sans")
      .classes(vec!["font-family:$sans"])
      .build(),
    Alias::builder()
      .name("font-sans")
      .classes(vec!["font-family:$sans"])
      .build(),
    Alias::builder()
      .name("text-xs")
      .classes(vec!["font-size:$xs"])
      .build(),
    Alias::builder()
      .name("text-sm")
      .classes(vec!["font-size:$sm"])
      .build(),
    Alias::builder()
      .name("text-base")
      .classes(vec!["font-size:$base"])
      .build(),
    Alias::builder()
      .name("text-lg")
      .classes(vec!["font-size:$lg"])
      .build(),
    Alias::builder()
      .name("text-xl")
      .classes(vec!["font-size:$xl"])
      .build(),
    Alias::builder()
      .name("text-2xl")
      .classes(vec!["font-size:$2xl"])
      .build(),
    Alias::builder()
      .name("text-3xl")
      .classes(vec!["font-size:$3xl"])
      .build(),
    Alias::builder()
      .name("text-4xl")
      .classes(vec!["font-size:$4xl"])
      .build(),
    Alias::builder()
      .name("text-5xl")
      .classes(vec!["font-size:$5xl"])
      .build(),
    Alias::builder()
      .name("text-6xl")
      .classes(vec!["font-size:$6xl"])
      .build(),
    Alias::builder()
      .name("text-7xl")
      .classes(vec!["font-size:$7xl"])
      .build(),
    Alias::builder()
      .name("text-8xl")
      .classes(vec!["font-size:$8xl"])
      .build(),
    Alias::builder()
      .name("text-9xl")
      .classes(vec!["font-size:$9xl"])
      .build(),
    Alias::builder()
      .name("antialiased")
      .classes(vec!["smoothing:$antialiased"])
      .build(),
    Alias::builder()
      .name("subpixel-antialiased")
      .classes(vec!["smoothing:$subpixel"])
      .build(),
    Alias::builder()
      .name("italic")
      .classes(vec!["font-style:$italic"])
      .build(),
    Alias::builder()
      .name("non-italic")
      .classes(vec!["font-style:$normal"])
      .build(),
    Alias::builder()
      .name("font-thin")
      .classes(vec!["font-weight:$thin"])
      .build(),
    Alias::builder()
      .name("font-extralight")
      .classes(vec!["font-weight:$extralight"])
      .build(),
    Alias::builder()
      .name("font-light")
      .classes(vec!["font-weight:$light"])
      .build(),
    Alias::builder()
      .name("font-normal")
      .classes(vec!["font-weight:$normal"])
      .build(),
    Alias::builder()
      .name("font-medium")
      .classes(vec!["font-weight:$medium"])
      .build(),
    Alias::builder()
      .name("font-semibold")
      .classes(vec!["font-weight:$semibold"])
      .build(),
    Alias::builder()
      .name("font-bold")
      .classes(vec!["font-weight:$bold"])
      .build(),
    Alias::builder()
      .name("font-extrabold")
      .classes(vec!["font-weight:$extrabold"])
      .build(),
    Alias::builder()
      .name("font-black")
      .classes(vec!["font-weight:$black"])
      .build(),
    Alias::builder()
      .name("slashed-zero")
      .classes(vec!["font-numeric:$slashed"])
      .build(),
    Alias::builder()
      .name("lining-nums")
      .classes(vec!["font-numeric:$lining"])
      .build(),
    Alias::builder()
      .name("oldstyle-nums")
      .classes(vec!["font-numeric:$oldstyle"])
      .build(),
    Alias::builder()
      .name("proportional-nums")
      .classes(vec!["font-numeric:$proportional"])
      .build(),
    Alias::builder()
      .name("tabular-nums")
      .classes(vec!["font-numeric:$tabular"])
      .build(),
    Alias::builder()
      .name("diagonal-fractions")
      .classes(vec!["font-numeric:$diagonal"])
      .build(),
    Alias::builder()
      .name("stacked-fractions")
      .classes(vec!["font-numeric:$stacked"])
      .build(),
    Alias::builder()
      .name("list-inside")
      .classes(vec!["list-position:$inside"])
      .build(),
    Alias::builder()
      .name("list-outside")
      .classes(vec!["list-position:$outside"])
      .build(),
    Alias::builder()
      .name("text-left")
      .classes(vec!["text-align:$left"])
      .build(),
    Alias::builder()
      .name("text-center")
      .classes(vec!["text-align:$center"])
      .build(),
    Alias::builder()
      .name("text-right")
      .classes(vec!["text-align:$right"])
      .build(),
    Alias::builder()
      .name("text-justify")
      .classes(vec!["text-align:$justify"])
      .build(),
    Alias::builder()
      .name("text-start")
      .classes(vec!["text-align:$start"])
      .build(),
    Alias::builder()
      .name("text-end")
      .classes(vec!["text-align:$end"])
      .build(),
    Alias::builder()
      .name("underline")
      .classes(vec!["text-decoration:$underline"])
      .build(),
    Alias::builder()
      .name("overline")
      .classes(vec!["text-decoration:$overline"])
      .build(),
    Alias::builder()
      .name("line-through")
      .classes(vec!["text-decoration:$through"])
      .build(),
    Alias::builder()
      .name("strikethrough")
      .classes(vec!["text-decoration:$through"])
      .build(),
    Alias::builder()
      .name("no-underline")
      .classes(vec!["text-decoration:$none"])
      .build(),
    Alias::builder()
      .name("decoration-solid")
      .classes(vec!["decoration-style:$solid"])
      .build(),
    Alias::builder()
      .name("decoration-double")
      .classes(vec!["decoration-style:$double"])
      .build(),
    Alias::builder()
      .name("decoration-dotted")
      .classes(vec!["decoration-style:$dotted"])
      .build(),
    Alias::builder()
      .name("decoration-dashed")
      .classes(vec!["decoration-style:$dashed"])
      .build(),
    Alias::builder()
      .name("decoration-wavy")
      .classes(vec!["decoration-style:$wavy"])
      .build(),
    Alias::builder()
      .name("decoration-auto")
      .classes(vec!["decoration-thickness:$auto"])
      .build(),
    Alias::builder()
      .name("decoration-from-font")
      .classes(vec!["decoration-thickness:$from-font"])
      .build(),
    Alias::builder()
      .name("decoration-0")
      .classes(vec!["decoration-thickness:$0"])
      .build(),
    Alias::builder()
      .name("decoration-1")
      .classes(vec!["decoration-thickness:$1"])
      .build(),
    Alias::builder()
      .name("decoration-2")
      .classes(vec!["decoration-thickness:$2"])
      .build(),
    Alias::builder()
      .name("decoration-4")
      .classes(vec!["decoration-thickness:$4"])
      .build(),
    Alias::builder()
      .name("decoration-8")
      .classes(vec!["decoration-thickness:$8"])
      .build(),
    Alias::builder()
      .name("uppercase")
      .classes(vec!["text-transform:$uppercase"])
      .build(),
    Alias::builder()
      .name("lowercase")
      .classes(vec!["text-transform:$lowercase"])
      .build(),
    Alias::builder()
      .name("capitalize")
      .classes(vec!["text-transform:$capitalize"])
      .build(),
    Alias::builder()
      .name("text-ellipsis")
      .classes(vec!["text-overflow:$ellipsis"])
      .build(),
    Alias::builder()
      .name("text-clip")
      .classes(vec!["text-overflow:$clip"])
      .build(),
    Alias::builder()
      .name("truncate")
      .classes(vec![
        "visibility:$hidden",
        "text-overflow:$ellipsis",
        "whitespace:$nowrap"
      ])
      .build(),
  ];
}
