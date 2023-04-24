use indexmap::indexmap;
use lazy_static::lazy_static;
use skribble_core::ValueSet;

lazy_static! {
  pub(crate) static ref ATOM_VALUE_SETS: Vec<ValueSet> = {
    vec![
      ValueSet::builder()
        .name("screen-reader")
        .values(indexmap! {
          "only" => indexmap! {
            "position" => "absolute",
            "width" => "1px",
            "height" => "1px",
            "padding" => "0",
            "margin" => "-1px",
            "overflow" => "hidden",
            "clip" => "rect(0, 0, 0, 0)",
            "whiteSpace" => "nowrap",
            "borderWidth" => "0"
          },
          "exclude" => indexmap! {
            "position" => "static",
            "width" => "auto",
            "height" => "auto",
            "padding" => "0",
            "margin" => "0",
            "overflow" => "visible",
            "clip" => "auto",
            "whiteSpace" => "normal"
          }
        })
        .build(),
      ValueSet::builder()
        .name("transition")
        .values(indexmap! {
          "main" => indexmap! {
            "transition-property" => "color, background-color, border-color, text-decoration-color, fill, stroke, opacity, box-shadow, transform, filter, backdrop-filter",
            "transition-timing-function" => "cubic-bezier(0.4, 0, 0.2, 1)",
            "transition-duration" => "var(--default-transition-duration)"
          },
          "none" => indexmap! { "transitionProperty" => "none" },
          "all" => indexmap! {
            "transition-property" => "all",
            "transition-timing-function" => "cubic-bezier(0.4, 0, 0.2, 1)",
            "transition-duration" => "var(--default-transition-duration)"
          },
          "colors" => indexmap! {
            "transition-property" => "color, background-color, border-color, text-decoration-color, fill, stroke",
            "transition-timing-function" => "cubic-bezier(0.4, 0, 0.2, 1)",
            "transition-duration" => "var(--default-transition-duration)"
          },
          "opacity" => indexmap! {
            "transition-property" => "opacity",
            "transition-timing-function" => "cubic-bezier(0.4, 0, 0.2, 1)",
            "transition-duration" => "var(--default-transition-duration)"
          },
          "shadow" => indexmap! {
            "transition-property" => "box-shadow",
            "transition-timing-function" => "cubic-bezier(0.4, 0, 0.2, 1)",
            "transition-duration" => "var(--default-transition-duration)"
          },
          "transform" => indexmap! {
            "transition-property" => "transform",
            "transition-timing-function" => "cubic-bezier(0.4, 0, 0.2, 1)",
            "transition-duration" => "var(--default-transition-duration)"
          }
        })
        .description("The built-in transition class values.")
        .build(),
      ValueSet::builder()
        .name("transition-properties")
        .values(indexmap!{
          "most" => "color, background-color, border-color, text-decoration-color, fill, stroke, opacity, box-shadow, transform, filter, backdrop-filter",
          "none" => "none",
          "all" => "all",
          "colors" => "color, background-color, border-color, text-decoration-color, fill, stroke",
          "opacity" => "opacity",
          "shadow" => "box-shadow",
          "transform" => "transform",
        })
        .build(),
      ValueSet::builder()
        .name("duration")
        .values(indexmap! {
          "0" => "0ms",
          "75" => "75ms",
          "100" => "100ms",
          "150" => "150ms",
          "200" => "200ms",
          "300" => "300ms",
          "500" => "500ms",
          "700" => "700ms",
          "1000" => "1000ms",
          "1500" => "1500ms",
          "2000" => "2000ms",
        })
        .build(),
      ValueSet::builder()
        .name("easing")
        .values(indexmap! {
          "linear" => "linear",
          "in" => "cubic-bezier(0.4, 0, 1, 1)",
          "out" => "cubic-bezier(0, 0, 0.2, 1)",
          "inOut" => "cubic-bezier(0.4, 0, 0.2, 1)",
        })
        .build(),
      ValueSet::builder()
        .name("animation-repetitions")
        .values(indexmap! {
          "0" => "0",
          "0.5" => "0.5",
          "1" => "1",
          "2" => "2",
          "3" => "3",
          "infinite" => "infinite",
          "in" => "cubic-bezier(0.4, 0, 1, 1)",
          "out" => "cubic-bezier(0, 0, 0.2, 1)",
          "inOut" => "cubic-bezier(0.4, 0, 0.2, 1)",
        })
        .build(),
      ValueSet::builder()
        .name("animation-direction")
        .values(indexmap! {
          "normal" => "normal",
          "reverse" => "reverse",
          "alt" => "alternate",
          "altReverse" => "alternate-reverse"
        })
        .build(),
      ValueSet::builder()
        .name("animation-fill-mode")
        .values(indexmap! {
          "none" => "none",
          "forwards" => "forwards",
          "backwards" => "backwards",
          "both" => "both",
        })
        .description("These are the default animation fill mode values.")
        .build(),
      ValueSet::builder()
        .name("animation-state")
        .values(indexmap! { "running" => "running", "paused" => "paused" })
        .description("These are the default animation state values.")
        .build(),
      ValueSet::builder()
        .name("opacity")
        .values(indexmap! {
          "0" => "0%",
          "5" => "5%",
          "10" => "10%",
          "20" => "20%",
          "30" => "30%",
          "40" => "40%",
          "50" => "50%",
          "60" => "60%",
          "70" => "70%",
          "75" => "75%",
          "80" => "80%",
          "90" => "90%",
          "95" => "95%",
          "100" => "100%",
         })
        .description("These are the default opacity values.")
        .build(),
      ValueSet::builder()
        .name("spacing")
        .values(indexmap! {
          "0" => "0px",
          "1" => "0.25rem",
          "2" => "0.5rem",
          "3" => "0.75rem",
          "4" => "1rem",
          "5" => "1.25rem",
          "6" => "1.5rem",
          "7" => "1.75rem",
          "8" => "2rem",
          "9" => "2.25rem",
          "10" => "2.5rem",
          "11" => "2.75rem",
          "12" => "3rem",
          "14" => "3.5rem",
          "16" => "4rem",
          "20" => "5rem",
          "24" => "6rem",
          "28" => "7rem",
          "32" => "8rem",
          "36" => "9rem",
          "40" => "10rem",
          "44" => "11rem",
          "48" => "12rem",
          "52" => "13rem",
          "56" => "14rem",
          "60" => "15rem",
          "64" => "16rem",
          "72" => "18rem",
          "80" => "20rem",
          "96" => "24rem",
          "px" => "1px",
          "0.5" => "0.125rem",
          "1.5" => "0.375rem",
          "2.5" => "0.625rem",
          "3.5" => "0.875rem",
         })
        .build(),
      ValueSet::builder()
        .name("negative-spacing")
        .values(indexmap! {
          "-1" => "-0.25rem",
          "-2" => "-0.5rem",
          "-3" => "-0.75rem",
          "-4" => "-1rem",
          "-5" => "-1.25rem",
          "-6" => "-1.5rem",
          "-7" => "-1.75rem",
          "-8" => "-2rem",
          "-9" => "-2.25rem",
          "-10" => "-2.5rem",
          "-11" => "-2.75rem",
          "-12" => "-3rem",
          "-14" => "-3.5rem",
          "-16" => "-4rem",
          "-20" => "-5rem",
          "-24" => "-6rem",
          "-28" => "-7rem",
          "-32" => "-8rem",
          "-36" => "-9rem",
          "-40" => "-10rem",
          "-44" => "-11rem",
          "-48" => "-12rem",
          "-52" => "-13rem",
          "-56" => "-14rem",
          "-60" => "-15rem",
          "-64" => "-16rem",
          "-72" => "-18rem",
          "-80" => "-20rem",
          "-96" => "-24rem",
          "-px" => "-1px",
          "-0.5" => "-0.125rem",
          "-1.5" => "-0.375rem",
          "-2.5" => "-0.625rem",
          "-3.5" => "-0.875rem",
         })
        .build(),
      ValueSet::builder()
        .name("relative-spacing")
        .values(indexmap! {
          "auto" => "auto",
          "half" => "50%",
          "1-of-3" => "33.333333%",
          "2-of-3" => "66.666667%",
          "1-of-4" => "25%",
          "3-of-4" => "75%",
          "1-of-5" => "20%",
          "2-of-5" => "40%",
          "3-of-5" => "60%",
          "4-of-5" => "80%",
          "1-of-6" => "16.666667%",
          "5-of-6" => "83.333333%",
          "1-of-12" => "8.333333%",
          "5-of-12" => "41.666667%",
          "7-of-12" => "58.333333%",
          "11-of-12" => "91.666667%",
          "full" => "100%",
         })
        .build(),
      ValueSet::builder()
        .name("negative-relative-spacing")
        .values(indexmap! {
          "-half" => "-50%",
          "-1-of-3" => "-33.333333%",
          "-2-of-3" => "-66.666667%",
          "-1-of-4" => "-25%",
          "-3-of-4" => "-75%",
          "-1-of-5" => "-20%",
          "-2-of-5" => "-40%",
          "-3-of-5" => "-60%",
          "-4-of-5" => "-80%",
          "-1-of-6" => "-16.666667%",
          "-5-of-6" => "-83.333333%",
          "-1-of-12" => "-8.333333%",
          "-5-of-12" => "-41.666667%",
          "-7-of-12" => "-58.333333%",
          "-11-of-12" => "-91.666667%",
          "-full" => "-100%",
         })
        .build(),
      ValueSet::builder()
         .name("content-fit")
         .values(indexmap! {
          "min" => "min-content",
          "max" => "max-content",
          "fit" => "fit-content",
         })
         .build(),
      ValueSet::builder()
         .name("screen-width")
         .values(indexmap! {
          "screen" => "100vw",
          "half-screen" => "50vw",
          "1-of-3-screen" => "33.333333vw",
          "2-of-3-screen" => "66.666666vw",
          "1-of-4-screen" => "25vw",
          "3-of-4-screen" => "75vw",
          "1-of-5-screen" => "20vw",
          "2-of-5-screen" => "40vw",
          "3-of-5-screen" => "60vw",
          "4-of-5-screen" => "80vw",
          "1-of-6-screen" => "16.666667vw",
          "5-of-6-screen" => "83.333333vw",
          "1-of-12-screen" => "8.333333vw",
          "5-of-12-screen" => "41.666667vw",
          "7-of-12-screen" => "58.333333vw",
          "11-of-12-screen" => "91.666667vw",
         })
         .build(),
      ValueSet::builder()
         .name("screen-height")
         .values(indexmap! {
          "screen" => "100vh",
          "half-screen" => "50vh",
          "1-of-3-screen" => "33.333333vh",
          "2-of-3-screen" => "66.666666vh",
          "1-of-4-screen" => "25vh",
          "3-of-4-screen" => "75vh",
          "1-of-5-screen" => "20vh",
          "2-of-5-screen" => "40vh",
          "3-of-5-screen" => "60vh",
          "4-of-5-screen" => "80vh",
          "1-of-6-screen" => "16.666667vh",
          "5-of-6-screen" => "83.333333vh",
          "1-of-12-screen" => "8.333333vh",
          "5-of-12-screen" => "41.666667vh",
          "7-of-12-screen" => "58.333333vh",
          "11-of-12-screen" => "91.666667vh",
         })
         .build(),
      ValueSet::builder()
         .name("max-width")
         .values(indexmap! {
          "0"	=> "0rem; /* 0px */",
          "none"	=> "none;",
          "xs"	=> "20rem; /* 320px */",
          "sm"	=> "24rem; /* 384px */",
          "md"	=> "28rem; /* 448px */",
          "lg"	=> "32rem; /* 512px */",
          "xl"	=> "36rem; /* 576px */",
          "2xl"	=> "42rem; /* 672px */",
          "3xl"	=> "48rem; /* 768px */",
          "4xl"	=> "56rem; /* 896px */",
          "5xl"	=> "64rem; /* 1024px */",
          "6xl"	=> "72rem; /* 1152px */",
          "7xl"	=> "80rem; /* 1280px */",
          "full"	=> "100%",
          "min"	=> "min-content",
          "max"	=> "max-content",
          "fit"	=> "fit-content",
          "prose"	=> "65ch",
          "screen-sm"	=> "640px",
          "screen-md"	=> "768px",
          "screen-lg"	=> "1024px",
          "screen-xl"	=> "1280px",
          "screen-2xl"	=> "1536px",
         })
         .build(),
      ValueSet::builder()
        .name("z-index")
        .values(indexmap! {
          "0" => "0",
          "1" => "1",
          "2" => "2",
          "3" => "3",
          "4" => "4",
          "5" => "5",
          "10" => "10",
          "20" => "20",
          "30" => "30",
          "40" => "40",
          "50" => "50",
          "60" => "60",
          "auto" => "auto",
          "-1" => "-1",
          "-2" => "-2",
          "-3" => "-3",
          "-5" => "-5",
          "-10" => "-10",
          "-20" => "-20",
          "-30" => "-30",
          "-40" => "-40",
          "-50" => "-50",
          "-60" => "-60",
         })
        .build(),
      ValueSet::builder()
        .name("direction")
        .values(indexmap! { "ltr" => "ltr", "rtl" => "rtl" })
        .build(),
      ValueSet::builder()
        .name("zoom")
        .values(indexmap! {
          "0" => "0",
          "50" => ".5",
          "75" => ".75",
          "90" => ".9",
          "95" => ".95",
          "100" => "1",
          "105" => "1.05",
          "110" => "1.1",
          "125" => "1.25",
          "150" => "1.5"
        })
        .build(),
      ValueSet::builder()
        .name("rotation")
        .values(indexmap! {
          "0" => "0deg",
          "1" => "1deg",
          "2" => "2deg",
          "3" => "3deg",
          "6" => "6deg",
          "12" => "12deg",
          "30" => "30deg",
          "45" => "45deg",
          "90" => "90deg",
          "180" => "180deg",
        })
        .build(),
      ValueSet::builder()
        .name("negative-translation")
        .values(indexmap! {
          "full" => "-100%",
          "0" => "-0px",
          "px" => "-1px",
          "0.5" => "-0.125rem",
          "1" => "-0.25rem",
          "1.5" => "-0.375rem",
          "2" => "-0.5rem",
          "2.5" => "-0.625rem",
          "3" => "-0.75rem",
          "3.5" => "-0.875rem",
          "4" => "-1rem",
          "5" => "-1.25rem",
          "6" => "-1.5rem",
          "7" => "-1.75rem",
          "8" => "-2rem",
          "9" => "-2.25rem",
          "10" => "-2.5rem",
          "11" => "-2.75rem",
          "12" => "-3rem",
          "14" => "-3.5rem",
          "16" => "-4rem",
          "20" => "-5rem",
          "24" => "-6rem",
          "28" => "-7rem",
          "32" => "-8rem",
          "36" => "-9rem",
          "40" => "-10rem",
          "44" => "-11rem",
          "48" => "-12rem",
          "52" => "-13rem",
          "56" => "-14rem",
          "60" => "-15rem",
          "64" => "-16rem",
          "72" => "-18rem",
          "80" => "-20rem",
          "96" => "-24rem",
          "1-of-3" => "-33.333333%",
          "2-of-3" => "-66.666667%",
          "1-of-4" => "-25%",
          "half" => "-50%",
          "3-of-4" => "-75%",
        })
        .build(),
      ValueSet::builder()
        .name("positive-translation")
        .values(indexmap! {
          "full" => "100%",
          "0" => "0px",
          "px" => "1px",
          "0.5" => "0.125rem",
          "1" => "0.25rem",
          "1.5" => "0.375rem",
          "2" => "0.5rem",
          "2.5" => "0.625rem",
          "3" => "0.75rem",
          "3.5" => "0.875rem",
          "4" => "1rem",
          "5" => "1.25rem",
          "6" => "1.5rem",
          "7" => "1.75rem",
          "8" => "2rem",
          "9" => "2.25rem",
          "10" => "2.5rem",
          "11" => "2.75rem",
          "12" => "3rem",
          "14" => "3.5rem",
          "16" => "4rem",
          "20" => "5rem",
          "24" => "6rem",
          "28" => "7rem",
          "32" => "8rem",
          "36" => "9rem",
          "40" => "10rem",
          "44" => "11rem",
          "48" => "12rem",
          "52" => "13rem",
          "56" => "14rem",
          "60" => "15rem",
          "64" => "16rem",
          "72" => "18rem",
          "80" => "20rem",
          "96" => "24rem",
          "1-of-3" => "33.333333%",
          "2-of-3" => "66.666667%",
          "1-of-4" => "25%",
          "half" => "50%",
          "3-of-4" => "75%",
         })
        .build(),
      ValueSet::builder()
        .name("font")
        .values(indexmap! {
          "sans" => "ui-sans-serif,system-ui,-apple-system,BlinkMacSystemFont,\"Segoe UI\",Roboto,\"Helvetica Neue\",Arial,\"Noto Sans\",sans-serif,\"Apple Color Emoji\",\"Segoe UI Emoji\",\"Segoe UI Symbol\",\"Noto Color Emoji\"",
          "serif" => "ui-serif,Georgia,Cambria,\"Times New Roman\",Times,serif",
          "mono" => "ui-monospace,SFMono-Regular,Menlo,Monaco,Consolas,\"Liberation Mono\",\"Courier New\",monospace",
        })
        .build(),
      ValueSet::builder()
        .name("ratio")
        .description("Ratio of the width to the height")
        .values(indexmap! {
          "auto" => "auto",
          "square" => "1",
          "video" => "16 / 9",
          "portrait" => "9 / 16",
          "box" => "4 / 3",
        })
        .build(),
      ValueSet::builder()
        .name("grid-count")
        .description("Column and row counts for grid layouts")
        .values(indexmap! {
          "1" => "1",
          "2" => "2",
          "3" => "3",
          "4" => "4",
          "5" => "5",
          "6" => "6",
          "7" => "7",
          "8" => "8",
          "9" => "9",
          "10" => "10",
          "11" => "11",
          "12" => "12",
        })
        .build(),
      ValueSet::builder()
        .name("grid-size")
        .description("Sizes for grid layouts")
        .values(indexmap! {
          "3xs" => "16rem",
          "2xs" => "18rem",
          "xs" => "20rem",
          "sm" => "24rem",
          "md" => "28rem",
          "lg" => "32rem",
          "xl" => "36rem",
          "2xl" => "42rem",
          "3xl" => "48rem",
          "4xl" => "56rem",
          "5xl" => "64rem",
          "6xl" => "72rem",
          "7xl" => "80rem",
        })
        .build(),
      ValueSet::builder()
        .name("break")
        .description("Different break rules for elements.")
        .values(indexmap! {
          "auto" => "auto",
          "avoid" => "avoid",
          "all" => "all",
          "avoid-page" => "avoid-page",
          "page" => "page",
          "left" => "left",
          "right" => "right",
          "column" => "column",
        })
        .build(),
      ValueSet::builder()
        .name("break-inside")
        .values(indexmap! {
          "auto" => "auto",
          "avoid" => "avoid",
          "avoid-page" => "avoid-page",
          "avoid-column" => "avoid-column",
        })
        .build(),
      ValueSet::builder()
        .name("box-decoration")
        .values(indexmap! {
          "clone" => "clone",
          "slice" => "slice",
        })
        .build(),
      ValueSet::builder()
        .name("box")
        .values(indexmap! {
          "border" => "border-box",
          "content" => "content-box",
        })
        .build(),
      ValueSet::builder()
        .name("display")
        .values(indexmap! {
          "block" => "block",
          "inline-block" => "inline-block",
          "inline" => "inline",
          "flex" => "flex",
          "inline-flex" => "inline-flex",
          "table" => "table",
          "inline-table" => "inline-table",
          "table-caption" => "table-caption",
          "table-cell" => "table-cell",
          "table-column" => "table-column",
          "table-column-group" => "table-column-group",
          "table-footer-group" => "table-footer-group",
          "table-header-group" => "table-header-group",
          "table-row-group" => "table-row-group",
          "table-row" => "table-row",
          "flow-root" => "flow-root",
          "grid" => "grid",
          "inline-grid" => "inline-grid",
          "contents" => "contents",
          "list-item" => "list-item",
          "hidden" => "none",
        })
        .build(),
      ValueSet::builder()
        .name("visibility")
        .values(indexmap! {
          "visible" => "visible",
          "invisible" => "invisible",
          "collapse" => "collapse",
        })
        .build(),
      ValueSet::builder()
        .name("float")
        .values(indexmap! {
          "right" => "right",
          "left" => "left",
          "none" => "none",
        })
        .build(),
      ValueSet::builder()
        .name("clear")
        .values(indexmap! { "both" => "both" })
        .build(),
      ValueSet::builder()
        .name("isolation")
        .values(indexmap! {
          "auto" => "auto",
          "isolate" => "isolate"
        })
        .build(),
      ValueSet::builder()
        .name("object-fit")
        .values(indexmap! {
          "contain" => "contain",
          "cover" => "cover",
          "fill" => "fill",
          "none" => "none",
          "scale-down" => "scale-down",
        })
        .build(),
      ValueSet::builder()
        .name("object-position")
        .values(indexmap! {
          "bottom" => "bottom",
          "center" => "center",
          "left" => "left",
          "left bottom" => "left bottom",
          "left top" => "left top",
          "right" => "right",
          "right bottom" => "right bottom",
          "right top" => "right top",
          "top" => "top",
        })
        .build(),
      ValueSet::builder()
        .name("overflow")
        .values(indexmap! {
          "auto" => "auto",
          "hidden" => "hidden",
          "clip" => "clip",
          "visible" => "visible",
          "scroll" => "scroll",
        })
        .build(),
      ValueSet::builder()
        .name("overscroll")
        .values(indexmap! {
          "auto" => "auto",
          "contain" => "contain",
          "none" => "none",
        })
        .build(),
      ValueSet::builder()
        .name("position")
        .values(indexmap! {
          "static" => "static",
          "fixed" => "fixed",
          "absolute" => "absolute",
          "relative" => "relative",
          "sticky" => "sticky",
        })
        .build(),
      ValueSet::builder()
        .name("blur")
        .values(indexmap! {
        	"default" => "blur(8px)",
          "none" => "blur(0)",
          "sm"	=> "blur(4px)",
          "md"	=> "blur(12px)",
          "lg"	=> "blur(16px)",
          "xl"	=> "blur(24px)",
          "2xl"	=> "blur(40px)",
          "3xl"	=> "blur(64px)",
        })
        .build(),
      ValueSet::builder()
        .name("brightness")
        .values(indexmap! {
          "0" => "brightness(0)",
          "50" => "brightness(.5)",
          "75" => "brightness(.75)",
          "90" => "brightness(.9)",
          "95" => "brightness(.95)",
          "100" => "brightness(1)",
          "105" => "brightness(1.05)",
          "110" => "brightness(1.1)",
          "125" => "brightness(1.25)",
          "150" => "brightness(1.5)",
          "200" => "brightness(2)",
        })
        .build(),
      ValueSet::builder()
        .name("contrast")
        .values(indexmap! {
          "0" => "contrast(0)",
          "50" => "contrast(.5)",
          "75" => "contrast(.75)",
          "100" => "contrast(1)",
          "125" => "contrast(1.25)",
          "150" => "contrast(1.5)",
          "200" => "contrast(2)",
        })
        .build(),
      ValueSet::builder()
        .name("drop-shadow")
        .values(indexmap! {
          "default" => "drop-shadow(0 1px 2px rgb(0 0 0 / 0.1)) drop-shadow(0 1px 1px rgb(0 0 0 / 0.06))",
          "sm" => "drop-shadow(0 1px 1px rgb(0 0 0 / 0.05))",
          "md" => "drop-shadow(0 4px 3px rgb(0 0 0 / 0.07)) drop-shadow(0 2px 2px rgb(0 0 0 / 0.06))",
          "lg" => "drop-shadow(0 10px 8px rgb(0 0 0 / 0.04)) drop-shadow(0 4px 3px rgb(0 0 0 / 0.1))",
          "xl" => "drop-shadow(0 20px 13px rgb(0 0 0 / 0.03)) drop-shadow(0 8px 5px rgb(0 0 0 / 0.08))",
          "2xl" => "drop-shadow(0 25px 25px rgb(0 0 0 / 0.15))",
          "none" => "drop-shadow(0 0 #0000)",
        })
        .build(),
      ValueSet::builder()
        .name("grayscale")
        .values(indexmap! {
          "default" => "grayscale(100%)",
          "0" => "grayscale(0)",
        })
        .build(),
      ValueSet::builder()
        .name("invert")
        .values(indexmap! {
          "default" => "invert(100%)",
          "0" => "invert(0)",
        })
        .build(),
      ValueSet::builder()
        .name("sepia")
        .values(indexmap! {
          "default" => "sepia(100%)",
          "0" => "sepia(0)",
        })
        .build(),
      ValueSet::builder()
        .name("hue-rotate")
        .values(indexmap! {
          "0" => "hue-rotate(0deg)",
          "15" => "hue-rotate(15deg)",
          "30" => "hue-rotate(30deg)",
          "60" => "hue-rotate(60deg)",
          "90" => "hue-rotate(90deg)",
          "180" => "hue-rotate(180deg)",
        })
        .build(),
      ValueSet::builder()
        .name("saturate")
        .values(indexmap! {
          "0" => "saturate(0)",
          "50" => "saturate(.5)",
          "100" => "saturate(1)",
          "150" => "saturate(1.5)",
          "200" => "saturate(2)",
        })
        .build(),
      ValueSet::builder()
        .name("filter")
        .values(indexmap! { "none" => "none" })
        .build(),
      ValueSet::builder()
        .name("flex-direction")
        .values(indexmap! {
          "row" => "row",
          "row-reverse" => "row-reverse",
          "col" => "column",
          "col-reverse" => "column-reverse",
        })
        .build(),
      ValueSet::builder()
        .name("flex-wrap")
        .values(indexmap! {
          "wrap"	=> "wrap",
          "wrap-reverse"	=> "wrap-reverse",
          "nowrap"	=> "nowrap",
        })
        .build(),
      ValueSet::builder()
        .name("flex")
        .values(indexmap! {
          "1" => "1 1 0%",
          "auto" => "1 1 auto",
          "initial" => "0 1 auto",
          "none" => "none",
        })
        .build(),
      ValueSet::builder()
        .name("flex-grow")
        .values(indexmap! {
          "1" => "1",
          "0" => "0",
        })
        .build(),
      ValueSet::builder()
        .name("flex-shrink")
        .values(indexmap! {
          "1" => "1",
          "0" => "0",
        })
        .build(),
      ValueSet::builder()
        .name("order")
        .values(indexmap! {
          "1" => "1",
          "2" => "2",
          "3" => "3",
          "4" => "4",
          "5" => "5",
          "6" => "6",
          "7" => "7",
          "8" => "8",
          "9" => "9",
          "10" => "10",
          "11" => "11",
          "12" => "12",
          "first" => "-9999",
          "last" => "9999",
          "none" => "0",
        })
        .build(),
      ValueSet::builder()
        .name("negative-order")
        .values(indexmap! {
          "-1" => "-1",
          "-2" => "-2",
          "-3" => "-3",
          "-4" => "-4",
          "-5" => "-5",
          "-6" => "-6",
          "-7" => "-7",
          "-8" => "-8",
          "-9" => "-9",
          "-10" => "-10",
          "-11" => "-11",
          "-12" => "-12",
        })
        .build(),
      ValueSet::builder()
        .name("grid-template")
        .values(indexmap! {
          "1" => "repeat(1, minmax(0, 1fr))",
          "2" => "repeat(2, minmax(0, 1fr))",
          "3" => "repeat(3, minmax(0, 1fr))",
          "4" => "repeat(4, minmax(0, 1fr))",
          "5" => "repeat(5, minmax(0, 1fr))",
          "6" => "repeat(6, minmax(0, 1fr))",
          "7" => "repeat(7, minmax(0, 1fr))",
          "8" => "repeat(8, minmax(0, 1fr))",
          "9" => "repeat(9, minmax(0, 1fr))",
          "10" => "repeat(10, minmax(0, 1fr))",
          "11" => "repeat(11, minmax(0, 1fr))",
          "12" => "repeat(12, minmax(0, 1fr))",
          "none" => "none",
        })
        .build(),
      ValueSet::builder()
        .name("grid-span")
        .values(indexmap! {
          "auto" =>	"auto",
          "1" =>	"span 1 / span 1",
          "2" =>	"span 2 / span 2",
          "3" =>	"span 3 / span 3",
          "4" =>	"span 4 / span 4",
          "5" =>	"span 5 / span 5",
          "6" =>	"span 6 / span 6",
          "7" =>	"span 7 / span 7",
          "8" =>	"span 8 / span 8",
          "9" =>	"span 9 / span 9",
          "10" =>	"span 10 / span 10",
          "11" =>	"span 11 / span 11",
          "12" =>	"span 12 / span 12",
          "full" =>	"1 / -1",
        })
        .build(),
      ValueSet::builder()
        .name("grid-start-end")
        .values(indexmap! {
          "1" =>	"1",
          "2" =>	"2",
          "3" =>	"3",
          "4" =>	"4",
          "5" =>	"5",
          "6" =>	"6",
          "7" =>	"7",
          "8" =>	"8",
          "9" =>	"9",
          "10" =>	"10",
          "11" =>	"11",
          "12" =>	"12",
          "13" =>	"13",
          "auto" =>	"auto",
        })
        .build(),
    ]
  };
}
