use indexmap::indexmap;
use lazy_static::lazy_static;
use skribble_core::CalcSymbol;
use skribble_core::ColorProperty;
use skribble_core::Group;
use skribble_core::Transformer;

lazy_static! {
  pub(crate) static ref TRANSFORMERS: Vec<Group<Transformer>> = vec![
    Group::builder()
      .name("important")
      .description("Make a property important.")
      .items(vec![
        Transformer::builder()
          .name("important")
          .description(
            "Append `!important` to every property in the transformed atom / named class."
          )
          .transformation("& !important")
          .scope("*")
          .recipient("property")
          .build(),
      ])
      .build(),
    Group::builder()
      .name("alpha")
      .description(
        "Set color alpha. For custom values the value should be between 0 and 1. Don't use \
         percentages."
      )
      .items(vec![
        Transformer::builder()
          .name("alpha")
          .description(
            "Sets the alpha value of a color to this value. Should be a value between 0 and 1."
          )
          .transformation(ColorProperty::Alpha(CalcSymbol::Set))
          .values(indexmap! {
            "000" => "0",
            "005" => "0.05",
            "010" => "0.1",
            "015" => "0.15",
            "020" => "0.2",
            "025" => "0.25",
            "030" => "0.3",
            "040" => "0.4",
            "050" => "0.5",
            "060" => "0.6",
            "070" => "0.7",
            "075" => "0.75",
            "080" => "0.8",
            "090" => "0.9",
            "095" => "0.95",
            "100" => "1",
          })
          .scope("color")
          .recipient("value")
          .build(),
        Transformer::builder()
          .name("opaquify")
          .description("Increase the opacity of the color.")
          .transformation(ColorProperty::Alpha(CalcSymbol::Add))
          .values(indexmap! {
            "001" => "0.01",
            "002" => "0.02",
            "003" => "0.03",
            "004" => "0.04",
            "005" => "0.05",
            "010" => "0.1",
            "015" => "0.15",
            "020" => "0.2",
            "025" => "0.25",
            "030" => "0.3",
            "040" => "0.4",
            "050" => "0.5",
            "060" => "0.6",
            "070" => "0.7",
            "075" => "0.75",
            "080" => "0.8",
            "090" => "0.9",
            "095" => "0.95",
            "100" => "1",
          })
          .scope("color")
          .recipient("value")
          .build(),
        Transformer::builder()
          .name("transparentize")
          .description("Decrease the opacity of the color.")
          .transformation(ColorProperty::Alpha(CalcSymbol::Subtract))
          .values(indexmap! {
            "001" => "0.01",
            "002" => "0.02",
            "003" => "0.03",
            "004" => "0.04",
            "005" => "0.05",
            "010" => "0.1",
            "015" => "0.15",
            "020" => "0.2",
            "025" => "0.25",
            "030" => "0.3",
            "040" => "0.4",
            "050" => "0.5",
            "060" => "0.6",
            "070" => "0.7",
            "075" => "0.75",
            "080" => "0.8",
            "090" => "0.9",
            "095" => "0.95",
            "100" => "1",
          })
          .scope("color")
          .recipient("value")
          .build(),
      ])
      .build(),
    Group::builder()
      .name("lightness")
      .description("Lighten and darken.")
      .items(vec![
        Transformer::builder()
          .name("lightness")
          .description("Set the lightness of the color.")
          .transformation(ColorProperty::Lightness(CalcSymbol::Set))
          .values(indexmap! {
            "000" => "0%",
            "001" => "1%",
            "002" => "2%",
            "003" => "3%",
            "004" => "4%",
            "005" => "5%",
            "010" => "10%",
            "015" => "15%",
            "020" => "20%",
            "025" => "25%",
            "030" => "30%",
            "035" => "35%",
            "040" => "40%",
            "045" => "45%",
            "050" => "50%",
            "055" => "55%",
            "060" => "60%",
            "065" => "65%",
            "070" => "70%",
            "075" => "75%",
            "080" => "80%",
            "085" => "85%",
            "090" => "90%",
            "095" => "95%",
            "100" => "100%",
          })
          .scope("color")
          .recipient("value")
          .build(),
        Transformer::builder()
          .name("lighten")
          .description("Lighten the color by the provided percentage.")
          .transformation(ColorProperty::Lightness(CalcSymbol::Add))
          .values(indexmap! {
            "001" => "1%",
            "002" => "2%",
            "003" => "3%",
            "004" => "4%",
            "005" => "5%",
            "010" => "10%",
            "015" => "15%",
            "020" => "20%",
            "025" => "25%",
            "030" => "30%",
            "035" => "35%",
            "040" => "40%",
            "045" => "45%",
            "050" => "50%",
            "055" => "55%",
            "060" => "60%",
            "065" => "65%",
            "070" => "70%",
            "075" => "75%",
            "080" => "80%",
            "085" => "85%",
            "090" => "90%",
            "095" => "95%",
            "100" => "100%",
          })
          .scope("color")
          .recipient("value")
          .build(),
        Transformer::builder()
          .name("darken")
          .description("Darken the color by the provided percentage.")
          .transformation(ColorProperty::Lightness(CalcSymbol::Subtract))
          .values(indexmap! {
            "001" => "1%",
            "002" => "2%",
            "003" => "3%",
            "004" => "4%",
            "005" => "5%",
            "010" => "10%",
            "015" => "15%",
            "020" => "20%",
            "025" => "25%",
            "030" => "30%",
            "035" => "35%",
            "040" => "40%",
            "045" => "45%",
            "050" => "50%",
            "055" => "55%",
            "060" => "60%",
            "065" => "65%",
            "070" => "70%",
            "075" => "75%",
            "080" => "80%",
            "085" => "85%",
            "090" => "90%",
            "095" => "95%",
            "100" => "100%",
          })
          .scope("color")
          .recipient("value")
          .build(),
      ])
      .build(),
  ];
}
