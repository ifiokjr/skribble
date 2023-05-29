use vfs::VfsPath;

use crate::AnyResult;
use crate::GlobSetPair;

/// Find all files in the given directory that match the given glob rules.
pub(crate) fn walk_directory(fs: &VfsPath, glob_rules: &Vec<String>) -> AnyResult<Vec<VfsPath>> {
  let glob_set_pair = GlobSetPair::try_from(glob_rules)?;

  let entries = fs
    .walk_dir()?
    .filter_map(|entry| entry.ok())
    .filter(|entry| entry.is_file().unwrap_or(false))
    .filter(|entry| glob_set_pair.is_match(entry.as_str()))
    .collect::<Vec<_>>();

  Ok(entries)
}
