use std::path::Path;

use globset::Glob;
use globset::GlobSet;
use globset::GlobSetBuilder;
use walkdir::DirEntry;
use walkdir::WalkDir;

use crate::AnyResult;

fn is_match(entry: &DirEntry, include_set: &GlobSet, exclude_set: &GlobSet) -> bool {
  entry
    .path()
    .to_str()
    .map(|file_name| !exclude_set.is_match(file_name) && include_set.is_match(file_name))
    .unwrap_or(false)
}

/// Find all files in the given directory that match the given glob rules.
pub(crate) fn walk_directory(
  path: impl AsRef<Path>,
  glob_rules: &Vec<String>,
) -> AnyResult<Vec<DirEntry>> {
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

  let entries = WalkDir::new(path)
    .into_iter()
    .filter_map(Result::ok)
    .filter(|entry| entry.file_type().is_file())
    .filter(|entry| is_match(entry, &include_set, &exclude_set))
    .collect::<Vec<_>>();

  Ok(entries)
}
