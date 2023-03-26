use crate::ColorError;

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

  #[error(
    "something went wrong with the plugin: `{id}` while generating the config in `mutate_config`"
  )]
  PluginMutateConfigError {
    id: String,
    #[source]
    source: Box<dyn std::error::Error>,
  },

  #[error("something went wrong with the plugin: `{id}` while reading options in `read_options`")]
  PluginReadConfigError {
    id: String,
    #[source]
    source: Box<dyn std::error::Error>,
  },

  #[error("something went wrong with the plugin: `{id}` while generating code in `generate_code`")]
  PluginGenerateCodeError {
    id: String,
    #[source]
    source: Box<dyn std::error::Error>,
  },

  #[error(
    "the runner has not generated the merged config yet, make sure to run `run()` before \
     `generate()`"
  )]
  RunnerNotSetup,

  #[error("color conversion error")]
  Color(#[from] ColorError),

  #[error("no initial color value was provided for css variable `{0}")]
  InvalidCssVariable(String),
}
