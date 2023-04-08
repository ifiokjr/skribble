use globset::Glob;
use globset::GlobSet;
use globset::GlobSetBuilder;
use vfs::VfsPath;

use crate::AnyResult;

fn is_match(file_path: impl AsRef<str>, include_set: &GlobSet, exclude_set: &GlobSet) -> bool {
  let file_path = file_path.as_ref();
  !exclude_set.is_match(file_path) && include_set.is_match(file_path)
}

/// Find all files in the given directory that match the given glob rules.
pub(crate) fn walk_directory(fs: &VfsPath, glob_rules: &Vec<String>) -> AnyResult<Vec<VfsPath>> {
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

  let entries = fs
    .walk_dir()?
    .filter_map(|entry| entry.ok())
    .filter(|entry| entry.is_file().unwrap_or(false))
    .filter(|entry| is_match(entry.as_str(), &include_set, &exclude_set))
    .collect::<Vec<_>>();
  // .filter(|entry| entry.)

  Ok(entries)
}
