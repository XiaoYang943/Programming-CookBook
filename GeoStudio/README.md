# GeoStudio

GeoStudio 是一个基于 Tauri v2、Vue 3、TypeScript、Vite 和 MapLibre GL JS 构建的桌面 GIS 实验平台。

当前第一阶段已经完成桌面地图应用骨架。应用启动后会直接进入 MapLibre 地图界面，支持地图缩放、拖动和窗口尺寸自适应。

## 环境要求

在 Windows 上开发和打包前，需要安装以下环境：

- Node.js 20 或更高版本
- npm
- Rust stable 工具链
- Cargo
- Microsoft Edge WebView2 Runtime
- Microsoft C++ Build Tools

推荐通过 [rustup](https://rustup.rs/) 安装 Rust，并确保安装了适用于 Windows 的 MSVC 工具链。

可以使用以下命令检查环境：

```powershell
node --version
npm.cmd --version
rustc --version
cargo --version
```

如果这些命令都能正常输出版本号，说明基础环境已经可用。

## 安装项目依赖

克隆或下载项目后，在项目根目录执行：

```powershell
npm.cmd install
```

该命令会根据 `package.json` 和 `package-lock.json` 安装 Vue、Vite、MapLibre、Tauri CLI 等前端依赖。

通常只需要在首次获取源码、删除了 `node_modules`，或者依赖配置发生变化后重新执行。

### 为什么使用 `npm.cmd`

部分 Windows PowerShell 环境禁止执行 `npm.ps1`，直接运行 `npm` 时可能出现以下错误：

```text
npm.ps1 cannot be loaded because running scripts is disabled on this system
```

此时直接使用 `npm.cmd` 即可，不需要修改系统的 PowerShell 执行策略：

```powershell
npm.cmd install
```

如果你的 PowerShell 可以正常执行 `npm`，也可以把文档中的 `npm.cmd` 替换为 `npm`。

## 源码运行

### 仅运行 Vue 前端

执行：

```powershell
npm.cmd run dev
```

Vite 默认启动地址为：

```text
http://localhost:1420
```

在浏览器打开该地址即可查看地图界面。

仅运行前端适合调试 Vue 组件、CSS 和 MapLibre 地图，不会启动 Tauri 桌面窗口。

按 `Ctrl+C` 可以停止开发服务器。

### 运行 Tauri 桌面程序

执行：

```powershell
npm.cmd run tauri dev
```

该命令会自动完成以下操作：

1. 启动 Vite 开发服务器。
2. 编译 Rust 后端。
3. 打开 GeoStudio 桌面窗口。
4. 在源码变化后自动更新或重新编译。

第一次运行时需要下载和编译 Rust 依赖，因此耗时会明显长于后续启动。

按 `Ctrl+C` 可以停止开发进程并关闭应用。

## 构建前端

如果只需要检查 Vue 前端能否通过类型检查并生成生产资源，执行：

```powershell
npm.cmd run build
```

该命令会依次执行：

- `vue-tsc --noEmit`：检查 TypeScript 和 Vue 类型。
- `vite build`：生成生产环境前端资源。

构建结果位于：

```text
dist/
```

`dist` 只是前端静态资源，不是 Windows 桌面程序。

## 打包 Windows EXE

在项目根目录执行：

```powershell
npm.cmd run tauri build
```

该命令会完成：

1. 前端类型检查和生产构建。
2. Rust Release 模式编译。
3. 生成 GeoStudio 主程序。
4. 生成 Windows NSIS 安装程序。

首次完整打包需要下载并编译 Tauri 和 Rust 依赖，可能需要几分钟。后续构建会复用 Cargo 缓存，速度通常会更快。

### 打包产物

免安装主程序：

```text
src-tauri/target/release/geostudio.exe
```

NSIS 安装程序：

```text
src-tauri/target/release/bundle/nsis/GeoStudio_0.1.0_x64-setup.exe
```

推荐向普通用户分发 NSIS 安装程序。`geostudio.exe` 可以用于本地快速测试，但正式分发时需要同时考虑运行环境和相关资源。

安装包文件名中的版本号来自以下配置：

- `package.json` 中的 `version`
- `src-tauri/tauri.conf.json` 中的 `version`
- `src-tauri/Cargo.toml` 中的 `version`

发布新版本时，应确保三处版本号保持一致。

## 常用命令

| 操作 | 命令 |
| --- | --- |
| 安装依赖 | `npm.cmd install` |
| 运行 Vue 前端 | `npm.cmd run dev` |
| 运行 Tauri 桌面应用 | `npm.cmd run tauri dev` |
| 构建前端 | `npm.cmd run build` |
| 打包 Windows EXE | `npm.cmd run tauri build` |

## 地图网络依赖

当前地图样式使用 MapLibre 官方在线演示地址：

```text
https://demotiles.maplibre.org/style.json
```

因此运行应用时需要能够访问网络。如果底图服务无法访问，应用仍会保持运行，并显示底图加载失败提示。

该演示底图适用于项目开发和功能验证。正式发布前应更换为稳定、符合授权要求的地图服务，或实现离线地图资源支持。

## 常见问题

### `npm.ps1` 被系统禁止执行

使用 `npm.cmd` 代替 `npm`：

```powershell
npm.cmd run tauri dev
```

### Tauri 首次编译时间较长

这是正常现象。Cargo 首次需要下载并编译 Rust 依赖，后续会复用 `src-tauri/target` 中的构建缓存。

### 桌面程序启动后地图空白

首先检查网络是否能够访问 MapLibre 演示底图地址。如果开发者工具中出现网络或 WebGL 错误，还需要检查代理、防火墙、显卡驱动和 WebView2 Runtime。

### 找不到安装包

确认 `npm.cmd run tauri build` 已成功完成，然后检查：

```text
src-tauri/target/release/bundle/nsis/
```

如果构建失败，应先根据终端中的第一条错误信息解决环境或代码问题，再重新执行构建。
