use serde::Deserialize;
use serde::Serialize;

use crate::config::*;

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PluginConfig {
  pub layers: Layers,
  pub keyframes: Keyframes,
  pub variables: CssVariables,
  pub media_queries: MediaQueries,
  pub modifiers: Modifiers,
  pub atoms: Atoms,
  pub classes: NamedClasses,
  pub palette: Palette,
  pub value_sets: ValueSets,
  pub groups: VariableGroups,
}
