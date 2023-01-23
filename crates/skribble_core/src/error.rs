pub type Result<T> = core::result::Result<T, Error>;

// Prevents a breaking change when adding a new error type
#[non_exhaustive]
#[derive(thiserror::Error, Debug)]
pub enum Error {
  #[error("an unknown skribble error has occurred")]
  Unknown,

  /// An invalid configuration object was provided.
  #[error("invalid configuration object provided")]
  InvalidConfig {
    #[source]
    source: serde_json::Error,
  },
}
