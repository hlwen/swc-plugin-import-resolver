use swc_core::ecma::{
  transforms::testing::test,
  visit::{Fold, VisitMut},
};
use swc_plugin_import_resolver::TransformVisitor;

fn test_visitor() -> impl 'static + Fold + VisitMut {
  let mut visitor = TransformVisitor::new();
  visitor.set_config(None, ".js".to_string(), None, None);

  visitor
}

fn test_visitor_with_alias() -> impl 'static + Fold + VisitMut {
  let mut visitor = TransformVisitor::new();
  visitor.set_config(Some(vec!["@/*".to_string()]), ".js".to_string(), None, None);

  visitor
}

test!(
  Default::default(),
  |_| test_visitor(),
  add_extension_to_no_extension_import,
  r#"
    import { Hoge, Fuga, Piyo } from "./hogehoge";
    import HogeHoge from "./hogehoge";
    import { pppoe } from "../pppoe";
    import { utils } from "./utils";
    "#,
  r#"
    import { Hoge, Fuga, Piyo } from "./hogehoge.js";
    import HogeHoge from "./hogehoge.js";
    import { pppoe } from "../pppoe.js";
    import { utils } from "./utils.js";
    "#
);

test!(
  Default::default(),
  |_| test_visitor(),
  rewrite_extension_ts_to_js,
  r#"
    import { Hoge, Fuga, Piyo } from "./hogehoge.ts";
    import HogeHoge from "./hogehoge.ts";
    import { pppoe } from "../pppoe.ts";
    "#,
  r#"
    import { Hoge, Fuga, Piyo } from "./hogehoge.js";
    import HogeHoge from "./hogehoge.js";
    import { pppoe } from "../pppoe.js";
    "#
);

test!(
  Default::default(),
  |_| test_visitor(),
  do_nothing_if_extension_is_js,
  r#"
    import { Hoge, Fuga, Piyo } from "./hogehoge.js";
    import HogeHoge from "./hogehoge.js";
    import { pppoe } from "../pppoe.js";
    "#,
  r#"
    import { Hoge, Fuga, Piyo } from "./hogehoge.js";
    import HogeHoge from "./hogehoge.js";
    import { pppoe } from "../pppoe.js";
    "#
);

test!(
  Default::default(),
  |_| test_visitor(),
  do_nothing_if_module_import,
  r#"
    import { Hoge, Fuga, Piyo } from "hogehoge";
    import HogeHoge from "hogehoge/hogehoge";
    import FugaFuga from "@hogehoge/fugafuga";
    "#,
  r#"
    import { Hoge, Fuga, Piyo } from "hogehoge";
    import HogeHoge from "hogehoge/hogehoge";
    import FugaFuga from "@hogehoge/fugafuga";
    "#
);

test!(
  Default::default(),
  |_| test_visitor_with_alias(),
  add_extension_to_no_extension_import_with_alias,
  r#"
    import { Hoge, Fuga, Piyo } from "@/hogehoge";
    import HogeHoge from "@/hogehoge";
    import { pppoe } from "@/pppoe";
    "#,
  r#"
    import { Hoge, Fuga, Piyo } from "@/hogehoge.js";
    import HogeHoge from "@/hogehoge.js";
    import { pppoe } from "@/pppoe.js";
    "#
);

test!(
  Default::default(),
  |_| test_visitor_with_alias(),
  rewrite_extension_ts_to_js_with_alias,
  r#"
    import { Hoge, Fuga, Piyo } from "@/hogehoge.ts";
    import HogeHoge from "@/hogehoge.ts";
    import { pppoe } from "@/pppoe.ts";
    "#,
  r#"
    import { Hoge, Fuga, Piyo } from "@/hogehoge.js";
    import HogeHoge from "@/hogehoge.js";
    import { pppoe } from "@/pppoe.js";
    "#
);

test!(
  Default::default(),
  |_| test_visitor_with_alias(),
  do_nothing_if_module_import_with_alias,
  r#"
    import { Hoge, Fuga, Piyo } from "hogehoge";
    import HogeHoge from "hogehoge/hogehoge";
    import FugaFuga from "@hogehoge/fugafuga";
    "#,
  r#"
    import { Hoge, Fuga, Piyo } from "hogehoge";
    import HogeHoge from "hogehoge/hogehoge";
    import FugaFuga from "@hogehoge/fugafuga";
    "#
);

test!(
  Default::default(),
  |_| test_visitor_with_alias(),
  for_export,
  r#"
    export { Hoge, Fuga, Piyo } from "hogehoge";
    export { pppoe } from "@/pppoe.ts";
    export { HogeHoge } from "@/hogehoge";
    "#,
  r#"
    export { Hoge, Fuga, Piyo } from "hogehoge";
    export { pppoe } from "@/pppoe.js";
    export { HogeHoge } from "@/hogehoge.js";
    "#
);
