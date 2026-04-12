# Task-ID: TID-20260412-settings-lock-inversion-deadlock-doc-commit

## Summary
- Title: 沉淀设置页锁反转死锁根因并提交
- Date: 2026-04-12
- Level: moderate
- Lane: deep
- Execution Profile: merge-gate
- Status: DONE

## Requirement Brief
- Goal restatement: 基于用户已给出的死锁链，补齐设置页彩球卡死的真实根因分析、最小代码修正、架构/日志/changelog 沉淀，并完成单独 commit。
- In-scope:
  - `apps/desktop/src-tauri/src/shell.rs` 中 `LAST_TRAY_MENU_TEXT_UPDATER` 的并发修正
  - `docs/Architecture.md`、`docs/CHANGELOG.md`
  - `docs/specs/TID-20260412-settings-lock-inversion-deadlock-doc-commit/*`
  - `docs/logs/2026-04-12.md`、`docs/plans/2026-04-12.md`
  - 本地 git commit
- Out-of-scope:
  - 设置页 autosave 前端实现
  - `commands.rs::update_settings()` 其他 host refresh 逻辑
  - 提示音、全屏浮出、官网下载或其他未完成改动
- Assumptions:
  - 用户提供的死锁链与当前 `shell.rs` 实现一致。
  - 之前“autosave 并发 + 全量刷新”是症状放大器，不是最终并发根因。
- Risks:
  - 若提交范围混入其他未完成宿主改动，会削弱这次根因文档的清晰度。
  - 若文档只写“改成 Arc”而不解释主线程阻塞机制，后续仍可能重复犯错。
- Interaction impact: none  <!-- none | indirect | direct -->
- Primary visible flow: N/A
- Fallback / secondary flow: N/A
- User-visible boundary: N/A
- Key visible states / transitions: N/A

## Goal
- 把设置页彩球卡死的真正宿主层死锁用代码和文档一起收口，让后续维护者能从单一 Task-ID 追溯到完整根因、修复方式与验证结果。

## Scope
- In-scope:
  - `shell.rs` 中 tray 文本 updater 的锁范围收窄
  - 并发根因与修复机制的架构沉淀
  - 当日 workflow docs 与 merge-gate commit
- Out-of-scope:
  - 前端设置页 UI/交互
  - pre-break sound / fullscreen break 浮出问题
  - 官网与发布链路文件

## Source Basis (Read Before Code)
- Related code/files reviewed:
  - `apps/desktop/src-tauri/src/shell.rs`
  - `apps/desktop/src-tauri/src/engine.rs`
  - `apps/desktop/src-tauri/src/lib.rs`
- Related docs/specs/logs reviewed:
  - `docs/specs/TID-20260411-settings-freeze-root-cause-fix/{README.md,plan.md}`
  - `docs/logs/2026-04-12.md`
  - `docs/plans/2026-04-12.md`
  - 用户在对话中给出的死锁链说明
- Why these are sufficient:
  - 当前问题发生在 tray live text 更新与 tray rebuild 的宿主边界；`shell.rs` 覆盖了锁、main-thread dispatch 和 tray rebuild 的全部关键路径，配合旧任务文档和用户给出的死锁序列，足以定位并说明本次真正根因。

## Acceptance Criteria (AC)
- AC1: `shell.rs` 明确避免在持有 `LAST_TRAY_MENU_TEXT_UPDATER` 锁时执行 `MenuItem::set_text()`。
- AC2: 新 task spec、当日日志/计划、`docs/Architecture.md` 与 `docs/CHANGELOG.md` 完整记录死锁链、修复策略与历史纠偏。
- AC3: `cargo test`、workflow docs validator 与 `git diff --check` 通过，并完成单独 commit。

## Interaction Freeze (only when `interaction_impact != none`)
- Freeze status: N/A
- Primary flow: N/A
- Fallback / secondary flow: N/A
- Interaction authority / ownership boundary: N/A
- Visible entrypoints / handoff cues: N/A
- In-scope interactions: N/A
- Out-of-scope interactions: N/A
- Interaction acceptance criteria: N/A
- Validator expectation: 当 `interaction_impact != none` 时，本节与 Requirement Brief 中的交互字段不得继续保留 `N/A/TBD`

