//! Regenerates `src/bindings.rs`: `cargo run -p bindgen`.

fn main() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    windows_bindgen::builder()
        .output(root.join("src/bindings.rs"))
        .flat()
        .filter_file(root.join("tools/bindgen/apis.txt"))
        .write();
}
