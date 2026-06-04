use std::fs;
use std::path::PathBuf;

use super::run_rust_trait_audit;

#[test]
fn fails_for_handwritten_trait_double_in_tests() {
    let root = temp_root("trait-violation");
    let tests = root.join("crates/demo/src/_tests_");
    fs::create_dir_all(&tests).expect("create tests");
    fs::write(
        tests.join("demo_tests.rs"),
        "trait Demo {}\nstruct Fake;\nimpl Demo for Fake {}\n",
    )
    .expect("write test");

    let result = run_rust_trait_audit(&root);

    assert!(result.is_err());
}

#[test]
fn passes_without_handwritten_trait_double() {
    let root = temp_root("trait-clean");
    let tests = root.join("crates/demo/src/_tests_");
    fs::create_dir_all(&tests).expect("create tests");
    fs::write(tests.join("demo_tests.rs"), "fn helper() {}\n").expect("write test");

    let result = run_rust_trait_audit(&root);

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
