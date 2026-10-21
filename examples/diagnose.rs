//! Prints what this machine has installed, and with `--install` installs Windows App
//! Runtime 2 through the Microsoft Store: `cargo run --example diagnose [-- --install]`.

#[cfg(windows)]
fn main() {
    use nyanpasu_windows_dependencies::{FrameworkDependency, store::StoreInstaller};

    for dependency in [
        FrameworkDependency::VisualCppRuntime,
        FrameworkDependency::WebView2Runtime,
        FrameworkDependency::WindowsAppRuntime2,
    ] {
        match dependency.diagnose() {
            Ok(Some(version)) => println!("{dependency:?}: {version}"),
            Ok(None) => println!("{dependency:?}: not installed"),
            Err(error) => println!("{dependency:?}: {error}"),
        }
    }

    let installer = match StoreInstaller::new("NyanpasuDiagnose") {
        Ok(installer) => installer,
        Err(error) => return println!("store: {error}"),
    };
    let available = installer.is_available();
    println!("store available: {available:?}");
    if std::env::args().any(|arg| arg == "--install") && matches!(available, Ok(true)) {
        let product = FrameworkDependency::WindowsAppRuntime2
            .store_product_id()
            .expect("Windows App Runtime 2 has a Store product");
        let result = installer.install(product, |progress| println!("{progress:?}"));
        println!("install: {result:?}");
    }
}

#[cfg(not(windows))]
fn main() {}
