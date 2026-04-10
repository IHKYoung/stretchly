# Task-ID: TID-20260409-language-selector-multilang-fix

## Summary
- Title: 修复设置页多语言选择
- Date: 2026-04-09
- Level: moderate
- Lane: fast
- Execution Profile: single-task
- Status: DONE

## Requirement Brief
- Goal restatement: 让设置页真正支持选择多语言，而不是只剩 `zh-CN/en` 两项或选了也被运行时回退。
- In-scope:
  - 前端 `i18n.ts` 语言列表与归一化逻辑
  - Rust `i18n.rs` 语言归一化逻辑
  - 设置页语言控件形态
  - 对应回归测试
- Out-of-scope:
  - 新增翻译
  - 修改 locale config/messages 内容
- Assumptions:
  - locale registry 已包含完整语言配置与消息
- Risks:
  - 前后端语言归一化不一致
- Interaction impact: none
- Primary visible flow: N/A
- Fallback / secondary flow: N/A
- User-visible boundary: N/A
- Key visible states / transitions: N/A

## Goal
- 修复“语言项可见性”和“语言值真正生效”两层问题。

## Scope
- In-scope:
  - `apps/desktop/src/i18n.ts`
  - `apps/desktop/src-tauri/src/i18n.rs`
  - `apps/desktop/src/App.tsx`
  - `test/desktopSettingsControls.js`
- Out-of-scope:
  - locale 资源翻译内容
  - registry 生成脚本

## Source Basis (Read Before Code)
- Related code/files reviewed:
  - `apps/desktop/src/App.tsx`
  - `apps/desktop/src/i18n.ts`
  - `apps/desktop/src-tauri/src/i18n.rs`
  - `apps/desktop/src/locales/config/*.json`
- Related docs/specs/logs reviewed:
  - `docs/RepositoryGuidelines.md`
  - `docs/CodeMap.md`
- Why these are sufficient:
  - 已覆盖 UI 入口、前后端归一化逻辑和 locale config 真源。

## Acceptance Criteria (AC)
- AC1: 设置页语言选择可显示完整 locale 列表。
- AC2: 已存在 locale code 的 `normalizeLanguage()` 与 Rust `normalize_language()` 不再强制回退到 `zh-CN/en`。
- AC3: `npm test`、`npm --prefix apps/desktop run typecheck`、`cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`、`npm --prefix apps/desktop run build` 通过。

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
- Execution Profile: single-task
- Orchestrator Execution Profile: aggressive baseline uses `sandbox_mode = "danger-full-access"` + `approval_policy = "never"`
- Required Roles: orchestrator,architect,coder,tester,scribe
- Execution Mode Policy: multi-agent preferred; `single-agent-fallback` only when warmup is BLOCKED and scope / reason code are explicitly bounded
- Subagent Approval Policy: non-orchestrator agents must use `approval_policy = "never"`
- Escalation Route: subagent -> Orchestrator -> direct local execution | user (only for destructive or external-impact decisions)
- Safe-local Command Route: subagent -> Orchestrator -> bare repo-local command (for example `git add -- <explicit paths...>`)
- Approval Packet Fields: command / purpose / risk / rollback

## Execution Safety Block
- service_impact: 设置页语言选择与 i18n 归一化
- touches_running_service: no
- backup_required: no
- backup_plan: 以测试、typecheck、cargo test 和 build 为边界
- rollback_plan: 回退 `App.tsx`、前后端 `i18n` 与测试
- destructive_operations: none
- operator_approval_required: no
- rationale: 本地设置行为修复，不涉及外部副作用

## Task Breakdown & Definition of Done (DoD)
- [x] Task-1: 去掉前后端 i18n 对 `desktopReady` 的封锁
  - DoD: 语言 code 可按 locale config 自身生效。
- [x] Task-2: 将设置页语言控件改为下拉
  - DoD: 可展示完整语言列表。
- [x] Task-3: 增加回归测试
  - DoD: 语言列表和归一化行为可被自动化验证。

## Evidence Plan (UI / E2E)
- Evidence required: no  <!-- yes | no | partial -->
- Owner: evidence_collector
- Artifact path: docs/specs/TID-20260409-language-selector-multilang-fix/evidence/
- Interaction validation note: 当 `interaction_impact != none` 时，此处不应保持 `no`，且需覆盖 primary / fallback / visible states
- Required states to capture:
  - loading:
  - empty:
  - error:
  - disabled:
  - success:

## Observability / Debug Plan
- Logs: 依赖现有设置保存错误提示
- Error codes: N/A
- Trace/metrics (optional): N/A
- Debug flags (optional): N/A

## Risks & Rollback
- Risks:
  - 部分语言 code 若未被前后端一致处理，可能保存后回退
- Rollback plan:
  - 回退前后端 i18n 与设置页语言控件改动

## Sequential Phases
- phase_execution: N/A  <!-- sequential | N/A -->
- phase_confirmation_policy: N/A  <!-- no-intermediate-confirmation | manual | N/A -->
- phase_stop_conditions:
  - N/A

## Execution Plan
- Steps:
  1. 审查语言列表来源与归一化逻辑。
  2. 修复前后端语言回退。
  3. 将设置页语言控件改为下拉并补测试。

## Definition of Done (DoD)
- `plan.md` / `testplan.md` / `README.md` 不保留 `TBD/INIT` 占位进入 DONE。
- 授权 owner、升级路径、风险与回滚已写明。
- 相关验证命令与结果可在 logs/plans 中追溯。

## Approval
- Approval needed: yes
- Approved: yes（orchestrator 已按当前 session 授权路由批准执行）
