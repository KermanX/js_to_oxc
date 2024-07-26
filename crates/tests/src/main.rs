use glob::glob;
use js_to_oxc_tests::generate_expr_tests;
use std::fs;
use std::path::PathBuf;

fn main() {
  let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
  let fixture_root = root.join("fixtures");
  let generated_root = root.join("generated");
  let src_root = generated_root.join("src");

  if src_root.exists() {
    fs::remove_dir_all(src_root.clone()).unwrap();
  }
  fs::create_dir(src_root.clone()).unwrap();

  let units = vec!["expr"];

  for unit in &units {
    let files = glob(fixture_root.join(unit).join("*.js").to_str().unwrap()).unwrap();
    fs::write(src_root.join(format!("{unit}.rs")), generate_expr_tests(files)).unwrap();
  }

  let lib_mod = units.iter().map(|unit| format!("mod {};", unit)).collect::<Vec<String>>().join("\n");
  fs::write(src_root.join("lib.rs"), lib_mod).unwrap();

  println!("Tests generated");
}
