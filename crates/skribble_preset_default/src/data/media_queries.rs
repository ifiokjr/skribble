use lazy_static::lazy_static;
use skribble_core::Group;
use skribble_core::MediaQuery;

lazy_static! {
  pub(crate) static ref MEDIA_QUERIES: Vec<Group<MediaQuery>> = vec![
    Group::builder()
      .name("deviceCategories")
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
      .name("orientation")
      .description("The orientation for the media query.")
      .items(vec![
        MediaQuery::builder()
          .name("portrait")
          .query("(orientation: portrait)")
          .description("The media query for devices with a portrait orientation.")
          .build(),
        MediaQuery::builder()
          .name("landscape")
          .query("(orientation: landscape)")
          .description("The media query for devices with a landscape orientation.")
          .build(),
      ])
      .build(),
    Group::builder()
      .name("motion")
      .description("The animation motion preference media query.")
      .items(vec![
        MediaQuery::builder()
          .name("motionReduce")
          .query("(prefers-reduced-motion: reduce)")
          .description("The media query for devices with a reduced motion preference.")
          .build(),
        MediaQuery::builder()
          .name("motionSafe")
          .query("(prefers-reduced-motion: no-preference)")
          .description("The media query for devices with a no preference motion preference.")
          .build(),
      ])
      .build(),
  ];
  pub(crate) static ref DARK_MEDIA_QUERIES: Vec<Group<MediaQuery>> = vec![
    Group::builder()
      .name("darkMode")
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
  ];
}
