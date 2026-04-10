# Task-ID: TID-20260409-break-window-native-exception-guard

## Summary
- Title: 修复 macOS 休息窗口原生异常闪退
- Date: 2026-04-09
- Level: moderate
- Lane: deep
- Execution Profile: single-task
- Status: DONE

## Requirement Brief
- Goal restatement: 修复 macOS 进入 break 页面前触发的 `Rust cannot catch foreign exceptions, aborting`，让 break 宿主窗口即使 native patch 失败也降级为 no-op，而不是直接终止进程。
- In-scope:
  - 调整 `apps/desktop/src-tauri/src/shell.rs` 中 break window 的 macOS native patch 时机。
  - 为 `ns_window()` 与相关 ObjC selector 调用添加 exception guard。
  - 补齐本任务的 evidence、Architecture、CHANGELOG 与 daily docs。
- Out-of-scope:
  - 不修改 React break 页面结构、文案和 CTA。
  - 不修改 `state.rs` / `engine.rs` 的 break 调度逻辑。
  - 不处理 tray、快捷键或多屏目标选择策略。
- Assumptions:
  - 根因在于 break window 新增的 macOS 原生 patch 触发了 Objective-C exception，而不是前端页面渲染或调度状态机本身。
  - 对 break window 而言，native patch 属于增强项而不是必须项，失败时允许退回 Tauri 默认显示路径。
  - 将原生 patch 延后到 `window.show()` 之后可以降低 `NSWindow` 尚未 ready 时访问原生句柄的风险。
- Risks:
  - 若 native patch 被跳过，macOS 全屏 Space 覆盖能力可能退化为较保守行为。
  - 当前没有自动化方式在 Tauri runtime 中稳定复现“进入 break”场景，最终现实确认仍需用户本机手动复测。
- Interaction impact: direct  <!-- none | indirect | direct -->
- Primary visible flow: 正常工作中的用户进入 break 前，宿主窗口不再因为原生异常直接 abort，而是继续尝试显示 break。
- Fallback / secondary flow: 当 macOS native patch 抛异常或句柄不可用时，break 退回 Tauri 默认显示路径，应用继续存活。
- User-visible boundary: 仅影响进入 break 前的宿主窗口展示行为，不改变 break 页面内部 UI。
- Key visible states / transitions:
  - break due
  - window.show()
  - native patch attempt
  - success or skip-on-exception
  - break page visible or继续由现有宿主路径处理

## Goal
- 让 macOS break window 的原生增强逻辑不再成为进程 abort 点，并把 patch 执行时机移动到窗口已显示之后。

## Scope
- In-scope:
  - `apps/desktop/src-tauri/src/shell.rs`
  - `docs/specs/TID-20260409-break-window-native-exception-guard/*`
  - `docs/plans/2026-04-09.md`
  - `docs/logs/2026-04-09.md`
  - `docs/Architecture.md`
  - `docs/CHANGELOG.md`
- Out-of-scope:
  - `apps/desktop/src/App.tsx`
  - `apps/desktop/src-tauri/src/state.rs`
  - `apps/desktop/src-tauri/src/engine.rs`
  - `apps/desktop/src-tauri/src/commands.rs`
  - `app/**`

## Source Basis (Read Before Code)
- Related code/files reviewed:
  - `apps/desktop/src-tauri/src/shell.rs`
  - `apps/desktop/src-tauri/src/engine.rs`
  - `apps/desktop/src-tauri/src/commands.rs`
  - `apps/desktop/src-tauri/Cargo.toml`
- Related docs/specs/logs reviewed:
  - `docs/RepositoryGuidelines.md`
  - `docs/CodeMap.md`
  - `docs/logs/2026-04-09.md`
  - `docs/specs/TID-20260408-tauri-window-fullscreen-fix/*`
  - `docs/specs/TID-20260409-break-window-macos-fullscreen-coverage/*`
  - `docs/specs/TID-20260404-fullscreen-break-close-fix/*`
  - `~/Library/Logs/DiagnosticReports/pauza-desktop-2026-04-09-224258.ips`
  - `~/Library/Logs/DiagnosticReports/pauza-desktop-2026-04-09-232038.ips`
- Why these are sufficient:
  - `shell.rs` 已覆盖 break window 创建、显示与 macOS native patch 全路径；结合 crash report 栈里的 `objc2::runtime::message_receiver::*` 即可确认崩溃与原生 selector 调用强相关。

## Acceptance Criteria (AC)
- AC1: macOS break window 的 `ns_window()` 与 ObjC selector 调用不再让 foreign exception 直接 abort 进程。
- AC2: break window 的 macOS native patch 改为在 `window.show()` 之后执行。
- AC3: native patch 失败时会记录上下文并降级为 no-op，不阻断 break 宿主后续流程。
- AC4: `cargo check`、`cargo test`、`npm --prefix apps/desktop run build`、workflow docs validator 与 agent config validator 通过。

## Interaction Freeze (only when `interaction_impact != none`)
- Freeze status: frozen
- Primary flow: break 到点后宿主窗口先 `show()`，随后尝试 macOS native patch；即便 patch 抛 Objective-C exception，也不允许应用 abort。
- Fallback / secondary flow: native patch 失败时仅跳过 `collectionBehavior` / `setLevel` / `orderFrontRegardless` 的原生增强，不改变已有 break 页面入口和 CTA。
- Interaction authority / ownership boundary: 只改 `shell.rs` 的宿主原生窗口行为，不改前端 break 页面和调度状态机。
- Visible entrypoints / handoff cues: 现有 break 到点弹出路径。
- In-scope interactions:
  - break 宿主窗口出现前的原生 patch
  - exception -> no-op 降级
  - break 页面继续显示
