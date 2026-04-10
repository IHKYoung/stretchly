# Task-ID: TID-20260409-apps-locale-single-source-refactor

## Summary
- Title: 收口 apps 文案到单一 messages 真源
- Date: 2026-04-09
- Level: complex
- Lane: deep
- Execution Profile: single-task
- Status: DONE

## Requirement Brief
- Goal restatement: 停止使用 `apps/desktop/src/locales/break-message-copy.ts` 与 `break-message-copy.json`，把当前桌面端所有可见文案彻底统一到 `apps/desktop/src/locales/messages/*.json`；`config/*.json` 只保留语言元信息，`app/` 不再重新参与任何当前桌面端文案链路。
- In-scope:
  - 将 break prompt 专属文案并入 `messages/{en,zh-CN}.json`
  - 改造 `App.tsx` 与 `break-prompt.ts` 直接通过 `i18n.ts` 读取 break 文案
  - 删除 `break-message-copy.ts` / `break-message-copy.json`
  - 更新当前架构文档、README、CodeMap、CHANGELOG 与本任务 docs
  - 补一条回归测试，防止后续重新引入并行文案文件
- Out-of-scope:
  - 物理删除 `app/` 目录
  - 新增更多 `desktopReady` 语言
  - 调整 break prompt 布局、按钮行为或 Tauri host 调度逻辑
- Assumptions:
  - `desktopReady` 语言当前只有 `zh-CN` / `en`
  - `i18n.ts` 的 key fallback 足以承接未来非 `desktopReady` 语言
  - `registry.generated.json` 的结构与前后端消费方式可以保持不变
- Risks:
  - 残留 `break-message-copy.*` import 或文档口径，导致架构继续双轨
  - `ui.breakCopy.*` key 漏配，运行时可能显示 key 或落回错误语言
  - locale registry 未重生成，导致前后端看到的文案源不一致
- Interaction impact: none  <!-- none | indirect | direct -->
- Primary visible flow: N/A
- Fallback / secondary flow: N/A
- User-visible boundary: N/A
- Key visible states / transitions: N/A

## Goal
- 让 `messages/*.json` 成为 `apps/desktop` 唯一文案真源，并完成 break prompt 文案链路的最终收口。

## Scope
- In-scope:
  - `apps/desktop/src/{App.tsx,lib/break-prompt.ts,i18n.ts}`
  - `apps/desktop/src/locales/{messages/**,config/**,registry.generated.json}`
  - `test/desktopBreakCopySource.js`
  - `apps/desktop/README.md`
  - `docs/{RepositoryGuidelines,CodeMap,Architecture,UI,CHANGELOG}.md`
  - `docs/specs/TID-20260409-apps-locale-single-source-refactor/*`
  - `docs/{plans,logs}/2026-04-09.md`
- Out-of-scope:
  - `app/**` 目录清理
  - Rust host 行为调整
  - commit / push

## Source Basis (Read Before Code)
- Related code/files reviewed:
  - `apps/desktop/src/App.tsx`
  - `apps/desktop/src/lib/break-prompt.ts`
  - `apps/desktop/src/i18n.ts`
  - `apps/desktop/src/locales/break-message-copy.ts`
  - `apps/desktop/src/locales/break-message-copy.json`
  - `apps/desktop/src/locales/messages/{en,zh-CN}.json`
  - `apps/desktop/src/locales/config/{en,zh-CN}.json`
  - `test/translations.js`
- Related docs/specs/logs reviewed:
  - `apps/desktop/README.md`
  - `docs/{RepositoryGuidelines,CodeMap,Architecture,UI,CHANGELOG}.md`
  - `docs/{plans,logs}/2026-04-09.md`
  - `docs/specs/TID-20260409-apps-single-source-refactor/*`
  - `docs/specs/TID-20260409-break-message-copy-centralization/*`
- Why these are sufficient:
  - 已覆盖当前 break 文案运行时入口、locale registry lookup、真源文件、测试与现行架构文档，足以完成这次“撤销过渡层、收口单真源”的重构。

## Acceptance Criteria (AC)
- AC1: `App.tsx` 与 `break-prompt.ts` 不再 import `break-message-copy.*`，而是只通过 `i18n.ts` 读取 break 文案。
- AC2: break prompt 专属文案统一落在 `apps/desktop/src/locales/messages/*.json` 的 `ui.breakCopy.*`。
- AC3: `apps/desktop/src/locales/break-message-copy.ts` 与 `break-message-copy.json` 被删除。
- AC4: `apps/desktop/README.md`、`docs/{RepositoryGuidelines,CodeMap,Architecture,UI,CHANGELOG}.md` 与本任务 docs 明确声明 `messages` 是唯一文案真源。
- AC5: `python3 scripts/sync_desktop_locales.py`、`npm --prefix apps/desktop run typecheck`、`npm --prefix apps/desktop run build`、`npm test` 与 `python3 scripts/validate_workflow_docs.py --mode manual` 全部通过。

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
- Required Roles: orchestrator,architect,coder,tester,scribe,reality_checker
- Execution Mode Policy: multi-agent preferred; `single-agent-fallback` only when warmup is BLOCKED and scope / reason code are explicitly bounded
- Subagent Approval Policy: non-orchestrator agents must use `approval_policy = "never"`
- Escalation Route: subagent -> Orchestrator -> direct local execution | user (only for destructive or external-impact decisions)
- Safe-local Command Route: subagent -> Orchestrator -> bare repo-local command (for example `git add -- <explicit paths...>`)
- Approval Packet Fields: command / purpose / risk / rollback

