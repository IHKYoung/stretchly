# Task-ID: TID-20260408-macos-icon-fidelity

> 若本任务不涉及架构/契约/数据/并发边界，可在此注明 `N/A` 并保持该文件存在（流程门禁要求）。  

## Goals
- 用最小风险修复 macOS tray / dock icon 的宿主接入问题。
- 避免继续依赖扫描全局 status items 的脆弱实现。
- 让图标生成链可重复执行，不再在脚本层面失败。

## Non-Goals
- 不重做品牌 logo 本体。
- 不调整业务逻辑、设置项或 tray 菜单行为。
- 不扩展到非 macOS 平台的宿主图标策略。

## Constraints & Assumptions
- `tray-icon` crate 在 macOS 上对状态栏图标使用固定 `18pt` 逻辑高度。
- 当前 session 不允许未获用户显式授权的子 agent 调度，因此由单 agent 完成架构/实现/测试/文档闭环。
- 终端未获 Screen Recording 权限，无法用系统截图完成最终视觉证据。

## System Boundaries
- Modules:
  - `apps/desktop/src-tauri/src/shell.rs`
  - `graphics/tray-icon.svg`
  - `graphics/generate_icon_assets.py`
  - `apps/desktop/src-tauri/icons/*`
- Ownership:
  - 宿主层图标接入：`shell.rs`
  - 图标源与生成链：`graphics/*`
- Dependency direction:
  - 源 SVG -> 生成脚本 -> 二进制图标资源 -> Tauri/macOS 宿主接入

## API / Contract
- Signatures / Endpoints: 无对外 API 变更
- Request/Response schema (typed): 无
- Error model (codes, retryability): 无新增产品级错误模型；仅修正本地脚本和宿主图标接入

## Data Model / Storage
- N/A

## Invariants
- Tray 菜单行为、break 调度和设置持久化语义保持不变。
- macOS tray icon 只允许操作 Pauza 自己的 `NSStatusItem`。
- Dock icon 使用 bundled multi-representation icon 资源，而不是运行时扫描/拼接其他状态。

## Concurrency / Lifecycle / Memory Model
- macOS tray patch 通过 `TrayIcon::with_inner_tray_icon()` 在主线程拿到原生 tray handle 后立即执行，避免旧实现里异步延迟 + 全局枚举造成的生命周期不确定性。

## Observability Plan (Debug-Driven)
- Logs: 依赖 `cargo check` / `cargo test` / `npm build` / icon generator 命令输出
- Metrics: N/A
- Traces: N/A
- Debug flags: N/A

## Security & Privacy Considerations
- 不新增权限、不触碰用户数据；仅由于缺少 Screen Recording 权限，系统截图验证未执行。

## Risks & Rollback
- Failure modes:
  - tray glyph 在真实菜单栏里仍显得偏大
  - 用户本机由于视觉偏好仍希望继续缩小
- Rollback steps:
  - 回退 `shell.rs`
  - 回退 `graphics/tray-icon.svg` / `graphics/generate_icon_assets.py`
  - 重生成原始 icon 产物

## Acceptance Criteria (System)
- 只 patch Pauza 自己的 `NSStatusItem`
- Dock 改用 bundled `icon.icns`
- 图标生成脚本完整跑通
- Rust / 前端构建通过

## Open Questions / Decision Requests
- 若用户本机仍认为 tray glyph 偏大，下一轮是否允许进一步缩小 `graphics/tray-icon.svg` 的外半径
