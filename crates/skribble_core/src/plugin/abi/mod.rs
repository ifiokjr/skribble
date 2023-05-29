#![deny(clippy::all)]
#![forbid(clippy::indexing_slicing)]

pub use abi_stable::*;
use derive_more::Deref;
use derive_more::DerefMut;
pub use external_types::RawValueBox;
pub use external_types::RawValueRef;
pub use plugin_error::*;
pub use sabi_types::RMut;
pub use sabi_types::VersionStrings;
use serde::Deserialize;
use serde::Serialize;
pub use std_types::*;
use typed_builder::TypedBuilder;
use StableAbi;

/// The root module of a`plugin` dynamic library.
#[repr(C)]
#[derive(StableAbi)]
#[sabi(kind(Prefix(prefix_ref = AbiPluginModuleRef)))]
#[sabi(missing_field(panic))]
pub struct AbiPluginModule {}

/// Used to read the data for each plugin.
#[repr(C)]
#[derive(Clone, Debug, Deserialize, Serialize, StableAbi, TypedBuilder)]
pub struct AbiPluginData {
  /// Store the globs for files supported by the plugin. This is only relevant
  /// if the plugin is scanning files.
  #[serde(default)]
  #[builder(default, setter(into))]
  pub globs: RVec<RString>,
  /// Store the id of the plugin. This should be globally unique and if the
  /// crate is published it should be the published crate name of the plugin.
  #[builder(setter(into))]
  pub id: RString,
  /// Store a readable name of the plugin. This is used for error messages and
  #[serde(default)]
  #[builder(default, setter(into))]
  pub name: ROption<RString>,
  /// Store the markdown description of the plugin.
  #[serde(default)]
  #[builder(default, setter(into))]
  pub description: ROption<RString>,
  /// Store the version of the plugin.
  #[serde(default)]
  #[builder(default, setter(into))]
  pub version: ROption<RString>,
}

impl PartialEq for AbiPluginData {
  fn eq(&self, other: &Self) -> bool {
    self.id == other.id && self.version == other.version
  }
}

impl Eq for AbiPluginData {}

#[sabi_trait]
pub trait AbiPlugin {
  /// The configuration provided to the plugin.
  type Configuration: Serialize + Clone + Send + Sync;
  /// The state produced by the plugin which is stored at runtime and used to
  /// speed up performance.
  type State: Serialize + Clone + Send + Sync;

  fn resolve_config(&mut self) -> PluginResult<Self::Configuration>;
}

pub struct AbiOptions {}

#[derive(Deref, DerefMut)]
pub struct OrderedHashMap<K, V>(RHashMap<K, (u32, V)>);

mod plugin_error;
