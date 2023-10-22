/// Allows you to pull the version from your Cargo.toml at compile time as
/// `MAJOR.MINOR.PATCH_PKGVERSION_PRE`
///
/// # Examples
///
/// ```no_run
/// # use skribble_core::crate_version;
/// let m = crate_version!();
/// ```
///
/// This is taken from [clap](https://docs.rs/clap_builder/4.2.1/src/clap_builder/macros.rs.html#16).
#[macro_export]
macro_rules! crate_version {
	() => {
		env!("CARGO_PKG_VERSION")
	};
}
