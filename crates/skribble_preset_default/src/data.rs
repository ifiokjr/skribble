use indexmap::indexmap;
use indexmap::IndexMap;
use lazy_static::lazy_static;
use skribble_core::Group;
use skribble_core::MediaQuery;
use skribble_core::Modifier;

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
  pub(crate) static ref MODIFIERS: Vec<Group<Modifier>> = vec![
    Group::builder()
      .name("hoverGroup")
      .items(vec![
        Modifier::builder()
          .name("hover")
          .values(vec!["&:hover"])
          .description("This class modifier becomes active when the element is hovered.")
          .build()
      ])
      .build(),
    Group::builder()
      .name("activeGroup")
      .items(vec![
        Modifier::builder()
          .name("active")
          .values(vec!["&:active"])
          .description("")
          .build()
      ])
      .build(),
    Group::builder()
      .name("focusGroup")
      .items(vec![
        Modifier::builder()
          .name("focus")
          .values(vec!["&:focus"])
          .description("")
          .build()
      ])
      .build(),
    Group::builder()
      .name("focusWithinGroup")
      .items(vec![
        Modifier::builder()
          .name("focusWithin")
          .values(vec!["&:focus-within"])
          .description("")
          .build()
      ])
      .build(),
    Group::builder()
      .name("focusVisibleGroup")
      .items(vec![
        Modifier::builder()
          .name("focusVisible")
          .values(vec!["&:focus-visible"])
          .description("")
          .build()
      ])
      .build(),
    Group::builder()
      .name("enabledGroup")
      .items(vec![
        Modifier::builder()
          .name("disabled")
          .values(vec!["&[disabled]", "&[aria-disabled=true]", "&:disabled"],)
          .description("")
          .build(),
        Modifier::builder()
          .name("notDisabled")
          .values(vec!["&[aria-disabled=false]", "&:disabled"],)
          .description("")
          .build(),
        Modifier::builder()
          .name("enabled")
          .values(vec!["&:enabled"])
          .description("")
          .build(),
      ])
      .build(),
    Group::builder()
      .name("emptyGroup")
      .items(vec![
        Modifier::builder()
          .name("empty")
          .values(vec!["&:empty"])
          .description("")
          .build()
      ])
      .build(),
    Group::builder()
      .name("readGroup")
      .items(vec![
        Modifier::builder()
          .name("readWrite")
          .values(vec!["&:read-write"],)
          .description("")
          .build(),
        Modifier::builder()
          .name("readOnly")
          .values(vec!["&[aria-readonly=true]", "&[readonly]", "&:read-only"],)
          .description("")
          .build(),
        Modifier::builder()
          .name("notReadOnly")
          .values(vec![
            "&[aria-readonly=false]",
            "&[readonly=false]",
            "&:not(:read-only)"
          ])
          .description("")
          .build(),
      ])
      .build(),
    Group::builder()
      .name("expandedGroup")
      .items(vec![
        Modifier::builder()
          .name("expanded")
          .values(vec!["&[aria-expanded=true]"])
          .description("")
          .build()
      ])
      .build(),
    Group::builder()
      .name("checkedGroup")
      .items(vec![
        Modifier::builder()
          .name("indeterminate")
          .values(vec!["&:indeterminate", "&[aria-checked=mixed]"],)
          .description("")
          .build(),
        Modifier::builder()
          .name("checked")
          .values(vec!["&[aria-checked=true]", "&:checked"],)
          .description("")
          .build(),
        Modifier::builder()
          .name("unchecked")
          .values(vec!["&[aria-checked=false]", "&:not(:checked)"])
          .description("")
          .build(),
      ])
      .build(),
    Group::builder()
      .name("grabbedGroup")
      .items(vec![
        Modifier::builder()
          .name("grabbed")
          .values(vec!["&[aria-grabbed=true]"])
          .description("")
          .build()
      ])
      .build(),
    Group::builder()
      .name("pressedGroup")
      .items(vec![
        Modifier::builder()
          .name("pressed")
          .values(vec!["&[aria-pressed=true]"])
          .description("")
          .build()
      ])
      .build(),
    Group::builder()
      .name("invalidGrammarGroup")
      .items(vec![
        Modifier::builder()
          .name("invalidGrammar")
          .values(vec!["&[aria-invalid=grammar]"])
          .description("")
          .build()
      ])
      .build(),
    Group::builder()
      .name("invalidSpellingGroup")
      .items(vec![
        Modifier::builder()
          .name("invalidSpelling")
          .values(vec!["&[aria-invalid=spelling]"])
          .description("")
          .build()
      ])
      .build(),
    Group::builder()
      .name("validGroup")
      .items(vec![
        Modifier::builder()
          .name("valid")
          .values(vec!["&[aria-invalid=false]", "&:valid"],)
          .description("")
          .build(),
        Modifier::builder()
          .name("invalid")
          .values(vec!["&[aria-invalid=true]", "&:invalid"])
          .description("")
          .build(),
      ])
      .build(),
    Group::builder()
      .name("loadingGroup")
      .items(vec![
        Modifier::builder()
          .name("loading")
          .values(vec!["&[aria-busy=true]"])
          .description("")
          .build()
      ])
      .build(),
    Group::builder()
      .name("selectedGroup")
      .items(vec![
        Modifier::builder()
          .name("selected")
          .values(vec!["&[aria-selected=true]"])
          .description("")
          .build()
      ])
      .build(),
    Group::builder()
      .name("hiddenGroup")
      .items(vec![
        Modifier::builder()
          .name("hidden")
          .values(vec!["&[hidden]"])
          .description("")
          .build()
      ])
      .build(),
    Group::builder()
      .name("autofillGroup")
      .items(vec![
        Modifier::builder()
          .name("autofill")
          .values(vec!["&:-webkit-autofill"])
          .description("")
          .build()
      ])
      .build(),
    Group::builder()
      .name("evenGroup")
      .items(vec![
        Modifier::builder()
          .name("even")
          .values(vec!["&:even"])
          .description("")
          .build(),
        Modifier::builder()
          .name("odd")
          .values(vec!["&:odd"])
          .description("")
          .build()
      ])
      .build(),
    Group::builder()
      .name("evenGroup")
      .items(vec![
        Modifier::builder()
          .name("evenOfType")
          .values(vec!["&:nth-of-type(even)"],)
          .description("")
          .build(),
        Modifier::builder()
          .name("oddOfType")
          .values(vec!["&:nth-of-type(odd)"])
          .description("")
          .build(),
      ])
      .build(),
    Group::builder()
      .name("nodePositionGroup")
      .items(vec![
        Modifier::builder()
          .name("first")
          .values(vec!["&:first"],)
          .description("")
          .build(),
        Modifier::builder()
          .name("notFirst")
          .values(vec!["&:not(:first-child)"],)
          .description("")
          .build(),
        Modifier::builder()
          .name("last")
          .values(vec!["&:last"],)
          .description("")
          .build(),
        Modifier::builder()
          .name("notLast")
          .values(vec!["&:not(:last-child)"])
          .description("")
          .build(),
      ])
      .build(),
    Group::builder()
      .name("nodeOfTypeGroup")
      .items(vec![
        Modifier::builder()
          .name("firstOfType")
          .values(vec!["&:first-of-type"],)
          .description("")
          .build(),
        Modifier::builder()
          .name("notFirstOfType")
          .values(vec!["&:not(:first-of-type)"],)
          .description("")
          .build(),
        Modifier::builder()
          .name("lastOfType")
          .values(vec!["&:last-of-type"],)
          .description("")
          .build(),
        Modifier::builder()
          .name("notLastOfType")
          .values(vec!["&:not(:last-of-type)"])
          .description("")
          .build(),
      ])
      .build(),
    Group::builder()
      .name("visitedGroup")
      .items(vec![
        Modifier::builder()
          .name("visited")
          .values(vec!["&:visited"])
          .description("")
          .build()
      ])
      .build(),
    Group::builder()
      .name("optionalGroup")
      .items(vec![
        Modifier::builder()
          .name("optional")
          .values(vec!["&:optional"])
          .description("")
          .build()
      ])
      .build(),
    Group::builder()
      .name("activeGroup")
      .items(vec![
        Modifier::builder()
          .name("activeLink")
          .values(vec!["&[aria-current=page]"],)
          .description("")
          .build(),
        Modifier::builder()
          .name("activeLocation")
          .values(vec!["&[aria-current=location]"],)
          .description("")
          .build(),
        Modifier::builder()
          .name("activeDate")
          .values(vec!["&[aria-current=date]"],)
          .description("")
          .build(),
        Modifier::builder()
          .name("activeTime")
          .values(vec!["&[aria-current=time]"],)
          .description("")
          .build(),
        Modifier::builder()
          .name("activeStep")
          .values(vec!["&[aria-current=step]"])
          .description("")
          .build(),
      ])
      .build(),
    Group::builder()
      .name("fullScreenGroup")
      .items(vec![
        Modifier::builder()
          .name("fullScreen")
          .values(vec!["&:fullscreen"])
          .description("")
          .build()
      ])
      .build(),
    Group::builder()
      .name("targetGroup")
      .items(vec![
        Modifier::builder()
          .name("target")
          .values(vec!["&:target"])
          .description("")
          .build()
      ])
      .build(),
    Group::builder()
      .name("placeholderShownGroup")
      .items(vec![
        Modifier::builder()
          .name("placeholderShown")
          .values(vec!["&:placeholder-shown"])
          .description("")
          .build()
      ])
      .build(),
    Group::builder()
      .name("requiredGroup")
      .items(vec![
        Modifier::builder()
          .name("required")
          .values(vec!["[aria-required=true]", "&:required"],)
          .description("")
          .build(),
        Modifier::builder()
          .name("notRequired")
          .values(vec!["[arira-required=false]", "&:not(:required)"])
          .description("")
          .build(),
      ])
      .build(),
    Group::builder()
      .name("defaultGroup")
      .items(vec![
        Modifier::builder()
          .name("default")
          .values(vec!["&:default"])
          .description("")
          .build()
      ])
      .build(),
    Group::builder()
      .name("onlyChildGroup")
      .items(vec![
        Modifier::builder()
          .name("onlyChild")
          .values(vec!["&:only-child"])
          .description("")
          .build(),
        Modifier::builder()
          .name("notOnlyChild")
          .values(vec!["&:not(:only-child)"])
          .description("")
          .build()
      ])
      .build(),
    Group::builder()
      .name("onlyGroup")
      .items(vec![
        Modifier::builder()
          .name("onlyOfType")
          .values(vec!["&:only-of-type"],)
          .description("")
          .build(),
        Modifier::builder()
          .name("notOnlyOfType")
          .values(vec!["&:not(:only-of-type)"])
          .description("")
          .build(),
      ])
      .build(),
    Group::builder()
      .name("rootGroup")
      .items(vec![
        Modifier::builder()
          .name("root")
          .values(vec!["&:root"])
          .description("")
          .build()
      ])
      .build(),
    Group::builder()
      .name("linkGroup")
      .items(vec![
        Modifier::builder()
          .name("link")
          .values(vec!["&:link"])
          .description("")
          .build()
      ])
      .build(),
    Group::builder()
      .name("pseudoGroup")
      .items(vec![
        Modifier::builder()
          .name("placeholder")
          .values(vec!["&::placeholder"],)
          .description("")
          .build(),
        Modifier::builder()
          .name("selection")
          .values(vec!["&::selection"],)
          .description("")
          .build(),
        Modifier::builder()
          .name("firstLetter")
          .values(vec!["&::first-letter"],)
          .description("")
          .build(),
        Modifier::builder()
          .name("firstLine")
          .values(vec!["&::first-line"],)
          .description("")
          .build(),
        Modifier::builder()
          .name("before")
          .values(vec!["&::before"],)
          .description("")
          .build(),
        Modifier::builder()
          .name("after")
          .values(vec!["&::after"])
          .description("")
          .build(),
      ])
      .build(),
  ];
  pub(crate) static ref OPEN_COLOR_PALETTE: IndexMap<&'static str, &'static str> = indexmap! {
    "inherit" => "inherit",
    "current" => "currentColor",
    "transparent" => "transparent",
    "white" => "#ffffff",
    "black" => "#000000",
    "gray50" => "#f8f9fa",
    "gray100" => "#f1f3f5",
    "gray200" => "#e9ecef",
    "gray300" => "#dee2e6",
    "gray400" => "#ced4da",
    "gray500" => "#adb5bd",
    "gray600" => "#868e96",
    "gray700" => "#495057",
    "gray800" => "#343a40",
    "gray900" => "#212529",
    "red50" => "#fff5f5",
    "red100" => "#ffe3e3",
    "red200" => "#ffc9c9",
    "red300" => "#ffa8a8",
    "red400" => "#ff8787",
    "red500" => "#ff6b6b",
    "red600" => "#fa5252",
    "red700" => "#f03e3e",
    "red800" => "#e03131",
    "red900" => "#c92a2a",
    "pink50" => "#fff0f6",
    "pink100" => "#ffdeeb",
    "pink200" => "#fcc2d7",
    "pink300" => "#faa2c1",
    "pink400" => "#f783ac",
    "pink500" => "#f06595",
    "pink600" => "#e64980",
    "pink700" => "#d6336c",
    "pink800" => "#c2255c",
    "pink900" => "#a61e4d",
    "grape50" => "#f8f0fc",
    "grape100" => "#f3d9fa",
    "grape200" => "#eebefa",
    "grape300" => "#e599f7",
    "grape400" => "#da77f2",
    "grape500" => "#cc5de8",
    "grape600" => "#be4bdb",
    "grape700" => "#ae3ec9",
    "grape800" => "#9c36b5",
    "grape900" => "#862e9c",
    "violet50" => "#f3f0ff",
    "violet100" => "#e5dbff",
    "violet200" => "#d0bfff",
    "violet300" => "#b197fc",
    "violet400" => "#9775fa",
    "violet500" => "#845ef7",
    "violet600" => "#7950f2",
    "violet700" => "#7048e8",
    "violet800" => "#6741d9",
    "violet900" => "#5f3dc4",
    "indigo50" => "#edf2ff",
    "indigo100" => "#dbe4ff",
    "indigo200" => "#bac8ff",
    "indigo300" => "#91a7ff",
    "indigo400" => "#748ffc",
    "indigo500" => "#5c7cfa",
    "indigo600" => "#4c6ef5",
    "indigo700" => "#4263eb",
    "indigo800" => "#3b5bdb",
    "indigo900" => "#364fc7",
    "blue50" => "#e7f5ff",
    "blue100" => "#d0ebff",
    "blue200" => "#a5d8ff",
    "blue300" => "#74c0fc",
    "blue400" => "#4dabf7",
    "blue500" => "#339af0",
    "blue600" => "#228be6",
    "blue700" => "#1c7ed6",
    "blue800" => "#1971c2",
    "blue900" => "#1864ab",
    "cyan50" => "#e3fafc",
    "cyan100" => "#c5f6fa",
    "cyan200" => "#99e9f2",
    "cyan300" => "#66d9e8",
    "cyan400" => "#3bc9db",
    "cyan500" => "#22b8cf",
    "cyan600" => "#15aabf",
    "cyan700" => "#1098ad",
    "cyan800" => "#0c8599",
    "cyan900" => "#0b7285",
    "teal50" => "#e6fcf5",
    "teal100" => "#c3fae8",
    "teal200" => "#96f2d7",
    "teal300" => "#63e6be",
    "teal400" => "#38d9a9",
    "teal500" => "#20c997",
    "teal600" => "#12b886",
    "teal700" => "#0ca678",
    "teal800" => "#099268",
    "teal900" => "#087f5b",
    "green50" => "#ebfbee",
    "green100" => "#d3f9d8",
    "green200" => "#b2f2bb",
    "green300" => "#8ce99a",
    "green400" => "#69db7c",
    "green500" => "#51cf66",
    "green600" => "#40c057",
    "green700" => "#37b24d",
    "green800" => "#2f9e44",
    "green900" => "#2b8a3e",
    "lime50" => "#f4fce3",
    "lime100" => "#e9fac8",
    "lime200" => "#d8f5a2",
    "lime300" => "#c0eb75",
    "lime400" => "#a9e34b",
    "lime500" => "#94d82d",
    "lime600" => "#82c91e",
    "lime700" => "#74b816",
    "lime800" => "#66a80f",
    "lime900" => "#5c940d",
    "yellow50" => "#fff9db",
    "yellow100" => "#fff3bf",
    "yellow200" => "#ffec99",
    "yellow300" => "#ffe066",
    "yellow400" => "#ffd43b",
    "yellow500" => "#fcc419",
    "yellow600" => "#fab005",
    "yellow700" => "#f59f00",
    "yellow800" => "#f08c00",
    "yellow900" => "#e67700",
    "orange50" => "#fff4e6",
    "orange100" => "#ffe8cc",
    "orange200" => "#ffd8a8",
    "orange300" => "#ffc078",
    "orange400" => "#ffa94d",
    "orange500" => "#ff922b",
    "orange600" => "#fd7e14",
    "orange700" => "#f76707",
    "orange800" => "#e8590c",
    "orange900" => "#d9480f",
  };
  pub(crate) static ref TAILWIND_PALETTE: IndexMap<&'static str, &'static str> = indexmap! {
    "inherit" => "inherit",
    "current" => "currentColor",
    "transparent" => "transparent",
    "black" => "#000",
    "white" => "#fff",
    "slate50" => "#f8fafc",
    "slate100" => "#f1f5f9",
    "slate200" => "#e2e8f0",
    "slate300" => "#cbd5e1",
    "slate400" => "#94a3b8",
    "slate500" => "#64748b",
    "slate600" => "#475569",
    "slate700" => "#334155",
    "slate800" => "#1e293b",
    "slate900" => "#0f172a",
    "gray50" => "#f9fafb",
    "gray100" => "#f3f4f6",
    "gray200" => "#e5e7eb",
    "gray300" => "#d1d5db",
    "gray400" => "#9ca3af",
    "gray500" => "#6b7280",
    "gray600" => "#4b5563",
    "gray700" => "#374151",
    "gray800" => "#1f2937",
    "gray900" => "#111827",
    "zinc50" => "#fafafa",
    "zinc100" => "#f4f4f5",
    "zinc200" => "#e4e4e7",
    "zinc300" => "#d4d4d8",
    "zinc400" => "#a1a1aa",
    "zinc500" => "#71717a",
    "zinc600" => "#52525b",
    "zinc700" => "#3f3f46",
    "zinc800" => "#27272a",
    "zinc900" => "#18181b",
    "neutral50" => "#fafafa",
    "neutral100" => "#f5f5f5",
    "neutral200" => "#e5e5e5",
    "neutral300" => "#d4d4d4",
    "neutral400" => "#a3a3a3",
    "neutral500" => "#737373",
    "neutral600" => "#525252",
    "neutral700" => "#404040",
    "neutral800" => "#262626",
    "neutral900" => "#171717",
    "stone50" => "#fafaf9",
    "stone100" => "#f5f5f4",
    "stone200" => "#e7e5e4",
    "stone300" => "#d6d3d1",
    "stone400" => "#a8a29e",
    "stone500" => "#78716c",
    "stone600" => "#57534e",
    "stone700" => "#44403c",
    "stone800" => "#292524",
    "stone900" => "#1c1917",
    "red50" => "#fef2f2",
    "red100" => "#fee2e2",
    "red200" => "#fecaca",
    "red300" => "#fca5a5",
    "red400" => "#f87171",
    "red500" => "#ef4444",
    "red600" => "#dc2626",
    "red700" => "#b91c1c",
    "red800" => "#991b1b",
    "red900" => "#7f1d1d",
    "orange50" => "#fff7ed",
    "orange100" => "#ffedd5",
    "orange200" => "#fed7aa",
    "orange300" => "#fdba74",
    "orange400" => "#fb923c",
    "orange500" => "#f97316",
    "orange600" => "#ea580c",
    "orange700" => "#c2410c",
    "orange800" => "#9a3412",
    "orange900" => "#7c2d12",
    "amber50" => "#fffbeb",
    "amber100" => "#fef3c7",
    "amber200" => "#fde68a",
    "amber300" => "#fcd34d",
    "amber400" => "#fbbf24",
    "amber500" => "#f59e0b",
    "amber600" => "#d97706",
    "amber700" => "#b45309",
    "amber800" => "#92400e",
    "amber900" => "#78350f",
    "yellow50" => "#fefce8",
    "yellow100" => "#fef9c3",
    "yellow200" => "#fef08a",
    "yellow300" => "#fde047",
    "yellow400" => "#facc15",
    "yellow500" => "#eab308",
    "yellow600" => "#ca8a04",
    "yellow700" => "#a16207",
    "yellow800" => "#854d0e",
    "yellow900" => "#713f12",
    "lime50" => "#f7fee7",
    "lime100" => "#ecfccb",
    "lime200" => "#d9f99d",
    "lime300" => "#bef264",
    "lime400" => "#a3e635",
    "lime500" => "#84cc16",
    "lime600" => "#65a30d",
    "lime700" => "#4d7c0f",
    "lime800" => "#3f6212",
    "lime900" => "#365314",
    "green50" => "#f0fdf4",
    "green100" => "#dcfce7",
    "green200" => "#bbf7d0",
    "green300" => "#86efac",
    "green400" => "#4ade80",
    "green500" => "#22c55e",
    "green600" => "#16a34a",
    "green700" => "#15803d",
    "green800" => "#166534",
    "green900" => "#14532d",
    "emerald50" => "#ecfdf5",
    "emerald100" => "#d1fae5",
    "emerald200" => "#a7f3d0",
    "emerald300" => "#6ee7b7",
    "emerald400" => "#34d399",
    "emerald500" => "#10b981",
    "emerald600" => "#059669",
    "emerald700" => "#047857",
    "emerald800" => "#065f46",
    "emerald900" => "#064e3b",
    "teal50" => "#f0fdfa",
    "teal100" => "#ccfbf1",
    "teal200" => "#99f6e4",
    "teal300" => "#5eead4",
    "teal400" => "#2dd4bf",
    "teal500" => "#14b8a6",
    "teal600" => "#0d9488",
    "teal700" => "#0f766e",
    "teal800" => "#115e59",
    "teal900" => "#134e4a",
    "cyan50" => "#ecfeff",
    "cyan100" => "#cffafe",
    "cyan200" => "#a5f3fc",
    "cyan300" => "#67e8f9",
    "cyan400" => "#22d3ee",
    "cyan500" => "#06b6d4",
    "cyan600" => "#0891b2",
    "cyan700" => "#0e7490",
    "cyan800" => "#155e75",
    "cyan900" => "#164e63",
    "sky50" => "#f0f9ff",
    "sky100" => "#e0f2fe",
    "sky200" => "#bae6fd",
    "sky300" => "#7dd3fc",
    "sky400" => "#38bdf8",
    "sky500" => "#0ea5e9",
    "sky600" => "#0284c7",
    "sky700" => "#0369a1",
    "sky800" => "#075985",
    "sky900" => "#0c4a6e",
    "blue50" => "#eff6ff",
    "blue100" => "#dbeafe",
    "blue200" => "#bfdbfe",
    "blue300" => "#93c5fd",
    "blue400" => "#60a5fa",
    "blue500" => "#3b82f6",
    "blue600" => "#2563eb",
    "blue700" => "#1d4ed8",
    "blue800" => "#1e40af",
    "blue900" => "#1e3a8a",
    "indigo50" => "#eef2ff",
    "indigo100" => "#e0e7ff",
    "indigo200" => "#c7d2fe",
    "indigo300" => "#a5b4fc",
    "indigo400" => "#818cf8",
    "indigo500" => "#6366f1",
    "indigo600" => "#4f46e5",
    "indigo700" => "#4338ca",
    "indigo800" => "#3730a3",
    "indigo900" => "#312e81",
    "violet50" => "#f5f3ff",
    "violet100" => "#ede9fe",
    "violet200" => "#ddd6fe",
    "violet300" => "#c4b5fd",
    "violet400" => "#a78bfa",
    "violet500" => "#8b5cf6",
    "violet600" => "#7c3aed",
    "violet700" => "#6d28d9",
    "violet800" => "#5b21b6",
    "violet900" => "#4c1d95",
    "purple50" => "#faf5ff",
    "purple100" => "#f3e8ff",
    "purple200" => "#e9d5ff",
    "purple300" => "#d8b4fe",
    "purple400" => "#c084fc",
    "purple500" => "#a855f7",
    "purple600" => "#9333ea",
    "purple700" => "#7e22ce",
    "purple800" => "#6b21a8",
    "purple900" => "#581c87",
    "fuchsia50" => "#fdf4ff",
    "fuchsia100" => "#fae8ff",
    "fuchsia200" => "#f5d0fe",
    "fuchsia300" => "#f0abfc",
    "fuchsia400" => "#e879f9",
    "fuchsia500" => "#d946ef",
    "fuchsia600" => "#c026d3",
    "fuchsia700" => "#a21caf",
    "fuchsia800" => "#86198f",
    "fuchsia900" => "#701a75",
    "pink50" => "#fdf2f8",
    "pink100" => "#fce7f3",
    "pink200" => "#fbcfe8",
    "pink300" => "#f9a8d4",
    "pink400" => "#f472b6",
    "pink500" => "#ec4899",
    "pink600" => "#db2777",
    "pink700" => "#be185d",
    "pink800" => "#9d174d",
    "pink900" => "#831843",
    "rose50" => "#fff1f2",
    "rose100" => "#ffe4e6",
    "rose200" => "#fecdd3",
    "rose300" => "#fda4af",
    "rose400" => "#fb7185",
    "rose500" => "#f43f5e",
    "rose600" => "#e11d48",
    "rose700" => "#be123c",
    "rose800" => "#9f1239",
    "rose900" => "#881337"
  };
}
