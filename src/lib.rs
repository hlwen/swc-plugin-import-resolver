use globset::Glob;
use regex::Regex;
use serde::Deserialize;
use swc_core::{
  ecma::{
    ast::{ExportAll, ImportDecl, NamedExport, Program},
    visit::{VisitMut, VisitMutWith},
  },
  plugin::{plugin_transform, proxies::TransformPluginProgramMetadata},
};

#[derive(Debug, Default, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
struct Config {
  aliases: Option<Vec<String>>,
  #[serde(default = "default_extension")]
  extension: String,
  #[serde(default)]
  dir_index: Option<Vec<String>>,
  #[serde(default)]
  dir_index_patterns: Option<Vec<String>>,
  #[serde(default)]
  skip: Option<Vec<String>>,
  #[serde(default = "default_skip_extensions")]
  skip_extensions: Vec<String>,
}

fn default_skip_extensions() -> Vec<String> {
  vec![
    ".js".to_string(),
    ".mjs".to_string(),
    ".cjs".to_string(),
    ".json".to_string(),
    ".css".to_string(),
    ".png".to_string(),
    ".svg".to_string(),
    ".jpg".to_string(),
    ".jpeg".to_string(),
    ".gif".to_string(),
    ".webp".to_string(),
    ".woff".to_string(),
    ".woff2".to_string(),
    ".ttf".to_string(),
    ".eot".to_string(),
    ".otf".to_string(),
    ".mp3".to_string(),
    ".mp4".to_string(),
    ".wav".to_string(),
    ".ogg".to_string(),
    ".webm".to_string(),
    ".pdf".to_string(),
    ".zip".to_string(),
    ".tar".to_string(),
    ".gz".to_string(),
    ".bz2".to_string(),
    ".7z".to_string(),
    ".rar".to_string(),
    ".exe".to_string(),
    ".dll".to_string(),
    ".so".to_string(),
    ".dylib".to_string(),
    ".node".to_string(),
    ".wasm".to_string(),
    ".map".to_string(),
  ]
}

fn default_extension() -> String {
  ".js".to_string()
}

pub struct TransformVisitor {
  aliases: Option<Vec<String>>,
  extension: String,
  dir_index: Option<Vec<String>>,
  dir_index_patterns: Option<Vec<Glob>>,
  skip: Option<Vec<Glob>>,
  skip_extensions: Vec<String>,
}

impl TransformVisitor {
  pub fn new() -> Self {
    TransformVisitor {
      aliases: None,
      extension: ".js".to_string(),
      dir_index: None,
      dir_index_patterns: None,
      skip: None,
      skip_extensions: default_skip_extensions(),
    }
  }

  pub fn set_config(
    &mut self,
    aliases: Option<Vec<String>>,
    extension: String,
    dir_index: Option<Vec<String>>,
    dir_index_patterns: Option<Vec<Glob>>,
    skip: Option<Vec<Glob>>,
    skip_extensions: Vec<String>,
  ) {
    self.aliases = aliases;
    self.extension = extension;
    self.dir_index = dir_index;
    self.dir_index_patterns = dir_index_patterns;
    self.skip = skip;
    self.skip_extensions = skip_extensions;
  }
}

impl VisitMut for TransformVisitor {
  fn visit_mut_import_decl(&mut self, decl: &mut ImportDecl) {
    let src = decl.src.value.as_str().unwrap_or("").to_string();
    let alias_globs: Vec<Glob> = self
      .aliases
      .as_mut()
      .unwrap_or(&mut vec![])
      .iter()
      .map(|alias| Glob::new(alias).unwrap())
      .collect();

    decl.src = Box::new(
      transform_extension(
        src,
        alias_globs,
        &self.extension,
        &self.dir_index,
        &self.dir_index_patterns,
        &self.skip,
        &self.skip_extensions,
      )
      .into(),
    );
  }

  fn visit_mut_export_all(&mut self, decl: &mut ExportAll) {
    let src = decl.src.value.as_str().unwrap_or("").to_string();
    let alias_globs: Vec<Glob> = self
      .aliases
      .as_mut()
      .unwrap_or(&mut vec![])
      .iter()
      .map(|alias| Glob::new(alias).unwrap())
      .collect();

    decl.src = Box::new(
      transform_extension(
        src,
        alias_globs,
        &self.extension,
        &self.dir_index,
        &self.dir_index_patterns,
        &self.skip,
        &self.skip_extensions,
      )
      .into(),
    );
  }

