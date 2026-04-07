# Task-ID: TID-20260403-hig-tailwind-redesign

## Summary
- Title: Tauri 前台改版：Tailwind + Shadcn + Apple HIG
- Date: 2026-04-03
- Level: moderate
- Lane: deep
- Execution Profile: single-task
- Status: DONE

## Requirement Brief
- Goal restatement: 用 Tailwind CSS v4 + shadcn 风格组件重做 `apps/desktop` 前台，让设置页和 break prompt 更接近 Apple HIG 的安静、克制、分组明确的桌面体验。
- In-scope:
  - `apps/desktop` 前端依赖、Tailwind/Vite 基座、shadcn primitives、设置页布局、break prompt 视觉与多语言文案
  - UI 证据采集与相关 docs 更新
- Out-of-scope:
  - Rust host / Tauri command contract 变更
  - Electron legacy 界面
  - 调度策略、tray 菜单、shortcut 行为语义调整
- Assumptions:
  - 用户要求即视为新增 Tailwind/shadcn 前端依赖的明确授权
  - `DesktopSnapshot` / `PauzaSettings` 结构保持不变
  - 浏览器 preview 可以作为设置页主证据，break prompt 证据允许通过 mocked runtime 采集
- Risks:
  - 新前端依赖引入的体积和维护成本
  - mocked break evidence 与真实 Tauri runtime 之间存在轻微差异
- Interaction impact: direct
- Primary visible flow: 打开主设置页，浏览 overview hero / live overview，按 grouped settings 修改节奏、信号、通用设置，必要时展开高级项，最后保存
- Fallback / secondary flow: 打开 `?window=break`，查看环形倒计时、主副 CTA（Done / Later / Skip），在 break 结束或跳过后退出
- User-visible boundary: 仅 `apps/desktop/src/App.tsx` 渲染出的主窗口与 break window
- Key visible states / transitions: loading、synced、dirty、advanced expanded、active break、manual-awaiting、error banner

## Goal
- 为 Tauri 前台建立稳定的 Tailwind/shadcn 设计基座，并把主设置页、保存 rail、break prompt 一次性收敛到更统一的 Apple HIG 风格。

## Scope
- In-scope:
  - `apps/desktop/package.json`
  - `apps/desktop/components.json`
  - `apps/desktop/vite.config.ts`
  - `apps/desktop/tsconfig.json`
  - `apps/desktop/src/App.tsx`
  - `apps/desktop/src/styles.css`
  - `apps/desktop/src/components/ui/*`
  - `apps/desktop/src/lib/utils.ts`
  - `apps/desktop/src/locales/{zh-CN,en}.json`
  - `docs/specs/TID-20260403-hig-tailwind-redesign/evidence/*`
  - `docs/UI.md`、`docs/CodeMap.md`、`docs/RepositoryGuidelines.md`、`docs/Architecture.md`、`docs/CHANGELOG.md`
- Out-of-scope:
  - `apps/desktop/src-tauri/src/*.rs`
  - 根级 Electron 旧 UI 资产
  - 新增后端数据结构或存储迁移

## Source Basis (Read Before Code)
- Related code/files reviewed:
  - `apps/desktop/src/App.tsx`
  - `apps/desktop/src/styles.css`
  - `apps/desktop/src/i18n.ts`
  - `apps/desktop/src/locales/{zh-CN,en}.json`
  - `apps/desktop/package.json`
  - `apps/desktop/vite.config.ts`
  - `apps/desktop/tsconfig.json`
  - `apps/desktop/src-tauri/tauri.conf.json`
- Related docs/specs/logs reviewed:
  - `AGENTS.md`
  - `docs/RepositoryGuidelines.md`
  - `docs/CodeMap.md`
  - `docs/UI.md`
  - `docs/Architecture.md`
  - `docs/logs/2026-04-03.md`
  - `docs/plans/2026-04-03.md`
- Why these are sufficient:
  - 已覆盖前端入口、现有视觉层、i18n、构建配置、窗口尺寸和任务历史，可在不触碰 Rust host 的前提下完整改造 UI 层。

## Acceptance Criteria (AC)
- AC1: `apps/desktop` 前台改为 Tailwind CSS v4 + shadcn 风格组件基座，不再依赖旧版整页手写样式类。
- AC2: 主设置页符合 Apple HIG 风格的 grouped settings 结构，默认内容清晰分成 overview、节奏、信号、通用和折叠高级区。
- AC3: break prompt 使用更聚焦的单卡片结构，包含环形倒计时、清晰的 Done / Later / Skip 层级和更克制的视觉语言。
- AC4: 现有 Tauri 命令面和 preview fallback 继续可用，`npm --prefix apps/desktop run build` 通过，并留存 UI 证据。

## Interaction Freeze
- Freeze status: frozen
- Primary flow: `主设置页 -> grouped settings -> save rail`
- Fallback / secondary flow: `?window=break -> countdown prompt -> action buttons`
- Interaction authority / ownership boundary: 仅 React 前台排版、控件和微文案；Rust host 提供状态与命令
- Visible entrypoints / handoff cues:
  - 主窗口默认进入设置页
  - query `?window=break` 切到 break prompt
  - dirty/synced badge 与保存 rail 是保存 handoff cue
