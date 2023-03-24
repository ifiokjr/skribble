pub type AnyError = Box<dyn std::error::Error>;
pub type AnyEmptyResult = Result<(), AnyError>;
pub type AnyResult<T> = Result<T, AnyError>;
