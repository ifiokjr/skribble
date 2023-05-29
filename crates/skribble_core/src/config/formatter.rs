use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

use indexmap::indexmap;
use indexmap::IndexMap;
use serde::Deserialize;
use serde::Serialize;
use typed_builder::TypedBuilder;

use crate::AnyEmptyResult;
use crate::GeneratedFile;
use crate::GeneratedFiles;
use crate::GlobSetPair;
use crate::StringList;

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct Formatter {
  /// The formatter command.
  /// e.g. `dprint`
  #[builder(setter(into))]
  pub command: String,
  /// The formatter arguments.
  /// e.g. `["fmt"]`
  #[builder(default, setter(into))]
  pub args: StringList,
  /// The file glob patters for which this formatter will be applied. Use `!` to
  /// negate a match.
  #[builder(setter(into))]
  pub globs: StringList,
}

impl Formatter {
  pub fn format(&self, files: &mut GeneratedFiles) -> AnyEmptyResult {
    let tmpdir = env::temp_dir().join("__skribble_formatted_files__");
    fs::create_dir_all(&tmpdir)?;
    let globset_pair = GlobSetPair::try_from(&self.globs)?;
    let mut files_to_format: IndexMap<&GeneratedFile, PathBuf> = indexmap! {};
    let cloned_files = files.clone();

    for file in cloned_files.iter() {
      let hex_name = hex::encode(file.path.to_string_lossy().to_string());
      let mut tmpfile = tmpdir.join(hex_name);

      if !globset_pair.is_match(&file.path) {
        continue;
      }

      let Some(file_name) = file.path.file_name().and_then(|v| v.to_str()) else {
        continue;
      };

      tmpfile.set_file_name(file_name);

      fs::write(&tmpfile, &file.content)?;
      files_to_format.insert(file, tmpfile);
    }

    let file_args = files_to_format
      .iter()
      .filter_map(|(_, path)| path.to_str())
      .collect::<Vec<_>>();
    let _ = Command::new(&self.command)
      .args(self.args.iter())
      .args(file_args)
      .output()?;

    for (file, tmpfile) in files_to_format {
      let content = fs::read_to_string(&tmpfile)?;
      let generated = file.with_content(content);
      files.replace(generated);
    }

    fs::remove_dir_all(&tmpdir)?;

    Ok(())
  }
}
