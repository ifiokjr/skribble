use std::env::current_dir;
use std::path::Path;
use std::sync::Arc;
use std::sync::Mutex;

use lightningcss::stylesheet::MinifyOptions;
use lightningcss::stylesheet::ParserOptions;
use lightningcss::stylesheet::PrinterOptions;
use lightningcss::stylesheet::StyleSheet;
use lightningcss::stylesheet::ToCssResult;
use vfs::PhysicalFS;
use vfs::VfsPath;

use super::generate_merged_config;
use super::walk_directory;
use super::RunnerConfig;
use crate::Classes;
use crate::Error;
use crate::GeneratedFiles;
use crate::Options;
use crate::PluginConfig;
use crate::Result;
use crate::StyleConfig;
use crate::ToSkribbleCss;
use crate::WrappedPlugin;

pub struct SkribbleRunner {
  options: Arc<Options>,
  base_config: Arc<PluginConfig>,
  plugins: Arc<Mutex<Vec<WrappedPlugin>>>,
  config: Option<RunnerConfig>,
  fs: Arc<VfsPath>,
}

impl SkribbleRunner {
  /// Create a new [`SkribbleRunner`].
  ///
  /// # Arguments
  ///
  /// * `config` - The configuration for the runner.
  /// * `cwd` - The current working directory.
  /// * `vfs` - The virtual file system to use. If `None` is provided, it
  ///   defaults to the physical file system. This is mainly used for testing.
  pub fn new(config: StyleConfig, cwd: impl AsRef<Path>, vfs: Option<VfsPath>) -> Self {
    let cwd = cwd.as_ref();
    let (mut options, wrapped_config, mut plugins) = config.into_wrapped_config();
    options.root = options.root.join(cwd);
    let options = Arc::new(options);
    let config = Arc::new(wrapped_config);

    // Extract the plugins from the config and sort them by priority.
    plugins.sort_by_priority();
    let plugins = Arc::new(Mutex::new(plugins.extract_plugins()));
    let vfs = vfs.unwrap_or_else(|| PhysicalFS::new(cwd).into());
    let fs = Arc::new(vfs);

    Self {
      options,
      base_config: config,
      plugins,
      config: None,
      fs,
    }
  }

  /// Create a new [`SkribbleRunner`] with the current directory automatically
  /// inferred.
  pub fn try_new(config: StyleConfig) -> Result<Self> {
    let cwd = current_dir().map_err(|_| Error::CwdLookupError)?;
    Ok(Self::new(config, cwd, None))
  }

  pub fn get_options(&self) -> &Options {
    self.options.as_ref()
  }

  /// Run the plugins to mutate the config and get the transformed config which
  /// is used.
  pub fn initialize(&mut self) -> Result<&RunnerConfig> {
    self.provide_options_to_plugins()?;
    let config_from_plugins = self.generate_plugin_config()?;
    self.merge(config_from_plugins);

    self.config.as_ref().ok_or(Error::RunnerNotSetup)
  }

  /// Provide options to the plugins.
  fn provide_options_to_plugins(&mut self) -> Result<()> {
    let options = self.options.as_ref();
    let mut plugins = self.plugins.lock().unwrap();

    for plugin in plugins.iter_mut() {
      plugin.read_options(options).map_err(|source| {
        Error::PluginReadConfigError {
          id: plugin.data().id.clone(),
          source,
        }
      })?;
    }

    Ok(())
  }

  /// Run the generate functions on all plugins with the provided merged
  /// configuration.
  pub fn generate(&self) -> Result<GeneratedFiles> {
    let Some(ref config) = self.config else {
      return Err(Error::RunnerNotSetup);
    };

    let mut plugins = self.plugins.lock().unwrap();
    let mut generated_files = GeneratedFiles::default();

    for plugin in plugins.iter_mut() {
      let generated = plugin.generate_code(config).map_err(|source| {
        Error::PluginGenerateCodeError {
          id: plugin.data().id.clone(),
          source,
        }
      })?;

      generated_files.merge(generated);
    }

    Ok(generated_files)
  }

  pub fn scan(&self) -> Result<ToCssResult> {
    let Some(ref config) = self.config else {
      return Err(Error::RunnerNotSetup);
    };

    let cwd = &config.options().root;

    let entries =
      walk_directory(self.fs.as_ref(), cwd, &self.options.files).map_err(Error::FileScanError)?;

    let mut plugins = self.plugins.lock().unwrap();
    let mut classes = Classes::default();

    for entry in entries.iter() {
      let path = entry.as_str();
      let contents = entry
        .read_to_string()
        .map_err(|_| Error::FileReadError(entry.as_str().to_string()))?;
      for plugin in plugins.iter_mut() {
        let scanned = plugin
          .scan_code(config, path, &contents)
          .map_err(|source| {
            Error::PluginScanCodeError {
              id: plugin.data().id.clone(),
              source,
            }
          })?;

        classes.merge(scanned);
      }
    }

    let parser_options = ParserOptions {
      filename: "skribble.css".into(),
      ..Default::default()
    };
    let printer_options = PrinterOptions {
      minify: self.options.minify,
      ..Default::default()
    };
    let css = classes
      .to_skribble_css(config)
      .map_err(Error::GenerateCssError)?;
    let css_reference: &'static mut str = Box::leak(css.into_boxed_str());
    let mut stylesheet =
      StyleSheet::parse(css_reference, parser_options).map_err(|_| Error::LightningCssError)?;
    stylesheet
      .minify(MinifyOptions::default())
      .map_err(|_| Error::LightningCssError)?;
    let result = stylesheet
      .to_css(printer_options)
      .map_err(|_| Error::LightningCssError)?;

    Ok(result)
  }

  fn generate_plugin_config(&self) -> Result<PluginConfig> {
    let mut plugin_config = PluginConfig::default();
    let mut plugins = self.plugins.lock().unwrap();

    for plugin in plugins.iter_mut() {
      plugin
        .mutate_config(&mut plugin_config, &self.options)
        .map_err(|source| {
          Error::PluginMutateConfigError {
            id: plugin.data().id.clone(),
            source,
          }
        })?;
    }

    Ok(plugin_config)
  }

  fn merge(&mut self, plugin_config: PluginConfig) {
    let config = generate_merged_config(plugin_config, self.options.clone(), &self.base_config);

    self.config = Some(config);
  }

  pub fn write_files(&self, files: &GeneratedFiles) -> Result<()> {
    for file in files.iter() {
      let entry = self
        .fs
        .join(file.path.to_string_lossy())
        .map_err(|_| Error::FileWriteError(file.path.clone()))?;
      let mut writer = entry
        .create_file()
        .map_err(|_| Error::FileWriteError(file.path.clone()))?;
      write!(writer, "{}", file.content).map_err(|_| Error::FileWriteError(file.path.clone()))?;
    }

    Ok(())
  }

  pub fn write_css(&self, css: &ToCssResult) -> Result<()> {
    let css_file = self.options.output.to_string_lossy();
    let entry = self
      .fs
      .join(&css_file)
      .map_err(|_| Error::FileWriteError(css_file.as_ref().into()))?;
    let mut writer = entry
      .create_file()
      .map_err(|_| Error::FileWriteError(css_file.as_ref().into()))?;
    write!(writer, "{}", css.code).map_err(|_| Error::FileWriteError(css_file.as_ref().into()))?;

    Ok(())
  }
}
