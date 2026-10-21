# `nyanpasu-windows-dependencies` 设计

**日期：** 2026-10-21

**状态：** 第一阶段实现中。第二阶段（运行传统安装器）与第三阶段（下载）只做规划。

**仓库：** `libnyanpasu/nyanpasu-windows-dependencies`，独立仓库、单 crate。第一个使用方是 [`libnyanpasu/enable-loopback`](https://github.com/libnyanpasu/enable-loopback)；以后 Clash Nyanpasu 的自定义安装器也用它来检测和安装依赖。

## 1. 目标

检测 Windows 桌面程序依赖的运行时（框架），缺失时按"Microsoft Store 安装服务 → 官方传统安装器 → 引导用户下载"的顺序补装：

| `FrameworkDependency` | 检测方式 | Store 安装服务 | 传统安装器 |
| --- | --- | --- | --- |
| `VisualCppRuntime` | 注册表 `HKLM\SOFTWARE\Microsoft\VisualStudio\14.0\VC\Runtimes\{x86,x64,arm64}` 的 `Installed`、`Major`/`Minor`/`Bld`/`Rbld`（[官方说明](https://learn.microsoft.com/en-us/cpp/windows/redistributing-visual-cpp-files#install-the-redistributable-packages)） | 无（见 §2） | `vc_redist.<arch>.exe /install /quiet /norestart`，需要管理员 |
| `WebView2Runtime` | 注册表 `pv`：`HKLM\SOFTWARE\WOW6432Node\Microsoft\EdgeUpdate\Clients\{F3017226-FE2A-4295-8BDF-00C3A9A7E4C5}` 与 `HKCU\...` 同名键，空值或 `0.0.0.0` 视为未装（[官方说明](https://learn.microsoft.com/en-us/microsoft-edge/webview2/concepts/distribution#detect-if-a-webview2-runtime-is-already-installed)） | 无（见 §2） | `MicrosoftEdgeWebview2Setup.exe /silent /install`；未提权时装当前用户 |
| `WindowsAppRuntime2` | `PackageManager` 查当前用户已注册的 `Microsoft.WindowsAppRuntime.2_8wekyb3d8bbwe` 框架包 | 商品 `9NRZT3Q9R3DL`，经 `AppInstallManager` 安装 | `WindowsAppRuntimeInstall-<arch>.exe --quiet`；未提权时装当前用户 |

`diagnose()` 返回当前进程架构下已装的最高版本（未装为 `None`）。版本下限由调用方给出，crate 不内置"需要哪个版本"：下限取决于调用方用什么工具链和 SDK 构建。

只检测当前进程的架构。安装器若要替另一架构的程序检测（例如 x64 安装器装 arm64 程序），到第二阶段再加架构参数。

## 2. 依据（2026-10-21 实测）

- **未打包的 Win32 进程可以直接调用 Store 安装服务。** `windows` crate 不带 `Windows.ApplicationModel.Store.Preview.InstallControl`，用 `windows-bindgen` 生成即可。非提权的普通 exe 激活 `AppInstallManager` 后用 `StartProductInstallWithOptionsAsync` 提交 `9NRZT3Q9R3DL`，Store 日志记录了调用方的 `ClientAppId`，整个流程走完。文档里说的私有 capability 没有挡住调用。WinGet 的 `--source msstore` 和 Chromium 的安装器走的也是这个接口。
- **商品 ID 与包族名的对应关系**以 DisplayCatalog（`displaycatalog.mp.microsoft.com/v7.0/products?bigIds=…`）为准：`9NRZT3Q9R3DL` 是 `Microsoft.WindowsAppRuntime.2` 框架包，不是 Main 包；`9NBLGGH4RV3K` 是 `Microsoft.VCLibs.140.00.UWPDesktop`。
- **Store 商品页不能当安装入口**：`9NRZT3Q9R3DL` 的商品页显示"当前不可用"，"获取"按钮是灰的，但安装服务照样接受这个 ID。
- **VC++ 与 WebView2 在 Store 没有可用的商品。** DisplayCatalog 和 `winget search --source msstore` 都搜不到。VCLibs 框架包代替不了 VC++ Redistributable，原因有三：
  1. Store 中最新版是 `14.0.33728.0`（DLL 14.40），低于构建所用的工具集 14.51，违反"运行库版本不低于工具集"（[binary compat](https://learn.microsoft.com/en-us/cpp/porting/binary-compat-2015-2017)）；
  2. 它是给 MSIX 打包程序声明依赖用的；
  3. 静态导入的 `vcruntime140.dll` 在进程代码运行之前就已经由加载器解析，未打包程序来不及通过 Dynamic Dependencies 把框架包加入进程。
  
  Store 里的 Edge 商品（`XPFFTQ037JWMHS`）只是包了一层 `MicrosoftEdgeUpdateSetup` 的 exe。
- **没有 Store 的系统**（Windows Sandbox 全新镜像，可当作 LTSC 的近似）：`Microsoft.WindowsStore` 与 `Microsoft.DesktopAppInstaller` 都未注册，`ms-windows-store:` 协议查询结果是 `NotSupported`；**`AppInstallManager` 仍能激活**，`IsStoreBlockedByPolicy` 返回 `false`，但所有商品都 404（安装服务以 `market=001` 查 DisplayCatalog）。因此"能激活安装服务"不能作为"Store 可用"的判断依据。
- **Windows App Runtime 不依赖 VC++ Redistributable**：`Microsoft.WindowsAppRuntime.2` 2.5.1 的 DLL 不导入 `vcruntime140.dll`，包清单也没有声明依赖。

## 3. Store 是否可用

`store_available()` 判断条件：当前用户已注册 `Microsoft.WindowsStore_8wekyb3d8bbwe` 包，且策略没有阻止 Store。不满足时直接走下一步，不提交注定失败的安装请求。满足时安装依然可能失败（离线、地区、商品下架），所以调用方必须把安装失败也当作回退的条件。

## 4. Dynamic Dependencies 的作用

**它不用于检测。** 检测用 `PackageManager` 即可，Windows 10 上也能用。`TryCreatePackageDependency` 只在 Windows 11 上有，调用它还会留下一个持久引用作为副作用。

它的用途是**使用和固定（pin）框架包**：

- 使用：未打包程序在运行时把框架包加入进程的包依赖图。enable-loopback 依赖的 windows-reactor 在 bootstrap 时自己会做（以进程为生命周期）。
- 固定：以文件或注册表项为生命周期标记创建引用后，只要标记还在，系统就不会卸载这个框架包；标记消失后引用自动失效。Chromium 在 Store 安装完成后就以版本目录为标记创建了这种引用。reactor 用的是进程级引用，进程退出后就不再保护框架包。

结论：第一阶段不用。若以后发现 Store 装好的、没有其他依赖方的框架包会被系统清理，再给调用方加一个"以 exe 路径为标记固定框架包"的接口。

## 5. 下载放在哪里

**决定：下载不在本 crate 实现，由使用方的适配器负责；本 crate 负责描述安装器、校验签名和运行安装器。**

理由：

1. HTTP 栈因使用方而异。Clash Nyanpasu 用 reqwest + rustls，要走应用的代理设置、显示进度、支持重试和缓存；enable-loopback 不需要也不应该带 HTTP 栈。放进本 crate，就得替所有人选一个 HTTP 客户端和代理策略。
2. 安全边界是"运行之前校验签名"，不在下载环节。不管安装器是下载来的还是随安装包带的，运行前都要用 `WinVerifyTrust` 校验 Authenticode，并确认签名者是 `Microsoft Corporation`。这一步放在本 crate，所有使用方都绕不开。
3. 本 crate 只给出每个依赖的官方安装器描述（下载地址、命令行参数、是否需要管理员），使用方拿描述去下载，或者换成随包携带的本地文件，再交回本 crate 运行。这样"本地文件优先、否则下载"也能实现。

官方下载地址（第二、三阶段使用）：

| 依赖 | 地址 |
| --- | --- |
| `VisualCppRuntime` | `https://aka.ms/vc14/vc_redist.{x64,arm64}.exe`（x64 包同时含 arm64 二进制） |
| `WebView2Runtime` | Evergreen Bootstrapper `https://go.microsoft.com/fwlink/p/?LinkId=2124703` |
| `WindowsAppRuntime2` | `https://aka.ms/windowsappsdk/<major.minor>/<version>/windowsappruntimeinstall-{x64,arm64}.exe` |

## 6. 分阶段

### 第一阶段（本期）

1. crate：`FrameworkDependency`（三个变体）+ `diagnose()`；`store_available()`；`StoreInstaller`（安装并等待完成，带进度回调）；`FrameworkDependency::store_product_id()`。
2. enable-loopback 在 reactor bootstrap 之前改用本 crate：
   - `diagnose(WindowsAppRuntime2)` 达到下限就继续启动；
   - 缺失且 `store_available()`：对话框多一个"从 Microsoft Store 安装"按钮。安装期间显示带进度条的 `TaskDialog`，完成后重新检测并继续启动；失败则提示错误，再回到下载选项；
   - Store 不可用：保持现在的"下载安装程序 / 打开下载页 / 取消"。
3. Clash Nyanpasu 的 draft PR #5867 去掉宿主侧的检测（`runtime_check.rs`、`UwpToolOutcome::RuntimeMissing`）和前端引导弹窗，只负责启动工具并传 `--lang`。两边的 spec §8.2 同步更新。

### 第二阶段：运行传统安装器

- `Installer` 描述：下载地址、参数、是否需要管理员。
- `run_installer(dependency, path)`：先 `WinVerifyTrust`，再确认签名者，然后运行；需要管理员且未提权时，用 `ShellExecuteExW` 的 `runas` 弹 UAC（用户拒绝时返回单独的错误）；最后解析退出码（vc_redist 的 3010 表示需要重启，1638 表示已装了更新的版本）。
- 在 Windows Sandbox 里验证三个安装器（Sandbox 默认没有 VC++、Store 和 App Installer）。

### 第三阶段：下载

由使用方实现（见 §5）。Clash Nyanpasu 自定义安装器接入时一起做。

## 7. 技术约束

- 依赖 crates.io 上的 `windows-core`/`windows-future`/`windows-collections`/`windows-link`/`windows-registry` 0.100，WinRT 与 Win32 绑定全部由 `windows-bindgen` 0.100 在 `build.rs` 生成到 `OUT_DIR`。crates.io 没有发布 `windows` 0.100，而 enable-loopback 用的是 windows-rs 0.100 的 fork。
- enable-loopback 可以用 `[patch.crates-io]` 把这些底层 crate 指向它 vendor 的 fork，只保留一份；前提是 fork 与 crates.io 0.100.0 生成的代码兼容，集成时验证。不兼容就接受重复的 `windows-core`（体积小）。
- 运行时可能在检测过程中变化（例如 Store 正在后台更新）。`diagnose()` 只反映调用那一刻的状态。

## 8. 验证

- 单元测试：版本比较、注册表值解析、包名格式、`AppInstallState` 映射。
- 本机：三种依赖的 `diagnose()` 结果与 `Get-AppxPackage`、注册表、文件版本一致；`StoreInstaller` 对已装商品完成（已在本机验证）。
- Windows Sandbox：三种依赖都检测为未装；`store_available()` 为 `false`。
- 真正下载并部署一个缺失的框架包：需要一台有 Store、缺 `Microsoft.WindowsAppRuntime.2` 的机器，用 enable-loopback 端到端验证。
