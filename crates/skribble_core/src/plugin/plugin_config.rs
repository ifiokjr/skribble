use serde::Deserialize;
use serde::Serialize;

use crate::config::*;

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PluginConfig {
  pub atoms: Atoms,
  pub aliases: Aliases,
  pub classes: NamedClasses,
  pub css_chunks: CssChunks,
  pub keyframes: Keyframes,
  pub layers: Layers,
  pub media_queries: MediaQueries,
  pub modifiers: Modifiers,
  pub palette: Palette,
  pub value_sets: ValueSets,
  pub variables: CssVariables,
}
