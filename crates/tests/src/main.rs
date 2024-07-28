use glob::glob;
use js_to_oxc_tests::{generate_expr_tests, generate_stmt_tests, generate_tests};
use proc_macro2::TokenStream;
use std::fs;
use std::path::PathBuf;
use std::process::{Command, Stdio};

fn main() {
  let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
  let fixture_root = root.join("fixtures");
  let generated_root = root.join("generated");
  let src_root = generated_root.join("src");

  if !src_root.exists() {
    fs::create_dir(src_root.clone()).unwrap();
  }

  let units: Vec<(&str, fn(&str, &str) -> TokenStream)> =
    vec![("expr", generate_expr_tests), ("stmt", generate_stmt_tests)];

  for (name, generator) in &units {
    let files = glob(fixture_root.join(name).join("*.js").to_str().unwrap()).unwrap();
    fs::write(src_root.join(format!("{name}.rs")), generate_tests(name, files, generator)).unwrap();
  }

  let lib_mod =
    units.iter().map(|(name, _)| format!("mod {};", name)).collect::<Vec<String>>().join("\n");
  fs::write(src_root.join("lib.rs"), lib_mod).unwrap();

  Command::new("cargo")
    .arg("fmt")
    .current_dir(generated_root)
    .stderr(Stdio::piped())
    .spawn()
    .unwrap();

  println!("Tests generated");
}
