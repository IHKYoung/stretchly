# Task-ID: TID-20260403-break-surface-mode-restore

> 若本任务不涉及架构/契约/数据/并发边界，可在此注明 `N/A` 并保持该文件存在（流程门禁要求）。  

## Goals
- 恢复 Tauri 端 break surface mode 的设置真源和宿主消费链。
- 保持与旧 Electron `fullscreen` 字段的语义兼容。

## Non-Goals
- 不恢复 Electron 全量 surface 高级配置。
- 不改 break scheduler、DND、natural breaks 或 shortcut 逻辑。

## Constraints & Assumptions
- `breakPromptStyle` 与 `fullscreen` 应是不同设置轴。
- 旧 `settings.json` 缺字段时必须能安全回退默认值。
- 当前前端继续通过完整 `PauzaSettings` round-trip 保存。

## System Boundaries
- Modules:
  - `apps/desktop/src-tauri/src/state.rs`
  - `apps/desktop/src-tauri/src/shell.rs`
  - `apps/desktop/src/App.tsx`
  - `apps/desktop/src/locales/{zh-CN,en}.json`
- Ownership:
  - 本任务同时拥有 Tauri 设置模型、host break profile 和前端入口。
- Dependency direction:
  - settings -> shell break window profile -> runtime break window behavior
  - settings -> React form -> update_settings -> state persistence

## API / Contract
- Signatures / Endpoints:
  - `update_settings({ settings: PauzaSettings })`
  - `get_snapshot() -> DesktopSnapshot`
- Request/Response schema (typed):
  - `PauzaSettings` 新增恢复 `fullscreen: bool`
- Error model (codes, retryability):
  - 无新增错误码；仍由 command 失败直接上浮到前端错误条。

## Data Model / Storage
- `settings.json` 重新持久化 `fullscreen`
- 缺少该字段的旧配置通过 `serde(default)` 回退到 `false`

## Invariants
- `fullscreen=false` 时 break window 不应被强制全屏。
- `fullscreen=true` 时 break window 目标显示器应铺满。
- `breakPromptStyle` 仍然存在，负责 window 模式下的强度差异。

## Concurrency / Lifecycle / Memory Model
- `PauzaState.settings` 更新后，下一次 break window profile 读取新字段。
- 前端 `dirty` 表单仍遵守“不被 snapshot 轮询覆盖”的既有规则。

## Observability Plan (Debug-Driven)
- Logs: 无新增专门日志，依赖现有 build/cargo/console 证据。
- Metrics: 无
- Traces: 无
- Debug flags: 无

## Security & Privacy Considerations
- 不新增权限、网络或用户数据采集。

## Risks & Rollback
- Failure modes:
  - 恢复字段后前后端类型不同步。
  - fullscreen 模式在个别平台上仍需要额外平台补丁。
- Rollback steps:
  - 回退上述 Rust/TS 文件和 docs，恢复到当前不含 `fullscreen` 的版本。

## Acceptance Criteria (System)
- `PauzaSettings`、前端 form、`update_settings`、`shell.rs` 使用链打通。
- Rust host 编译通过。
- 不触碰 scheduler 以外模块。

## Open Questions / Decision Requests
- 若后续用户要恢复“常规窗口”或窗口尺寸百分比，应作为另一个高级 surface task 单独处理。
