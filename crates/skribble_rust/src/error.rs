#[non_exhaustive]
#[derive(thiserror::Error, Debug)]
pub enum Error {
  #[error("invalid method name: `{0}`")]
  InvalidMethodName(String),
}