- Out-of-scope interactions:
  - break 页面布局和 CTA
  - settings page
  - scheduler 触发时机
- Interaction acceptance criteria:
  - break 进入前不再因为 foreign exception 直接 abort。
  - native patch 的失败应只影响增强效果，不影响应用继续存活。
- Validator expectation: direct interaction 字段已补齐。

## Agent Governance
- Approval Owner: orchestrator
- Execution Profile: single-task
- Orchestrator Execution Profile: aggressive baseline uses `sandbox_mode = "danger-full-access"` + `approval_policy = "never"`
- Required Roles: orchestrator,architect,coder,tester,scribe
- Execution Mode Policy: multi-agent preferred; `single-agent-fallback` only when warmup is BLOCKED and scope / reason code are explicitly bounded
- Subagent Approval Policy: non-orchestrator agents must use `approval_policy = "never"`
- Escalation Route: subagent -> Orchestrator -> direct local execution | user (only for destructive or external-impact decisions)
- Safe-local Command Route: subagent -> Orchestrator -> bare repo-local command (for example `git add -- <explicit paths...>`)
- Approval Packet Fields: command / purpose / risk / rollback

## Execution Safety Block
- service_impact: 仅限 macOS break 宿主窗口的原生 patch 与任务文档
- touches_running_service: no
- backup_required: no
- backup_plan: 以 crash report 对照、Git diff、`cargo check`、`cargo test`、desktop build 和 workflow validator 作为回归边界
- rollback_plan: 回退 `apps/desktop/src-tauri/src/shell.rs` 中的 exception guard / patch 时机调整与相关文档
- destructive_operations: none
- operator_approval_required: no
- rationale: 不涉及运行中服务、数据迁移、权限、新依赖、外部副作用或付费操作

## Task Breakdown & Definition of Done (DoD)
- [x] Task-1: 为 break window 的 macOS native patch 加上 ObjC exception guard
  - DoD: `ns_window()` 和后续 selector 调用经统一 helper 包裹，异常时只记录上下文并返回 `Ok(())`
- [x] Task-2: 调整 break window 的原生 patch 时机并完成文档闭环
  - DoD: `window.show()` 之后才执行 native patch / present，相关 docs、validator 与构建链通过

## Evidence Plan (UI / E2E)
- Evidence required: partial  <!-- yes | no | partial -->
- Owner: evidence_collector
- Artifact path: docs/specs/TID-20260409-break-window-native-exception-guard/evidence/
- Interaction validation note: 以 crash report 分析、代码路径、Rust/前端构建回归为主；由于当前缺少可编排的 Tauri 原生“强制进入 break”自动化入口，最终现实确认仍需用户本机手动复测一次 break 进入流程。
- Required states to capture:
  - loading: N/A
  - empty: N/A
  - error: ObjC exception 路径被降级为 no-op，不再触发进程 abort
  - disabled: N/A
  - success: break 宿主窗口在 `window.show()` 之后再做 native patch，自动化构建链通过

## Observability / Debug Plan
- Logs: 新增 `eprintln!`，在 `configure_break_window_native_behavior` / `present_break_window` 的 ObjC exception 分支输出上下文
- Error codes: 沿用现有 `Result<(), String>` 错误上抛；ObjC exception 改为日志可见、流程降级
- Trace/metrics (optional): 通过 crash report 栈与构建链结果做一次性定位，不新增运行时 metrics
- Debug flags (optional): 无

## Risks & Rollback
- Risks:
  - 若 macOS native patch 被跳过，break 在全屏 Space 下的覆盖行为可能退化为较保守路径。
  - 如果后续发现 `window.show()` 之后仍有生命周期竞态，可能需要进一步改成显式延迟或更细粒度的 AppKit 主线程编排。
- Rollback plan:
  - 回退 `shell.rs` 中的 `run_macos_native_break_window_patch()`、`show_break_window()` patch 时机调整与对应文档
  - 重新执行 `cargo check`、`cargo test`、desktop build 与 docs validator

## Sequential Phases
- phase_execution: N/A  <!-- sequential | N/A -->
- phase_confirmation_policy: N/A  <!-- no-intermediate-confirmation | manual | N/A -->
- phase_stop_conditions:
  - N/A

## Execution Plan
- Steps:
  1. 复盘 break window 现有 macOS native patch 与 crash report 栈，确认 foreign exception 来源。
  2. 在 `shell.rs` 引入统一 exception guard，并把 native patch 移到 `window.show()` 之后。
  3. 运行 Rust/前端构建链与 workflow validators，补齐 evidence、Architecture、CHANGELOG、daily docs。

## Definition of Done (DoD)
- `plan.md` / `testplan.md` / `README.md` 不保留 `TBD/INIT` 占位进入 DONE。
- 授权 owner、升级路径、风险与回滚已写明。
- 相关验证命令与结果可在 logs/plans 中追溯。

## Approval
- Approval needed: yes  <!-- yes | no -->
- Approved: yes（用户于 2026-04-09 提供 crash 信息并明确要求先修 fatal break 崩溃；本轮不涉及运行中服务、破坏性操作、提权或外部副作用）
