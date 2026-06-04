use std::fs;
use std::path::PathBuf;

use super::run_rust_source_audit;

#[test]
fn fails_for_orphan_production_source() {
    let root = temp_root("source-orphan");
    let src = root.join("crates/demo/src");
    fs::create_dir_all(&src).expect("create src");
    fs::write(
        root.join("crates/demo/Cargo.toml"),
        "[package]\nname = \"demo\"\n",
    )
    .expect("write manifest");
    fs::write(src.join("lib.rs"), "mod used;\n").expect("write lib");
    fs::write(src.join("used.rs"), "").expect("write used");
    fs::write(src.join("orphan.rs"), "").expect("write orphan");

    let result = run_rust_source_audit(&root);

    assert!(result.is_err());
}

#[test]
fn passes_for_reachable_production_source() {
    let root = temp_root("source-clean");
    let src = root.join("crates/demo/src");
    fs::create_dir_all(&src).expect("create src");
    fs::write(
        root.join("crates/demo/Cargo.toml"),
        "[package]\nname = \"demo\"\n",
    )
    .expect("write manifest");
    fs::write(src.join("lib.rs"), "mod used;\n").expect("write lib");
    fs::write(src.join("used.rs"), "").expect("write used");

    let result = run_rust_source_audit(&root);

    assert!(result.is_ok());
}

fn temp_root(name: &str) -> PathBuf {
    let root = std::env::temp_dir().join(format!(
        "internal-error-xtask-{name}-{}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&root);
    fs::create_dir_all(&root).expect("create temp root");
    root
}
