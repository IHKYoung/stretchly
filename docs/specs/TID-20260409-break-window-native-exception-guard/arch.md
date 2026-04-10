# Task-ID: TID-20260409-break-window-native-exception-guard

> 若本任务不涉及架构/契约/数据/并发边界，可在此注明 `N/A` 并保持该文件存在（流程门禁要求）。  

## Goals
- 把 macOS break window 原生 patch 从“可能直接触发 Rust foreign exception abort”改为可降级的 best-effort 路径。
- 将修复限制在 Tauri host 的原生窗口层，不扩散到 React break 页面和调度状态机。

## Non-Goals
- 不调整 break 页面 UI、CTA 或文案。
- 不修改 `PauzaState` 调度、提醒策略或命令面。

## Constraints & Assumptions
- 不能新增依赖；必须复用当前 `objc2`、Tauri `ns_window()` 和现有 `Result<(), String>` 错误模型。
- 修复必须在非 macOS 平台保持 no-op。
- crash report 中的 `objc2::runtime::message_receiver::*` 与 `_rust_foreign_exception` 足以支持“问题来自 Objective-C exception”这一根因假设。

## System Boundaries
- Modules: `apps/desktop/src-tauri/src/shell.rs`
- Ownership: break window 创建、显示、原生 `NSWindow` patch 与展示前置/前置失败降级
- Dependency direction: `show_break_window()` -> `configure_break_window_native_behavior()` / `present_break_window()` -> `run_macos_native_break_window_patch()`；不改 `state.rs` / `engine.rs` / `commands.rs` 对外契约

## API / Contract
- Signatures / Endpoints: 无新增 Tauri command；仅调整 `shell.rs` 内部 helper 调用顺序
- Request/Response schema (typed): N/A
- Error model (codes, retryability): 原生 patch 里的 Objective-C exception 由“不可恢复 abort”改为“记录上下文并返回 `Ok(())`”；其他字符串错误继续按现有路径上抛

## Data Model / Storage
- N/A（无持久化结构变化）

## Invariants
- 非 macOS 平台的 break window 行为不变。
- break 页面本身的前台 UI 和 CTA 语义不变。
- 即便 native patch 失败，应用也不应仅因此退出。

## Concurrency / Lifecycle / Memory Model
- 不新增线程、不修改共享状态。
- native patch 仍绑定在 break window 的主线程显示生命周期内，只是执行时机从 `window.show()` 之前挪到之后。
- Objective-C exception 不允许穿透到 Rust event loop 外层的 `catch_unwind`，否则会再次触发 foreign exception abort。

## Observability Plan (Debug-Driven)
- Logs: 在 macOS break native patch 的 exception 分支打印 `eprintln!`，包含 `context` 和 exception 摘要
- Metrics: N/A
- Traces: 使用本机 crash report 作为一次性根因定位证据
- Debug flags: 无

## Security & Privacy Considerations
- 无新增权限、网络调用或敏感数据采集；`eprintln!` 只记录窗口 patch 上下文，不含用户数据。

## Risks & Rollback
- Failure modes:
  - 若 native patch 被跳过，break 在 macOS 全屏 Space 中的覆盖增强可能失效
  - 若后续仍有非当前 helper 覆盖到的 AppKit 异常，仍可能需要进一步收紧原生调用边界
- Rollback steps:
  - 回退 `shell.rs` 中新增的 exception guard 和 patch 时机调整
  - 重跑 `cargo check`、`cargo test` 和 desktop build

## Acceptance Criteria (System)
- `run_macos_native_break_window_patch()` 能拦截并降级 macOS break native patch 中的 ObjC exception。
- break window 的 macOS native patch 在 `window.show()` 之后执行。
- 自动化构建链通过，且 docs 明确记录最终现实确认缺口。

## Open Questions / Decision Requests
- 当前无阻塞性开放问题；后续若仍出现 native 崩溃，应单开任务继续缩小 AppKit 调用面。
