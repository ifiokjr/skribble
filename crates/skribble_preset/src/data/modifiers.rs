use lazy_static::lazy_static;
use skribble_core::Group;
use skribble_core::Modifier;

lazy_static! {
  pub(crate) static ref MODIFIERS: Vec<Group<Modifier>> = vec![
    Group::builder()
      .name("parent-group")
      .description("These are modifiers that are affect the class name based on parent selectors")
      .items(vec![
        Modifier::builder()
          .name("rtl")
          .values(vec!["[dir=rtl] &"])
          .description(
            "This class modifier becomes active when when the text direction of any parent nodes \
             is set to right to left."
          )
          .build(),
        Modifier::builder()
          .name("group-hover")
          .values(vec![
            ".\\$group:hover &",
            ".group:hover &",
            "[role='group']:hover &"
          ])
          .description("This class modifier becomes active when a parent group is hovered.")
          .build(),
        Modifier::builder()
          .name("group-focus")
          .values(vec![
            ".\\$group:focus &",
            ".group:focus &",
            "[role='group']:focus &"
          ])
          .description("This class modifier becomes active when a parent group is focused.")
          .build(),
        Modifier::builder()
          .name("group-active")
          .values(vec![
            ".\\$group:active &",
            ".group:active &",
            "[role='group']:active &"
          ])
          .description("This class modifier becomes active when a parent group is active.")
          .build(),
        Modifier::builder()
          .name("group-visited")
          .values(vec![
            ".\\$group:visited &",
            ".group:visited &",
            "[role='group']:visited &"
          ])
          .description("This class modifier becomes active when a parent group is visited.")
          .build(),
      ])
      .build(),
    Group::builder()
      .name("hover-group")
      .items(vec![
        Modifier::builder()
          .name("hover")
          .values(vec!["&:hover"])
          .description("This class modifier becomes active when the element is hovered.")
          .build()
      ])
      .build(),
    Group::builder()
      .name("active-group")
      .items(vec![
        Modifier::builder()
          .name("active")
          .values(vec!["&:active"])
          .build()
      ])
      .build(),
    Group::builder()
      .name("focus-group")
      .items(vec![
        Modifier::builder()
          .name("focus")
          .values(vec!["&:focus"])
          .build()
      ])
      .build(),
    Group::builder()
      .name("focus-within-group")
      .items(vec![
        Modifier::builder()
          .name("focus-within")
          .values(vec!["&:focus-within"])
          .build()
      ])
      .build(),
    Group::builder()
      .name("focus-visible-group")
      .items(vec![
        Modifier::builder()
          .name("focus-visible")
          .values(vec!["&:focus-visible"])
          .build()
      ])
      .build(),
    Group::builder()
      .name("enabled-group")
      .items(vec![
        Modifier::builder()
          .name("disabled")
          .values(vec!["&[disabled]", "&[aria-disabled=true]", "&:disabled"],)
          .build(),
        Modifier::builder()
          .name("not-disabled")
          .values(vec!["&[aria-disabled=false]", "&:disabled"],)
          .build(),
        Modifier::builder()
          .name("enabled")
          .values(vec!["&:enabled"])
          .build(),
      ])
      .build(),
    Group::builder()
      .name("empty-group")
      .items(vec![
        Modifier::builder()
          .name("empty")
          .values(vec!["&:empty"])
          .build()
      ])
      .build(),
    Group::builder()
      .name("read-group")
      .items(vec![
        Modifier::builder()
          .name("read-write")
          .values(vec!["&:read-write"],)
          .build(),
        Modifier::builder()
          .name("read-only")
          .values(vec!["&[aria-readonly=true]", "&[readonly]", "&:read-only"],)
          .build(),
        Modifier::builder()
          .name("not-read-only")
          .values(vec![
            "&[aria-readonly=false]",
            "&[readonly=false]",
            "&:not(:read-only)"
          ])
          .build(),
      ])
      .build(),
    Group::builder()
      .name("expanded-group")
      .items(vec![
        Modifier::builder()
          .name("expanded")
          .values(vec!["&[aria-expanded=true]"])
          .build()
      ])
      .build(),
    Group::builder()
      .name("checked-group")
      .items(vec![
        Modifier::builder()
          .name("indeterminate")
          .values(vec!["&:indeterminate", "&[aria-checked=mixed]"],)
          .build(),
        Modifier::builder()
          .name("checked")
          .values(vec!["&[aria-checked=true]", "&:checked"],)
          .build(),
        Modifier::builder()
          .name("unchecked")
          .values(vec!["&[aria-checked=false]", "&:not(:checked)"])
          .build(),
      ])
      .build(),
    Group::builder()
      .name("grabbed-group")
      .items(vec![
        Modifier::builder()
          .name("grabbed")
          .values(vec!["&[aria-grabbed=true]"])
          .build()
      ])
      .build(),
    Group::builder()
      .name("pressed-group")
      .items(vec![
        Modifier::builder()
          .name("pressed")
          .values(vec!["&[aria-pressed=true]"])
          .build()
      ])
      .build(),
    Group::builder()
      .name("invalid-grammar-group")
      .items(vec![
        Modifier::builder()
          .name("invalid-grammar")
          .values(vec!["&[aria-invalid=grammar]"])
          .build()
      ])
      .build(),
    Group::builder()
      .name("invalid-spelling-group")
      .items(vec![
        Modifier::builder()
          .name("invalid-spelling")
          .values(vec!["&[aria-invalid=spelling]"])
          .build()
      ])
      .build(),
    Group::builder()
      .name("valid-group")
      .items(vec![
        Modifier::builder()
          .name("valid")
          .values(vec!["&[aria-invalid=false]", "&:valid"],)
          .build(),
        Modifier::builder()
          .name("invalid")
          .values(vec!["&[aria-invalid=true]", "&:invalid"])
          .build(),
      ])
      .build(),
    Group::builder()
      .name("loading-group")
      .items(vec![
        Modifier::builder()
          .name("loading")
          .values(vec!["&[aria-busy=true]"])
          .build()
      ])
      .build(),
    Group::builder()
      .name("selected-group")
      .items(vec![
        Modifier::builder()
          .name("selected")
          .values(vec!["&[aria-selected=true]"])
          .build()
      ])
      .build(),
    Group::builder()
      .name("hidden-group")
      .items(vec![
        Modifier::builder()
          .name("aria-hidden")
          .values(vec!["&[aria-hidden=true]"])
          .build()
      ])
      .build(),
    Group::builder()
      .name("autofill-group")
      .items(vec![
        Modifier::builder()
          .name("autofill")
          .values(vec!["&:-webkit-autofill"])
          .build()
      ])
      .build(),
    Group::builder()
      .name("even-group")
      .items(vec![
        Modifier::builder()
          .name("even")
          .values(vec!["&:even"])
          .build(),
        Modifier::builder()
          .name("odd")
          .values(vec!["&:odd"])
          .build()
      ])
      .build(),
    Group::builder()
      .name("even-group")
      .items(vec![
        Modifier::builder()
          .name("even-of-type")
          .values(vec!["&:nth-of-type(even)"],)
          .build(),
        Modifier::builder()
          .name("odd-of-type")
          .values(vec!["&:nth-of-type(odd)"])
          .build(),
      ])
      .build(),
    Group::builder()
      .name("node-position-group")
      .items(vec![
        Modifier::builder()
          .name("first")
          .values(vec!["&:first"],)
          .build(),
        Modifier::builder()
          .name("not-first")
          .values(vec!["&:not(:first-child)"],)
          .build(),
        Modifier::builder()
          .name("last")
          .values(vec!["&:last"],)
          .build(),
        Modifier::builder()
          .name("not-last")
          .values(vec!["&:not(:last-child)"])
          .build(),
      ])
      .build(),
    Group::builder()
      .name("node-of-type-group")
      .items(vec![
        Modifier::builder()
          .name("first-of-type")
          .values(vec!["&:first-of-type"],)
          .build(),
        Modifier::builder()
          .name("not-first-of-type")
          .values(vec!["&:not(:first-of-type)"],)
          .build(),
        Modifier::builder()
          .name("last-of-type")
          .values(vec!["&:last-of-type"],)
          .build(),
        Modifier::builder()
          .name("not-last-of-type")
          .values(vec!["&:not(:last-of-type)"])
          .build(),
      ])
      .build(),
    Group::builder()
      .name("visited-group")
      .items(vec![
        Modifier::builder()
          .name("visited")
          .values(vec!["&:visited"])
          .build()
      ])
      .build(),
    Group::builder()
      .name("optional-group")
      .items(vec![
        Modifier::builder()
          .name("optional")
          .values(vec!["&:optional"])
          .build()
      ])
      .build(),
    Group::builder()
      .name("active-group")
      .items(vec![
        Modifier::builder()
          .name("active-link")
          .values(vec!["&[aria-current=page]"],)
          .build(),
        Modifier::builder()
          .name("active-location")
          .values(vec!["&[aria-current=location]"],)
          .build(),
        Modifier::builder()
          .name("active-date")
          .values(vec!["&[aria-current=date]"],)
          .build(),
        Modifier::builder()
          .name("active-time")
          .values(vec!["&[aria-current=time]"],)
          .build(),
        Modifier::builder()
          .name("active-step")
          .values(vec!["&[aria-current=step]"])
          .build(),
      ])
      .build(),
    Group::builder()
      .name("full-screen-group")
      .items(vec![
        Modifier::builder()
          .name("full-screen")
          .values(vec!["&:fullscreen"])
          .build()
      ])
      .build(),
    Group::builder()
      .name("target-group")
      .items(vec![
        Modifier::builder()
          .name("target")
          .values(vec!["&:target"])
          .build()
      ])
      .build(),
    Group::builder()
      .name("placeholder-shown-group")
      .items(vec![
        Modifier::builder()
          .name("placeholder-shown")
          .values(vec!["&:placeholder-shown"])
          .build()
      ])
      .build(),
    Group::builder()
      .name("required-group")
      .items(vec![
        Modifier::builder()
          .name("required")
          .values(vec!["[aria-required=true]", "&:required"],)
          .build(),
        Modifier::builder()
          .name("not-required")
          .values(vec!["[arira-required=false]", "&:not(:required)"])
          .build(),
      ])
      .build(),
    Group::builder()
      .name("default-group")
      .items(vec![
        Modifier::builder()
          .name("default")
          .values(vec!["&:default"])
          .build()
      ])
      .build(),
    Group::builder()
      .name("only-child-group")
      .items(vec![
        Modifier::builder()
          .name("only-child")
          .values(vec!["&:only-child"])
          .build(),
        Modifier::builder()
          .name("not-only-child")
          .values(vec!["&:not(:only-child)"])
          .build()
      ])
      .build(),
    Group::builder()
      .name("only-group")
      .items(vec![
        Modifier::builder()
          .name("only-of-type")
          .values(vec!["&:only-of-type"],)
          .build(),
        Modifier::builder()
          .name("not-only-of-type")
          .values(vec!["&:not(:only-of-type)"])
          .build(),
      ])
      .build(),
    Group::builder()
      .name("root-group")
      .items(vec![
        Modifier::builder()
          .name("root")
          .values(vec!["&:root"])
          .build()
      ])
      .build(),
    Group::builder()
      .name("link-group")
      .items(vec![
        Modifier::builder()
          .name("link")
          .values(vec!["&:link"])
          .build()
      ])
      .build(),
    Group::builder()
      .name("pseudo-group")
      .items(vec![
        Modifier::builder()
          .name("placeholder")
          .values(vec!["&::placeholder"],)
          .build(),
        Modifier::builder()
          .name("selection")
          .values(vec!["&::selection"],)
          .build(),
        Modifier::builder()
          .name("first-letter")
          .values(vec!["&::first-letter"],)
          .build(),
        Modifier::builder()
          .name("first-line")
          .values(vec!["&::first-line"],)
          .build(),
        Modifier::builder()
          .name("before")
          .values(vec!["&::before"],)
          .build(),
        Modifier::builder()
          .name("after")
          .values(vec!["&::after"])
          .build(),
      ])
      .build(),
  ];
  pub(crate) static ref DARK_PARENT_MODIFIERS: Group<Modifier> = Group::builder()
    .name("parent-group")
    .items(vec![
      Modifier::builder()
        .name("light")
        .values(vec![".light &"])
        .description("This class modifier becomes active when the light mode is enabled.")
        .build(),
      Modifier::builder()
        .name("dark")
        .values(vec![".dark &"])
        .description("This class modifier becomes active when the dark mode is enabled.")
        .build(),
    ])
    .build();
}
