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
  skip: Option<Vec<String>>,
}

fn default_extension() -> String {
  ".js".to_string()
}

pub struct TransformVisitor {
  aliases: Option<Vec<String>>,
  extension: String,
  dir_index: Option<Vec<String>>,
  skip: Option<Vec<Glob>>,
}

impl TransformVisitor {
  pub fn new() -> Self {
    TransformVisitor {
      aliases: None,
      extension: ".js".to_string(),
      dir_index: None,
      skip: None,
    }
  }

  pub fn set_config(
    &mut self,
    aliases: Option<Vec<String>>,
    extension: String,
    dir_index: Option<Vec<String>>,
    skip: Option<Vec<Glob>>,
  ) {
    self.aliases = aliases;
    self.extension = extension;
    self.dir_index = dir_index;
    self.skip = skip;
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
        &self.skip,
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
        &self.skip,
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
        &self.skip,
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
  skip: &Option<Vec<Glob>>,
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

  // 处理 dir_index 目录导入
  if let Some(dirs) = dir_index {
    for dir in dirs {
      if src == *dir || src.starts_with(&format!("{}/", dir)) {
        return format!("{}/index{}", dir, extension);
      }
    }
  }

  // 判断路径是否已有扩展名（.js/.mjs/.cjs/.json/.css 等），如果是则跳过
  // 注意：.module 不是扩展名，应该加 .js
  if let Some(file_name) = std::path::Path::new(&src).file_name() {
    if let Some(file_str) = file_name.to_str() {
      let has_known_ext = file_str.ends_with(".js")
        || file_str.ends_with(".mjs")
        || file_str.ends_with(".cjs")
        || file_str.ends_with(".json")
        || file_str.ends_with(".css")
        || file_str.ends_with(".png")
        || file_str.ends_with(".svg")
        || file_str.ends_with(".jpg")
        || file_str.ends_with(".jpeg")
        || file_str.ends_with(".gif")
        || file_str.ends_with(".webp")
        || file_str.ends_with(".woff")
        || file_str.ends_with(".woff2")
        || file_str.ends_with(".ttf")
        || file_str.ends_with(".eot")
        || file_str.ends_with(".otf")
        || file_str.ends_with(".mp3")
        || file_str.ends_with(".mp4")
        || file_str.ends_with(".wav")
        || file_str.ends_with(".ogg")
        || file_str.ends_with(".webm")
        || file_str.ends_with(".pdf")
        || file_str.ends_with(".zip")
        || file_str.ends_with(".tar")
        || file_str.ends_with(".gz")
        || file_str.ends_with(".bz2")
        || file_str.ends_with(".7z")
        || file_str.ends_with(".rar")
        || file_str.ends_with(".exe")
        || file_str.ends_with(".dll")
        || file_str.ends_with(".so")
        || file_str.ends_with(".dylib")
        || file_str.ends_with(".node")
        || file_str.ends_with(".wasm")
        || file_str.ends_with(".map");
      if has_known_ext && !src.ends_with(".ts") {
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
      .skip
      .map(|patterns| patterns.iter().map(|p| Glob::new(p).unwrap()).collect()),
  );

  program.visit_mut_with(&mut visitor);
  program
}