## Agent Governance
- Approval Owner: orchestrator
- Execution Profile: merge-gate
- Orchestrator Execution Profile: aggressive baseline uses `sandbox_mode = "danger-full-access"` + `approval_policy = "never"`
- Required Roles: orchestrator,architect,coder,tester,scribe
- Execution Mode Policy: multi-agent preferred; `single-agent-fallback` only when warmup is BLOCKED and scope / reason code are explicitly bounded
- Subagent Approval Policy: non-orchestrator agents must use `approval_policy = "never"`
- Escalation Route: subagent -> Orchestrator -> direct local execution | user (only for destructive or external-impact decisions)
- Safe-local Command Route: subagent -> Orchestrator -> bare repo-local command (for example `git add -- <explicit paths...>`)
- Approval Packet Fields: command / purpose / risk / rollback

## Execution Safety Block
- service_impact: 仅限本地 desktop shell 并发修正、文档沉淀与 git commit
- touches_running_service: no
- backup_required: no
- backup_plan: `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml` + `python3 scripts/validate_workflow_docs.py --mode manual` + `git diff --check`
- rollback_plan: 若后续发现 tray 文本热更新回归，可直接以本次 commit 为边界 `git revert`
- destructive_operations: git commit（用户已明确要求）
- operator_approval_required: no
- rationale: 纯本地 merge-gate 收口，无线上影响、无提权、无额外成本

## Task Breakdown & Definition of Done (DoD)
- [x] Task-1: 收窄 `shell.rs` 中 tray updater 锁区间，只保留死锁修正，不夹带其他宿主行为改动
  - DoD: `git diff -- apps/desktop/src-tauri/src/shell.rs` 只剩 updater `Arc` 化与锁外 `set_text()` 的差异
- [x] Task-2: 补齐新的根因文档切片、架构沉淀、changelog 与当日 logs/plans
  - DoD: 本 Task-ID 的 spec 五件套、`docs/Architecture.md`、`docs/CHANGELOG.md`、`docs/logs/2026-04-12.md`、`docs/plans/2026-04-12.md` 不再保留 `TBD/INIT`
- [x] Task-3: 完成验证与 merge-gate 提交
  - DoD: 相关命令 PASS，且生成单独 commit

## Evidence Plan (UI / E2E)
- Evidence required: no  <!-- yes | no | partial -->
- Owner: evidence_collector
- Artifact path: docs/specs/TID-20260412-settings-lock-inversion-deadlock-doc-commit/evidence/
- Interaction validation note: 当 `interaction_impact != none` 时，此处不应保持 `no`，且需覆盖 primary / fallback / visible states
- Required states to capture:
  - loading: N/A
  - empty: N/A
  - error: N/A
  - disabled: N/A
  - success: N/A

## Observability / Debug Plan
- Logs:
  - 通过 `docs/Architecture.md` 显式记录死锁链、锁顺序要求与修复不变量。
- Error codes:
  - 无新增错误码；宿主 API 错误仍沿 `Result<(), String>` 返回。
- Trace/metrics (optional):
  - N/A
- Debug flags (optional):
  - N/A

## Risks & Rollback
- Risks:
  - 若未来再次在 updater 锁内加入会等待主线程的调用，死锁会复发。
  - 若本次 commit 混入无关宿主改动，会削弱根因归因的清晰度。
- Rollback plan:
  - 直接 `git revert` 本次 commit；若需保留并发安全，则需要重新设计一条不共享该锁的 tray 文本更新通道。

## Sequential Phases
- phase_execution: N/A  <!-- sequential | N/A -->
- phase_confirmation_policy: N/A  <!-- no-intermediate-confirmation | manual | N/A -->
- phase_stop_conditions:
  - N/A

## Execution Plan
- Steps:
  1. 清理 `shell.rs` 中与本任务无关的未提交宿主改动，只保留锁反转修复。
  2. 把真实死锁链、修复前后差异和历史纠偏写入 spec / architecture / changelog / daily docs。
  3. 运行验证命令并完成单独 commit。

## Definition of Done (DoD)
- `plan.md` / `testplan.md` / `README.md` 不保留 `TBD/INIT` 占位进入 DONE。
- 授权 owner、升级路径、风险与回滚已写明。
- 相关验证命令与结果可在 logs/plans 中追溯。

## Approval
- Approval needed: yes  <!-- yes | no -->
- Approved: yes（用户已明确要求“做一个详细的文档沉淀，然后 commit”）
