pub type Result<T> = core::result::Result<T, Error>;

// Prevents a breaking change when adding a new error type
#[non_exhaustive]
#[derive(thiserror::Error, Debug)]
pub enum Error {
  #[error("an unknown skribble error has occurred, please create an issue on GitHub")]
  Unknown,

  /// An invalid configuration object was provided.
  #[error("invalid configuration object provided")]
  InvalidConfig {
    #[source]
    source: serde_json::Error,
  },

  /// An invalid configuration object was provided.
  #[error("the provided configuration object could not be serialized")]
  CouldNotSerializeConfig {
    #[source]
    source: serde_json::Error,
  },

  #[error("something went wrong with the plugin: `{id}` while running `mutate_config`")]
  PluginMutateConfigError {
    id: String,
    #[source]
    source: Box<dyn std::error::Error>,
  },

  #[error("something went wrong with the plugin: `{id}` while running `finalize_config`")]
  PluginReadConfigError {
    id: String,
    #[source]
    source: Box<dyn std::error::Error>,
  },
}