- In-scope interactions:
  - segmented control、switch、accordion、save/revert/defaults、break CTA
- Out-of-scope interactions:
  - tray menu flow
  - system notification flow
  - Electron legacy 页面
- Interaction acceptance criteria:
  - grouped settings 默认可读、可改、可保存
  - break prompt 在 active state 下具有明确主副操作
  - preview 与 mocked break evidence 可证明两个入口的最终形态

## Agent Governance
- Approval Owner: orchestrator
- Execution Profile: single-task
- Orchestrator Execution Profile: aggressive baseline uses `sandbox_mode = "danger-full-access"` + `approval_policy = "never"`
- Required Roles: orchestrator,architect,coder,tester,scribe
- Execution Mode Policy: multi-agent preferred; `single-agent-fallback` only when warmup is BLOCKED and scope / reason code are explicitly bounded
- Subagent Approval Policy: non-orchestrator agents must use `approval_policy = "never"`
- Escalation Route: subagent -> Orchestrator -> direct local execution | user
- Safe-local Command Route: subagent -> Orchestrator -> bare repo-local command
- Approval Packet Fields: command / purpose / risk / rollback

## Execution Safety Block
- service_impact: 仅调整 Tauri 前台视觉、样式系统和组件组织，不改后端调度与数据真源
- touches_running_service: no
- backup_required: no
- backup_plan: 依赖 VCS 回退；同时保留 build 与 Playwright 证据作为旁证
- rollback_plan: 回退 `apps/desktop/**` 的本轮前端改版和相关 docs
- destructive_operations: 以新的 Tailwind/shadcn 架构替换旧 React 前台的 CSS/JS 组织
- operator_approval_required: no
- rationale: 用户已明确要求使用 Tailwind + Shadcn 并遵守 Apple HIG；本轮不涉及运行中服务、数据删除或外部副作用

## Task Breakdown & Definition of Done (DoD)
- [x] Task-1: 接入 Tailwind CSS v4 与 shadcn 风格组件基座
  - DoD: `components.json`、`vite.config.ts`、`src/components/ui/*`、`src/lib/utils.ts` 落盘并可编译
- [x] Task-2: 重写主设置页信息架构
  - DoD: overview hero、grouped settings、advanced accordion、save rail 全部接入新视觉语言
- [x] Task-3: 重写 break prompt 视觉层
  - DoD: 环形倒计时、主副 CTA、微文案和空状态表现收敛到统一风格
- [x] Task-4: 采集 UI 证据并更新 docs
  - DoD: screenshot / snapshot / console log 与 logs/plans/specs/UI docs 同步完成

## Evidence Plan (UI / E2E)
- Evidence required: yes
- Owner: orchestrator（single-agent-fallback）
- Artifact path: docs/specs/TID-20260403-hig-tailwind-redesign/evidence/
- Required states to capture:
  - loading: 通过主设置页结构 snapshot 验证完成渲染
  - empty: break prompt 的 cleared state 不作为本轮主证据
  - error: browser console error log
  - disabled: dirty=false 时保存 rail 的保存按钮禁用
  - success: 设置页 full-page screenshot + active break prompt snapshot

## Observability / Debug Plan
- Logs:
  - `docs/specs/TID-20260403-hig-tailwind-redesign/evidence/browser-console-errors.log`
- Error codes:
  - 无新增运行时错误码；继续沿用前端 `error banner` 的 message 直显策略
- Trace/metrics (optional):
  - 暂不引入
- Debug flags (optional):
  - 使用浏览器 preview + mocked `__TAURI_INTERNALS__` 作为 UI 验证开关

## Risks & Rollback
- Risks:
  - Tailwind/shadcn 组件层会增加前端依赖与 bundle 体积
  - break prompt 的 mocked evidence 不能完全替代真实 Tauri runtime 截图
- Rollback plan:
  - 回退 `apps/desktop/**` 本轮改动并恢复旧版 `App.tsx` / `styles.css`
  - 删除 `src/components/ui/*`、`components.json` 和相关依赖声明

## Sequential Phases
- phase_execution: N/A
- phase_confirmation_policy: N/A
- phase_stop_conditions:
  - N/A

## Execution Plan
- Steps:
  1. 阅读现有前台和 docs，确认 source basis 与 interaction boundary
  2. 接入 Tailwind/shadcn 基座并建立复用组件
  3. 重写设置页与 break prompt
  4. 执行 build 与 Playwright 证据采集
  5. 回填 specs/logs/plans 与索引文档

## Definition of Done (DoD)
- `plan.md` / `testplan.md` / `README.md` 不保留 `TBD/INIT`
- 新前台能通过 `npm --prefix apps/desktop run build`
- 至少一张设置页截图与一份 break prompt 证据落盘
- logs/plans/specs/UI 索引可追溯到本次实现

## Approval
- Approval needed: yes
- Approved: yes（用户本轮请求即对 UI 重设计与依赖接入给出明确授权；Execution Safety Block 不包含额外高风险人工门禁）