  fn visit_mut_named_export(&mut self, named_export: &mut NamedExport) {
    let src = named_export
      .src
      .as_mut()
      .unwrap_or(&mut Box::new("".into()))
      .value
      .as_str()
      .unwrap_or("")
      .to_string();
    let alias_globs: Vec<Glob> = self
      .aliases
      .as_mut()
      .unwrap_or(&mut vec![])
      .iter()
      .map(|alias| Glob::new(alias).unwrap())
      .collect();

    named_export.src = Some(Box::new(
      transform_extension(
        src,
        alias_globs,
        &self.extension,
        &self.dir_index,
        &self.dir_index_patterns,
        &self.skip,
        &self.skip_extensions,
      )
      .into(),
    ));
  }
}

fn transform_extension(
  src: String,
  alias_glob: Vec<Glob>,
  extension: &str,
  dir_index: &Option<Vec<String>>,
  dir_index_patterns: &Option<Vec<Glob>>,
  skip: &Option<Vec<Glob>>,
  skip_extensions: &[String],
) -> String {
  // 如果路径匹配 skip 模式，直接返回原路径
  if let Some(skip_patterns) = skip {
    if skip_patterns
      .iter()
      .any(|pattern| pattern.compile_matcher().is_match(src.as_str()))
    {
      return src;
    }
  }

  // 处理 dir_index 目录导入（精确匹配）
  if let Some(dirs) = dir_index {
    for dir in dirs {
      if src == *dir || src.starts_with(&format!("{}/", dir)) {
        return format!("{}/index{}", dir, extension);
      }
    }
  }

  // 处理 dir_index_patterns 目录导入（glob 模式匹配）
  // 例如 "./modules/*" 匹配 "./modules/logger" -> "./modules/logger/index.js"
  if let Some(patterns) = dir_index_patterns {
    for pattern in patterns {
      let matcher = pattern.compile_matcher();
      if matcher.is_match(src.as_str()) {
        return format!("{}/index{}", src, extension);
      }
    }
  }

  // 判断路径是否已有扩展名（.js/.mjs/.cjs/.json/.css 等），如果是则跳过
  // 注意：.module 不是扩展名，应该加 .js
  if !src.ends_with(".ts") {
    for ext in skip_extensions {
      if src.ends_with(ext) {
        return src;
      }
    }
  }

  let ts_re = Regex::new(r"^([\./].+)(\.ts)$").unwrap();
  let ext = extension;

  let ts_to_js = ts_re
    .replace(src.as_str(), &format!("$1{}", ext)[..])
    .to_string();
  let no_extension_to_js = if ts_to_js.starts_with(".") && !ts_to_js.ends_with(ext) {
    format!("{}{}", ts_to_js, ext)
  } else {
    ts_to_js
  };
  let new_src = alias_glob
    .iter()
    .any(|alias| {
      alias
        .compile_matcher()
        .is_match(no_extension_to_js.as_str())
    })
    .then(|| {
      let ts_re = Regex::new(r"^(.+)(\.ts)$").unwrap();

      let ts_to_js = ts_re
        .replace(no_extension_to_js.as_str(), &format!("$1{}", ext)[..])
        .to_string();
      let no_extension_to_js = if !ts_to_js.ends_with(ext) {
        format!("{}{}", ts_to_js, ext)
      } else {
        ts_to_js
      };

      no_extension_to_js
    })
    .unwrap_or(no_extension_to_js)
    .into();

  new_src
}

#[plugin_transform]
pub fn process_transform(
  mut program: Program,
  metadata: TransformPluginProgramMetadata,
) -> Program {
  let config = serde_json::from_str::<Config>(
    &metadata
      .get_transform_plugin_config()
      .expect("failed to get plugin config"),
  )
  .expect("invalid plugin config");

  let mut visitor = TransformVisitor::new();
  visitor.set_config(
    config.aliases,
    config.extension,
    config.dir_index,
    config
      .dir_index_patterns
      .map(|patterns| patterns.iter().map(|p| Glob::new(p).unwrap()).collect()),
    config
      .skip
      .map(|patterns| patterns.iter().map(|p| Glob::new(p).unwrap()).collect()),
    config.skip_extensions,
  );

  program.visit_mut_with(&mut visitor);
  program
}
