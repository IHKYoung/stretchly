# Task-ID: TID-20260403-tauri-i18n-resource-layer

## Summary
- Title: Tauri 多语言资源化
- Date: 2026-04-03
- Level: moderate
- Lane: deep
- Execution Profile: single-task
- Status: DONE

## Requirement Brief
- Goal restatement: 将 Tauri 端多语言从临时内联文案重构为共享 locale 资源，并恢复默认中文。
- In-scope: locale JSON、前端 lookup、Rust lookup、设置页语言切换、tray/break/status 文案接线。
- Out-of-scope: 新增第三方 i18n 依赖、扩展更多语言、重做设置页结构。
- Assumptions: `zh-CN / en` 足够覆盖当前阶段；现有 preview 和 Tauri host 都可接受轻量 lookup 方案。
- Risks: key 漏配、插值变量不一致、前后端文案漂移。
- Interaction impact: none
- Primary visible flow: 语言切换属于现有设置流的一部分，无新增路径。
- Fallback / secondary flow: 浏览器 preview 可用于快速检查不同语言。
- User-visible boundary: 主设置页、break prompt、tray、状态文案。
- Key visible states / transitions: 默认中文、切换英文、保存后持久化。

## Goal
- 为 Tauri 建立可维护的多语言底座，并停止在组件/状态机内维护大段翻译文本。

## Scope
- In-scope:
  - `apps/desktop/src/locales/*.json`
  - `apps/desktop/src/i18n.ts`
  - `apps/desktop/src-tauri/src/i18n.rs`
  - `App.tsx` / `state.rs` / `shell.rs` / `commands.rs` 的文案调用点
- Out-of-scope:
  - Electron 端 locale 重构
  - 多语言自动检测
  - 新增语言包

## Source Basis (Read Before Code)
- Related code/files reviewed:
  - `app/main.js`
  - `app/locales/*.json`
  - `apps/desktop/src/App.tsx`
  - `apps/desktop/src-tauri/src/{state,shell,commands}.rs`
- Related docs/specs/logs reviewed:
  - `docs/UI.md`
  - `docs/CodeMap.md`
  - `docs/logs/2026-04-03.md`
- Why these are sufficient:
  - 已覆盖旧 Electron 的多语言组织方式、当前 Tauri 前后端调用点以及仓库约束文档。

## Acceptance Criteria (AC)
- AC1: Tauri 前后端使用共享 locale JSON，不再在调用点内维护大段翻译文本。
- AC2: 默认语言为中文，设置页保留最小语言切换入口。
- AC3: tray、break prompt、运行时状态文案随语言设置一致切换。

## Interaction Freeze (only when `interaction_impact != none`)
- Freeze status: N/A
- Primary flow: N/A
- Fallback / secondary flow: N/A
- Interaction authority / ownership boundary: N/A
- Visible entrypoints / handoff cues: N/A
- In-scope interactions: N/A
- Out-of-scope interactions: N/A
- Interaction acceptance criteria: N/A

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
- service_impact: 仅重构 Tauri 端文案管理与默认语言，不改调度语义
- touches_running_service: no
- backup_required: no
- backup_plan: 依赖 VCS 回退
- rollback_plan: 回退 locale 资源与调用点
- destructive_operations: 移除临时内联文案方案
- operator_approval_required: no
- rationale: 用户明确要求改成优雅的 i18n 管理方式

## Task Breakdown & Definition of Done (DoD)
- [x] Task-1: 建立共享 locale JSON 与前端/Rust lookup 层
  - DoD: React 与 Rust 都能通过 key 读取文案并处理简单插值
- [x] Task-2: 改造 Tauri 设置页与宿主层文案调用点
  - DoD: 默认中文、语言切换恢复，tray/break/status 文案统一接入

## Evidence Plan (UI / E2E)
- Evidence required: no
- Owner: evidence_collector
- Artifact path: docs/specs/TID-20260403-tauri-i18n-resource-layer/evidence/
- Required states to capture:
  - loading: 已由现有 UI 逻辑覆盖
  - empty: break cleared
  - error: error banner
  - disabled: busyAction
  - success: 语言切换保存后文案一致

## Observability / Debug Plan
- Logs: 保持现有 `last_action` 与 `status_detail`，key 缺失时回退到 key 本身便于定位
- Error codes: 无新增
- Trace/metrics (optional): 无
- Debug flags (optional): 无

## Risks & Rollback
- Risks:
  - key 漏配导致直接显示 key
  - 插值变量名不一致导致文本残缺
- Rollback plan:
  - 回退 `src/locales/*.json`、`src/i18n.ts`、`src-tauri/src/i18n.rs` 及调用点

## Sequential Phases
- phase_execution: N/A  <!-- sequential | N/A -->
- phase_confirmation_policy: N/A  <!-- no-intermediate-confirmation | manual | N/A -->
- phase_stop_conditions:
  - N/A

## Execution Plan
- Steps:
  1. 抽离 locale JSON 并建立 lookup / interpolation 层
  2. 将前端和 Rust host 的调用点全部切到 key 方案
  3. 验证 typecheck / build / cargo check / workflow docs

## Definition of Done (DoD)
- `plan.md` / `testplan.md` / `README.md` 不保留 `TBD/INIT` 占位进入 DONE。
- 授权 owner、升级路径、风险与回滚已写明。
- 相关验证命令与结果可在 logs/plans 中追溯。

## Approval
- Approval needed: yes
- Approved: yes（用户以直接实现指令批准）
