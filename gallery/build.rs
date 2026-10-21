fn main() {
    for path in ["tailwind.css", "src", "../theme", "../src/components"] {
        println!("cargo::rerun-if-changed={path}");
    }
    topcoat::tailwind::BuildConfig::new()
        .input("tailwind.css")
        .render()
        .unwrap();
}
