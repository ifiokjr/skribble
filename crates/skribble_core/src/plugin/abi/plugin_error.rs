use abi_stable::std_types::RResult;
use abi_stable::std_types::RString;
use abi_stable::StableAbi;

#[repr(u8)]
#[derive(thiserror::Error, Debug, StableAbi)]
pub enum PluginError {
	#[error("an unknown plugin error has occurred for the plugin: `{0}`")]
	Unknown(RString),
}

pub type PluginResult<T> = RResult<T, PluginError>;
