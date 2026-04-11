# Task-ID: TID-20260411-break-copy-line-layout

## Summary
- Title: 优化休息文案按句分行显示
- Date: 2026-04-11
- Level: moderate
- Lane: deep
- Execution Profile: single-task
- Status: DONE

## Requirement Brief
- Goal restatement: 优化 break prompt 中央文案的显示方式，让它按整句优先阅读，避免从任意字位截断。
- In-scope:
  - `apps/desktop/src/App.tsx` 的 break prompt 文案渲染
  - 新增前台纯函数 helper 与对应测试
  - browser preview 现实检查与任务 docs / changelog / UI / CodeMap 同步
- Out-of-scope:
  - locale 文案内容本身
  - break 触发逻辑、CTA、宿主窗口与 Rust host
  - 新增设置项
- Assumptions:
  - 当前最刺眼的问题来自自动段落换行，而不是文案本身太长
  - 中文应优先按句号/逗号等标点换行；英文至少保持一句一行
- Risks:
  - 阈值太保守会让文案被切得太碎
  - 阈值太宽则仍会残留浏览器自动折字
- Interaction impact: direct  <!-- none | indirect | direct -->
- Primary visible flow: break 弹出后，中央文案按整句或分句独立成行显示。
- Fallback / secondary flow: 英文等有空格语言在多句场景下保持一句一行；超长单句才按分句换行。
- User-visible boundary: `?window=break` 的中央文案区。
- Key visible states / transitions:
  - short prompt -> 单行
  - multi-sentence prompt -> 一句一行
  - long CJK sentence -> 按逗号/分号拆成多行
  - very long hero copy -> 自动收紧一档字号

## Goal
- 让 break prompt 的阅读节奏从“自动段落换行”升级为“按句展示”。

## Scope
- In-scope:
- `apps/desktop/src/App.tsx`
- `apps/desktop/src/lib/break-copy-layout.ts`
- `test/desktopBreakCopyLayout.js`
- `docs/specs/TID-20260411-break-copy-line-layout/*`
- `docs/{UI,CodeMap,CHANGELOG}.md`
- `docs/{plans,logs}/2026-04-11.md`
- Out-of-scope:
- `apps/desktop/src/locales/messages/*.json`
- `apps/desktop/src-tauri/src/*`
- `apps/desktop/src/styles.css`

## Source Basis (Read Before Code)
- Related code/files reviewed:
- `apps/desktop/src/App.tsx`
- `apps/desktop/src/lib/break-ideas.ts`
- `apps/desktop/src/styles.css`
- `apps/desktop/src/i18n.ts`
- Related docs/specs/logs reviewed:
- `docs/UI.md`
- `docs/CodeMap.md`
- `docs/specs/TID-20260408-break-prompt-purity-pass/plan.md`
- 用户提供的当前 break prompt 截图
- Why these are sufficient:
- 已覆盖当前 break prompt 文案拼装点、现有字体/容器宽度、idea 真源和上一轮 break prompt 收敛文档，足以判断问题位于“渲染层如何分行”，而不是文案数据源或宿主层。

## Acceptance Criteria (AC)
- AC1: 中文长文案不再从任意字位自动截断；会按整句或分句独立成行。
- AC2: 英文等多句文案保持一句一行；超长单句在需要时按分句换行。
- AC3: 新增纯函数 helper 与 Vitest 回归测试，固定住中文/英文分行规则。
- AC4: `npm test`、`npm --prefix apps/desktop run build` 与 `python3 scripts/validate_workflow_docs.py --mode manual` 通过。

## Interaction Freeze (only when `interaction_impact != none`)
- Freeze status: frozen
- Primary flow: break prompt 中央文案按整句优先显示
- Fallback / secondary flow: 超长句按分句换行；英文多句保持一句一行
- Interaction authority / ownership boundary: 仅前台文案渲染层；不改 CTA、倒计时、宿主窗口和调度逻辑
- Visible entrypoints / handoff cues: `?window=break` 打开的中央文案区
- In-scope interactions:
  - 阅读主文案
  - 在同一文案块内观察多行节奏
- Out-of-scope interactions:
  - CTA 点击
  - break 开始/结束时机
  - 声音、背景、语言切换
