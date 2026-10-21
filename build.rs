// Hands the theme's path to dependents: their build scripts read it as
// `DEP_OXIDT_UI_THEME` and import `$DEP_OXIDT_UI_THEME/oxidt.css`.
fn main() {
    let dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    println!("cargo::metadata=theme={dir}/theme");
    println!("cargo::rerun-if-changed=build.rs");
}
