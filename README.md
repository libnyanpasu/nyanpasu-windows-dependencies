# nyanpasu-windows-dependencies

Detects the runtimes a Windows desktop program depends on, and installs missing
ones through the Microsoft Store install service.

| `FrameworkDependency` | Detected from | Store product |
| --- | --- | --- |
| `VisualCppRuntime` | `HKLM\SOFTWARE\Microsoft\VisualStudio\14.0\VC\Runtimes\<arch>` (32-bit view) | none |
| `WebView2Runtime` | EdgeUpdate client `pv`, per machine and per user | none |
| `WindowsAppRuntime2` | `Microsoft.WindowsAppRuntime.2` packages registered for the current user | `9NRZT3Q9R3DL` |

```rust
use nyanpasu_windows_dependencies::{FrameworkDependency, Version, store::StoreInstaller};

let required = Version::new(2, 5, 1, 0);
let dependency = FrameworkDependency::WindowsAppRuntime2;
if dependency.diagnose()?.is_none_or(|installed| installed < required) {
    let installer = StoreInstaller::new("MyApp")?;
    if installer.is_available()? {
        // Blocks until the Store finishes; run it off the UI thread.
        installer.install(dependency.store_product_id().unwrap(), |progress| {
            println!("{progress:?}");
        })?;
    }
}
```

The Store install service accepts requests from unpackaged processes, which is
what `winget install --source msstore` uses. On systems without the Store app,
such as Windows LTSC, `is_available` returns `false`; fall back to the official
installer then. See [docs/design.md](docs/design.md) for the evidence and plans.

`src/bindings.rs` is generated: run `cargo run -p bindgen` after editing
`tools/bindgen/apis.txt`.

## License

GPL-3.0
