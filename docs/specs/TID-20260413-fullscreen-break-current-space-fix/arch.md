# Task-ID: TID-20260413-fullscreen-break-current-space-fix

> 若本任务不涉及架构/契约/数据/并发边界，可在此注明 `N/A` 并保持该文件存在（流程门禁要求）。

## Goals
- 在不改 scheduler、settings schema 或前端 UI 的前提下，修复 macOS break window 对当前 fullscreen Space 的覆盖回归。

## Non-Goals
- 不重写 `engine.rs::spawn()` / `PauzaState::tick()`。
- 不新增持久化字段、命令或依赖。
- 不改变 break 页面 React 结构和 CTA。

## Constraints & Assumptions
- 不新增依赖，沿用现有 Tauri / objc2 能力。
- break 到点入口仍由 `engine.rs -> shell::show_break_window()` 负责。
- 当前问题更接近宿主原生 `collectionBehavior` 回归，而不是 monitor 选择或 scheduler 没触发。
- 必须保留当前工作树里已经存在的通知权限、pre-show native patch、activate helper 等改动，不做误回滚。

## System Boundaries
- Modules:
  - `apps/desktop/src-tauri/src/shell.rs`
  - `apps/desktop/src-tauri/src/engine.rs`（只读确认触发入口）
  - `docs/{Architecture,CHANGELOG,CodeMap}.md`
- Ownership:
  - `shell.rs` 负责 break window show/present/focus/activate/native patch
  - 调度、设置和前端 UI 均不在本轮变更边界内
- Dependency direction:
  - `engine.tick()` -> `show_break_window()`
  - `show_break_window()` -> `configure_break_window_native_behavior()` -> `present_break_window()` -> `set_focus()` / `activate_break_application()`

## API / Contract
- Signatures / Endpoints:
  - `show_break_window()`
  - `configure_break_window_native_behavior()`
  - `activate_break_application()`
- Request/Response schema (typed):
  - 无新增对外 API；仅调整内部 macOS overlay policy
- Error model (codes, retryability):
  - 继续使用 `Result<(), String>` 宿主错误模型

## Data Model / Storage
- 无新增字段；`PauzaSettings`、`DesktopSnapshot` 与本地配置结构保持不变。

## Invariants
- 当前 fullscreen Space 必须能看见到点后的 break prompt。
- `show_break_window()` 仍先完成 native patch，再 `show()`，避免异步 level 写回覆盖同步 patch。
- non-focusable break 仍保留应用激活兜底；本轮不把问题错误地下放到 scheduler 或前端。

## Concurrency / Lifecycle / Memory Model
- 不改 `run_on_main_thread()`、deferred destroy 或后台 tick 线程模型。
- 修复只调整原生 window behavior bits 与测试断言，不引入新锁或跨线程共享状态。

## Observability Plan (Debug-Driven)
- Logs:
  - 本轮不新增 logger；若未来仍复现，优先在 `configure_break_window_native_behavior()` 输出 `collectionBehavior` 和当前 `profile.fullscreen`
- Metrics:
  - N/A
- Traces:
  - N/A
- Debug flags:
  - N/A

## Security & Privacy Considerations
- 不新增权限边界、网络调用、外部服务或敏感数据流。

## Risks & Rollback
- Failure modes:
  - 原生 overlay 策略若仍不足，break 可能继续留在后台 Space
  - 策略若过强，可能让普通 non-strict break 更主动抢前台
- Rollback steps:
  - 回退 `shell.rs` 的 collection behavior 修复、测试断言与相关 docs，然后重跑验证

## Acceptance Criteria (System)
- macOS overlay policy 与 2026-04-09 / 2026-04-11 的已知正确路径重新一致。
- Rust 回归测试能守住关键 behavior bits。
- 构建与 docs validator 通过。

## Open Questions / Decision Requests
- 无。