## Execution Safety Block
- service_impact: 仅限桌面端 locale 真源、break prompt 文案读取路径、locale registry、回归测试与相关文档
- touches_running_service: no
- backup_required: no
- backup_plan: 依赖 Git diff、locale sync、typecheck/build/test 与 docs validator 作为回退前边界
- rollback_plan: 回退 `App.tsx`、`break-prompt.ts`、`messages/{en,zh-CN}.json`、删除的 `break-message-copy.*`、测试与相关 docs
- destructive_operations: none
- operator_approval_required: no
- rationale: 不涉及线上服务、提权、付费成本、历史重写或外部副作用

## Task Breakdown & Definition of Done (DoD)
- [x] Task-1: 把 break prompt 专属文案从 `break-message-copy.*` 并入 `messages/*.json`
  - DoD: `ui.breakCopy.*` key 落到 `messages/{en,zh-CN}.json`，并能被 registry 生成链路直接消费
- [x] Task-2: 改造前台读取路径并删除旧文件
  - DoD: `App.tsx` / `break-prompt.ts` 只走 `t()` / `tList()`，`break-message-copy.*` 被删除
- [x] Task-3: 同步架构文档与回归测试
  - DoD: 当前 README / docs / spec 不再把 `break-message-copy.*` 描述为有效架构；测试能守住单真源边界
- [x] Task-4: 完成验证闭环
  - DoD: locale sync、typecheck、build、test 与 docs validator 全部通过

## Evidence Plan (UI / E2E)
- Evidence required: no  <!-- yes | no | partial -->
- Owner: evidence_collector
- Artifact path: docs/specs/TID-20260409-apps-locale-single-source-refactor/evidence/
- Interaction validation note: 当 `interaction_impact != none` 时，此处不应保持 `no`，且需覆盖 primary / fallback / visible states
- Required states to capture:
  - loading: N/A
  - empty: N/A
  - error: N/A
  - disabled: N/A
  - success: N/A

## Observability / Debug Plan
- Logs:
  - 用 `rg -n "break-message-copy"` 审计残留引用
  - 用 `sync_desktop_locales.py` 的失败信息暴露 locale key / 文件配对问题
- Error codes:
  - N/A（脚本失败直接退出）
- Trace/metrics (optional):
  - N/A
- Debug flags (optional):
  - N/A

## Risks & Rollback
- Risks:
  - 过渡文件删除后如果有漏改 import，会直接导致 typecheck/build 失败
  - break prompt key 若未完全并入 messages，运行时会出现 key 泄漏或 fallback 偏差
  - 当前文档如果仍描述旧路径，后续维护容易再次引入第二条文案链
- Rollback plan:
  - 回退前台源码、message JSON、测试与 docs 改动；必要时再单独恢复 `break-message-copy.*`

## Sequential Phases
- phase_execution: N/A  <!-- sequential | N/A -->
- phase_confirmation_policy: N/A  <!-- no-intermediate-confirmation | manual | N/A -->
- phase_stop_conditions:
  - N/A

## Execution Plan
- Steps:
  1. 补齐本任务 spec / plan，固定“messages 单真源、config 只存元信息”的架构口径。
  2. 将 break prompt 文案并入 `messages/{en,zh-CN}.json`，并改造 `App.tsx` / `break-prompt.ts` 直接使用 `i18n.ts`。
  3. 删除 `break-message-copy.*`，同步 README / docs / CHANGELOG 与回归测试。
  4. 重生成 locale registry，跑 typecheck/build/test/docs validator，并回填结案日志。

## Definition of Done (DoD)
- `plan.md` / `testplan.md` / `README.md` 不保留 `TBD/INIT` 占位进入 DONE。
- 授权 owner、升级路径、风险与回滚已写明。
- 相关验证命令与结果可在 logs/plans 中追溯。

## Approval
- Approval needed: yes  <!-- yes | no -->
- Approved: yes（用户于 2026-04-09 明确要求“现在不要再使用这两个文件了”，并要求这一步作为 `apps/` 的彻底重构执行）
