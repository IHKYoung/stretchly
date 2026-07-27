# TID-20260727-settings-simplification-015 Evidence

- Scope: 设置导航、profile/default、完整休息术语与 prompt 轮换、本地 0.1.5 版本收口。
- Version consistency: root/desktop npm manifests + lockfiles、Cargo manifest + lockfile、Tauri config 均为 `0.1.5`。
- Locale generators: PASS，50 languages；break ideas generator PASS，50 bundles。
- Frontend tests: PASS，6 files / 122 tests。
- Rust tests: PASS，29 tests。
- Typecheck: PASS。
- Frontend production build: PASS；保留既有 Vite large-chunk warning。
- Workflow: agent config SKIP（无 multi-agent config）；kit sync SKIP（无 kit）；workflow docs PASS。
- Root lint: FAIL；Standard 会递归扫描未排除的 Tauri `target/` 生成资产，并报告 site 与 tests 中既有的尾逗号风格，当前不是 clean baseline gate；本任务未扩大范围修复。
- Rust format: BLOCKED；当前 stable toolchain 未安装 `cargo-fmt`，未新增组件。
- UI runtime: `http://127.0.0.1:43179/` 返回 HTTP 200。
- Evidence gap: browser discovery 返回空后端列表，未采集截图、点击 trace 或真实等待 60 秒的视频；导航、计时常量与执行顺序由自动化测试覆盖。
- External boundary: 未构建 DMG，未 tag/push/release，site pinned URL 继续指向公开版 0.1.4。
