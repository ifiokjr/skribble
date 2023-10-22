use std::path::Path;

use globset::Glob;
use globset::GlobSet;
use globset::GlobSetBuilder;

use crate::AnyError;
use crate::AnyResult;
use crate::StringList;

/// A pair of glob sets, one for including files and one for excluding files.
/// This is generated
pub struct GlobSetPair {
	include_set: GlobSet,
	exclude_set: GlobSet,
}

impl GlobSetPair {
	/// Check if the given file path matches the glob rules.
	pub fn is_match(&self, file_path: impl AsRef<Path>) -> bool {
		let file_path = file_path.as_ref();
		!self.exclude_set.is_match(file_path) && self.include_set.is_match(file_path)
	}
}

impl TryFrom<&Vec<String>> for GlobSetPair {
	type Error = AnyError;

	fn try_from(glob_rules: &Vec<String>) -> AnyResult<Self> {
		let mut include_builder = GlobSetBuilder::new();
		let mut exclude_builder = GlobSetBuilder::new();

		for rule in glob_rules {
			if let Some(stripped) = rule.strip_prefix('!') {
				let glob = Glob::new(stripped)?;
				exclude_builder.add(glob);
				continue;
			}

			let glob = Glob::new(rule)?;
			include_builder.add(glob);
		}

		let include_set = include_builder.build()?;
		let exclude_set = exclude_builder.build()?;
		let glob_set_pair = Self {
			include_set,
			exclude_set,
		};

		Ok(glob_set_pair)
	}
}

impl TryFrom<&StringList> for GlobSetPair {
	type Error = AnyError;

	fn try_from(glob_rules: &StringList) -> AnyResult<Self> {
		let value: &Vec<String> = glob_rules.as_ref();
		Self::try_from(value)
	}
}
