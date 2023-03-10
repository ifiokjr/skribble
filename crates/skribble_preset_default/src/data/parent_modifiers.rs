use lazy_static::lazy_static;
use skribble_core::Modifier;

lazy_static! {
  pub(crate) static ref PARENT_MODIFIERS: Vec<Modifier> = vec![
    Modifier::builder()
      .name("rtl")
      .values(vec!["[dir=rtl] &"])
      .description(
        "This class modifier becomes active when when the text direction of any parent nodes is \
         set to right to left."
      )
      .build(),
    Modifier::builder()
      .name("groupHover")
      .values(vec![
        ".\\$group:hover &",
        ".group:hover &",
        "[role='group']:hover &"
      ])
      .description("This class modifier becomes active when a parent group is hovered.")
      .build(),
    Modifier::builder()
      .name("groupFocus")
      .values(vec![
        ".\\$group:focus &",
        ".group:focus &",
        "[role='group']:focus &"
      ])
      .description("This class modifier becomes active when a parent group is focused.")
      .build(),
    Modifier::builder()
      .name("groupActive")
      .values(vec![
        ".\\$group:active &",
        ".group:active &",
        "[role='group']:active &"
      ])
      .description("This class modifier becomes active when a parent group is active.")
      .build(),
    Modifier::builder()
      .name("groupVisited")
      .values(vec![
        ".\\$group:visited &",
        ".group:visited &",
        "[role='group']:visited &"
      ])
      .description("This class modifier becomes active when a parent group is visited.")
      .build(),
  ];
  pub(crate) static ref DARK_PARENT_MODIFIERS: Vec<Modifier> = vec![
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
  ];
}
