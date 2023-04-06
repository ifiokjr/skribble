use std::path::PathBuf;

use skribble_color::ColorError;

use crate::AnyError;

pub type Result<T> = core::result::Result<T, Error>;

#[non_exhaustive]
#[derive(thiserror::Error, Debug)]
pub enum Error {
  #[error("an unknown skribble error has occurred, please create an issue on GitHub")]
  Unknown,
  #[error("could not load the current working directory")]
  CwdLookupError,
  /// An invalid configuration object was provided.
  #[error("invalid configuration object provided")]
  InvalidConfig(#[source] serde_json::Error),
  /// An invalid configuration object was provided.
  #[error("the provided configuration object could not be serialized")]
  CouldNotSerializeConfig(#[source] serde_json::Error),
  #[error(
    "something went wrong with the plugin: `{id}` while generating the config in `mutate_config`"
  )]
  PluginMutateConfigError {
    id: String,
    #[source]
    source: AnyError,
  },
  #[error("something went wrong with the plugin: `{id}` while reading options in `read_options`")]
  PluginReadConfigError {
    id: String,
    #[source]
    source: AnyError,
  },
  #[error("something went wrong with the plugin: `{id}` while generating code in `generate_code`")]
  PluginGenerateCodeError {
    id: String,
    #[source]
    source: AnyError,
  },
  #[error("something went wrong while generating css in the plugin: `{id}`")]
  PluginScanCodeError {
    id: String,
    #[source]
    source: AnyError,
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
  #[error("something went wrong while scanning the files")]
  FileScanError(#[source] AnyError),
  #[error("something went wrong while reading the file: `{0}`")]
  FileReadError(String),
  #[error("something went wrong while writing the file: `{0}`")]
  FileWriteError(PathBuf),
  #[error("generating the css failed")]
  GenerateCssError(#[source] AnyError),
  #[error("generating the css with lightning css failed")]
  LightningCssError,
}
