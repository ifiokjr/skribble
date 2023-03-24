use crate::config::*;

#[derive(Clone, Default)]
pub struct WrappedPluginConfig {
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
  pub additional_fields: AdditionalFields,
}
