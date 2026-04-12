# Task-ID: TID-20260412-settings-lock-inversion-deadlock-doc-commit

> 若本任务不涉及架构/契约/数据/并发边界，可在此注明 `N/A` 并保持该文件存在（流程门禁要求）。  

## Goals
- 把设置页彩球卡死的真实并发根因收口为可追溯的宿主层死锁分析，而不是继续保留不完整的表层解释。
- 在 `apps/desktop/src-tauri/src/shell.rs` 中建立明确不变量：任何 `MenuItem::set_text()` 调用都不能发生在 `LAST_TRAY_MENU_TEXT_UPDATER` 锁持有期内。
- 为后续排查 tray / settings / main-thread dispatch 类问题保留一份能够直接复用的死锁链、修复前后差异和回滚边界。

## Non-Goals
- 不改动前端 autosave 逻辑、`commands.rs::update_settings()` 的 diff 策略或其他设置项 UI。
- 不在本任务内继续处理提示音、全屏浮出或其他 break delivery 议题。
- 不回写改造旧 Task-ID 的历史结论，只在新切片中追加纠偏说明。

## Constraints & Assumptions
- 用户提供的死锁链与当前 `shell.rs` 代码结构一致，足以解释“修改设置时偶发 beachball”以及为何卡点出现在 tray rebuild 路径。
- 运行环境仍为 Tauri/macOS 桌面宿主；`MenuItem::set_text()` 会经由 `run_item_main_thread!` 投递到主线程，并阻塞调用线程直到主线程完成。
- 修复需要最小化差异，避免混入其他宿主层行为变更。

## System Boundaries
- Modules:
  - `apps/desktop/src-tauri/src/shell.rs`
  - `docs/Architecture.md`
  - `docs/CHANGELOG.md`
  - `docs/specs/TID-20260412-settings-lock-inversion-deadlock-doc-commit/*`
- Ownership:
  - `shell.rs` 负责 tray 文本 updater 的缓存、tray rebuild 与窗口宿主行为。
  - `LAST_TRAY_MENU_TEXT_UPDATER` 只负责保存“如何把 status/detail 文本写回当前 tray 菜单项”的可调用句柄，不应承担跨线程同步屏障职责。
- Dependency direction:
  - 后台线程 `engine.rs -> shell::refresh_tray_if_needed()/sync_tray_text()`
  - 主线程 `create_tray_menu() -> register_tray_menu_text_updater()`
  - 两条链路只能通过短暂的 updater 句柄交换共享状态，不能在锁内再触发主线程阻塞调用。

## API / Contract
- Signatures / Endpoints:
  - `fn register_tray_menu_text_updater<R: Runtime + 'static>(status_item: MenuItem<R>, detail_item: MenuItem<R>)`
  - `fn sync_tray_menu_text(snapshot: &DesktopSnapshot) -> Result<(), String>`
- Request/Response schema (typed):
  - `LAST_TRAY_MENU_TEXT_UPDATER` 当前存储 `Arc<dyn Fn(&str, &str) -> Result<(), String> + Send + Sync>`。
  - `register_tray_menu_text_updater()` 负责生成新的 live updater 闭包并写入全局缓存。
  - `sync_tray_menu_text()` 负责读取缓存并把 `snapshot.status / snapshot.status_detail` 推给当前 tray 菜单项。
- Error model (codes, retryability):
  - `MenuItem::set_text()` 的宿主错误仍通过 `Result<(), String>` 向上传递。
  - 并发层面不再允许以“重试”掩盖死锁；修复目标是消除互锁条件，而不是增加超时或吞错。

## Data Model / Storage
- 无持久化结构变更。
- 全局状态只涉及进程内 `OnceLock<Mutex<Option<TrayMenuTextUpdateFn>>>`，用于保存当前 tray 菜单项的 live updater。

## Invariants
- 不得在持有 `LAST_TRAY_MENU_TEXT_UPDATER` 锁时调用任何会同步等待主线程完成的宿主 API。
- `sync_tray_menu_text()` 的锁区间只允许执行“读取并 clone updater 引用”这一廉价操作。
- `register_tray_menu_text_updater()` 可以替换当前 updater，但不能与运行中的 `set_text()` 形成锁环。

## Concurrency / Lifecycle / Memory Model
- 修复前的死锁链：
  - 后台 1s tick 进入 `sync_tray_menu_text()` 并获取 `LAST_TRAY_MENU_TEXT_UPDATER` 锁。
  - 该线程在锁内调用 `status_item.set_text()` / `detail_item.set_text()`。
  - Tauri 把这两个写入操作投递到主线程并让调用线程阻塞等待。
  - 同时，设置保存触发的 tray rebuild 在主线程执行 `create_tray_menu() -> register_tray_menu_text_updater()`，尝试获取同一把锁。
  - 结果变成“后台线程持锁等主线程；主线程等同一把锁”，形成 lock inversion deadlock。
- 修复后的并发模型：
  - 锁内只 clone `Arc` 闭包并立即释放锁。
  - 真正的 `set_text()` 在锁外执行，即使它阻塞等待主线程，也不会阻塞主线程去替换 updater。
  - `Arc` 保证旧 updater 即使被主线程替换，当前一次文本同步仍持有有效句柄直到调用结束。

## Observability Plan (Debug-Driven)
- Logs:
  - 当前没有新增结构化日志；本次更重要的是把死锁链与修复前后边界明确写进架构文档，避免后续再把“彩球”误归因到表层 autosave 行为。
- Metrics:
  - N/A
- Traces:
  - N/A
- Debug flags:
  - N/A

## Security & Privacy Considerations
- 本次不新增外部接口、权限或数据收集。
- 死锁修复仅影响进程内并发与 UI 响应性。

## Risks & Rollback
- Failure modes:
  - 若 future tray API 需要在 updater 闭包里捕获更多状态，开发者可能再次把耗时或会阻塞主线程的逻辑塞回锁区间。
  - 若回滚到旧实现，设置保存和后台 tick 并发命中 tray rebuild 时，beachball 风险会回归。
- Rollback steps:
  - 如需撤回，直接以本次 commit 为边界执行 `git revert`。
  - 回滚后若仍要保留运行稳定性，需要重新设计一条不经共享锁直接调用主线程 `set_text()` 的替代路径。

## Acceptance Criteria (System)
- `shell.rs` 中不再持有 `LAST_TRAY_MENU_TEXT_UPDATER` 锁执行 `MenuItem::set_text()`。
- 文档能完整描述死锁链、为何旧解释不完整、以及为什么 `Arc` clone + 锁外调用可以解除互锁。
- 相关验证命令、workflow docs 与 merge-gate commit 可追溯到同一 Task-ID。

## Open Questions / Decision Requests
- 无。当前范围内的根因、修复方式与提交边界都已明确。