- Interaction acceptance criteria:
  - 长中文句不再被浏览器从中间折断
  - 多句文案的每一句具有独立行节奏
  - 超长 hero 文案不会因行数增加而失控撑满屏幕

## Agent Governance
- Approval Owner: orchestrator
- Execution Profile: single-task
- Orchestrator Execution Profile: aggressive baseline uses `sandbox_mode = "danger-full-access"` + `approval_policy = "never"`
- Required Roles: orchestrator,scribe,ui_designer,architect,coder,tester
- Execution Mode Policy: multi-agent preferred; `single-agent-fallback` only when warmup is BLOCKED and scope / reason code are explicitly bounded
- Subagent Approval Policy: non-orchestrator agents must use `approval_policy = "never"`
- Escalation Route: subagent -> Orchestrator -> direct local execution | user (only for destructive or external-impact decisions)
- Safe-local Command Route: subagent -> Orchestrator -> bare repo-local command (for example `git add -- <explicit paths...>`)
- Approval Packet Fields: command / purpose / risk / rollback

## Execution Safety Block
- service_impact: 仅限 `apps/desktop` break prompt 文案分行 helper、前台排版与对应测试/docs
- touches_running_service: no
- backup_required: no
- backup_plan: 依赖 `npm test`、desktop build、browser preview 截图与 workflow docs validator
- rollback_plan: 回退 `apps/desktop/src/{App.tsx,lib/break-copy-layout.ts}`、`test/desktopBreakCopyLayout.js` 与本任务 docs
- destructive_operations: none
- operator_approval_required: no
- rationale: 纯前台 UI 排版修复，不涉及数据、权限、外部副作用或线上服务

## Task Breakdown & Definition of Done (DoD)
- [x] Task-1: 抽离文案切行 helper
  - DoD: 长文案按整句优先、超长句按分句换行，并有纯函数测试
- [x] Task-2: 调整 break prompt 前台渲染
  - DoD: break prompt 会按 line list 渲染标题/正文，并在超多行时自动收紧 hero 字号
- [x] Task-3: 完成 preview 取证与文档同步
  - DoD: screenshot 落盘，UI/CodeMap/CHANGELOG 与 daily docs 同步完成

## Evidence Plan (UI / E2E)
- Evidence required: partial  <!-- yes | no | partial -->
- Owner: evidence_collector
- Artifact path: docs/specs/TID-20260411-break-copy-line-layout/evidence/
- Interaction validation note: 当 `interaction_impact != none` 时，此处不应保持 `no`，且需覆盖 primary / fallback / visible states
- Required states to capture:
  - loading: N/A
  - empty: N/A
  - error: N/A
  - disabled: N/A
  - success: break prompt 中央文案按句分行显示

## Observability / Debug Plan
- Logs:
- 不新增 runtime logs；通过 Vitest 与 browser preview 截图验证
- Error codes:
- N/A
- Trace/metrics (optional):
- N/A
- Debug flags (optional):
- N/A

## Risks & Rollback
- Risks:
- 文案切分阈值可能需要继续按真实截图微调
- Rollback plan:
- 回退 helper / JSX 渲染逻辑与相关测试

## Sequential Phases
- phase_execution: N/A  <!-- sequential | N/A -->
- phase_confirmation_policy: N/A  <!-- no-intermediate-confirmation | manual | N/A -->
- phase_stop_conditions:
  - N/A

## Execution Plan
- Steps:
  1. 新增前台纯函数 helper，把 break 文案拆成可控的 line list。
  2. 让 `BreakWindow` 渲染 title/body line list，并对超多行标题自动收紧字号。
  3. 运行 Vitest、desktop build、workflow docs validator，并用 browser preview 截图取证。

## Definition of Done (DoD)
- `plan.md` / `testplan.md` / `README.md` 不保留 `TBD/INIT` 占位进入 DONE。
- 授权 owner、升级路径、风险与回滚已写明。
- 相关验证命令与结果可在 logs/plans 中追溯。

## Approval
- Approval needed: yes  <!-- yes | no -->
- Approved: yes（用户已明确要求直接优化这块 break 文案显示）
