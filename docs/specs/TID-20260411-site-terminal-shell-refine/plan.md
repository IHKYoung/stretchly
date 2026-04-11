# Task-ID: TID-20260411-site-terminal-shell-refine

## Summary
- Title: 官网去卡片化并改为终端式提示头
- Date: 2026-04-11
- Level: trivial
- Lane: fast
- Execution Profile: single-task
- Status: DONE

## Requirement Brief
- Goal restatement: 在保持当前官网极简方向、交互层和打字机效果不变的前提下，去掉中央舞台的卡片壳，并把 `Pauza>` 提示头放到左上角，让页面更像终端输出。
- In-scope:
  - 去掉首页舞台的边框、底色、阴影和高光卡片效果
  - 把 `Pauza>` 提示头改为左上角终端式位置
  - 保持当前 10 秒 hold、交互粒子和下载按钮可用
  - 同步本任务 docs、evidence、UI、CodeMap、CHANGELOG
- Out-of-scope:
  - 改动首页脚本逻辑或下载路由结构
  - 改动文案池内容
  - 重做站点整体视觉方向
- Assumptions:
  - 用户要的是更像终端的结构秩序，而不是更复杂的视觉层次
  - 去卡片化后，纸面背景本身已经足够承担页面氛围
  - 交互层可以继续保留，但不应重新做重装饰
- Risks:
  - 去掉卡片后如果结构不稳，页面容易显得散
  - `Pauza>` 如果位置太靠上或太靠左，移动端会压缩主文案空间
- Interaction impact: direct
- Primary visible flow: 用户打开首页后，先看到左上角的 `Pauza>` 终端提示头，再看到中央居中的打字机文案与底部轻提示。
- Fallback / secondary flow: 移动端与 reduced-motion 下仍保留无卡片布局和左上提示头，只弱化动态反馈。
- User-visible boundary: 仅限 `apps/site` 首页首屏结构和视觉层。
- Key visible states / transitions: shell-header-top-left、centered-output、cardless-stage、pointer-feedback-retained

## Goal
- 让首页更像一块安静的终端输出界面，而不是一张悬浮卡片。

## Scope
- In-scope:
  - `apps/site/index.html`
  - `apps/site/styles.css`
  - `apps/site/README.md`
  - `docs/specs/TID-20260411-site-terminal-shell-refine/*`
  - `docs/{plans,logs}/2026-04-11.md`
  - `docs/{CHANGELOG,CodeMap,UI}.md`
- Out-of-scope:
  - `apps/site/script.js`
  - `apps/site/copy.js`
  - `apps/site/download/**`

## Source Basis (Read Before Code)
- Related code/files reviewed:
  - `apps/site/index.html`
  - `apps/site/styles.css`
- Related docs/specs/logs reviewed:
  - `apps/site/README.md`
  - `docs/UI.md`
  - `docs/CodeMap.md`
  - `docs/specs/TID-20260411-site-interaction-refresh/*`
  - `docs/logs/2026-04-11.md`
  - `docs/plans/2026-04-11.md`
- Why these are sufficient:
  - 已覆盖当前首页真实结构、样式真源和上一轮官网交互增强边界，足以在不改脚本逻辑的前提下完成这次纯视觉结构微调。

## Acceptance Criteria (AC)
- AC1: 首页中央舞台不再显示卡片式边框、底色、阴影或高光层。
- AC2: `Pauza>` 作为单独的终端提示头放在舞台区域左上角，主文案仍保持中央居中。
- AC3: 下载按钮、打字机文案和现有粒子/点击互动继续可用，不因布局变化失效。
- AC4: 浏览器现实检查与 workflow docs validator 通过。

## Interaction Freeze (only when `interaction_impact != none`)
- Freeze status: frozen-for-implementation
- Primary flow: 首页加载 -> 左上角显示 `Pauza>` -> 中央展示打字机输出 -> 底部提示继续引导点击互动
- Fallback / secondary flow: 小屏与 reduced-motion 下保持相同终端结构，只减少动态反馈存在感
- Interaction authority / ownership boundary: 仅首页首屏结构；不改下载按钮逻辑和交互脚本事件绑定
- Visible entrypoints / handoff cues: 左上角 `Pauza>` 提示头、中央打字机输出、底部互动提示、右上角下载按钮
- In-scope interactions:
  - cardless layout
  - shell prompt top-left
  - centered output retained
- Out-of-scope interactions:
  - 粒子逻辑重写
  - 下载页样式调整
  - 文案池改写
- Interaction acceptance criteria:
  - 去卡片后页面仍然稳定、不松散
  - `Pauza>` 与主文案形成明确的终端层级
  - 既有互动反馈不因结构调整而失效

## Agent Governance
- Approval Owner: orchestrator
- Execution Profile: single-task
- Orchestrator Execution Profile: aggressive baseline uses `sandbox_mode = "danger-full-access"` + `approval_policy = "never"`
- Required Roles: orchestrator,ui_designer,coder,tester,scribe,evidence_collector
- Execution Mode Policy: multi-agent preferred; `single-agent-fallback` only when warmup is BLOCKED and scope / reason code are explicitly bounded
- Subagent Approval Policy: non-orchestrator agents must use `approval_policy = "never"`
- Escalation Route: subagent -> Orchestrator -> direct local execution | user (only for destructive or external-impact decisions)
- Safe-local Command Route: subagent -> Orchestrator -> bare repo-local command (for example `git add -- <explicit paths...>`)
- Approval Packet Fields: command / purpose / risk / rollback

## Execution Safety Block
- service_impact: 仅限 `apps/site` 首页结构样式与相关 docs/evidence
- touches_running_service: no
- backup_required: no
- backup_plan: 依赖本地静态服务器、浏览器现实检查、截图证据与 workflow docs validator
- rollback_plan: 回退 `apps/site/{index.html,styles.css}`、相关 docs 与 evidence
- destructive_operations: none
- operator_approval_required: no
- rationale: 纯静态站点的视觉结构微调，不涉及线上服务、数据、权限或外部副作用

## Task Breakdown & Definition of Done (DoD)
- [x] Task-1: 去掉首页舞台的卡片壳
  - DoD: 首页不再出现边框、底色、阴影或高光卡片效果
- [x] Task-2: 调整终端提示头与主文案层级
  - DoD: `Pauza>` 位于左上角，主文案仍稳定居中
- [x] Task-3: 完成现实检查与文档收口
  - DoD: 截图证据、CHANGELOG、UI、CodeMap 与当日 docs 更新完成

## Evidence Plan (UI / E2E)
- Evidence required: partial
- Owner: evidence_collector
- Artifact path: docs/specs/TID-20260411-site-terminal-shell-refine/evidence/
- Required states to capture:
  - loading: N/A
  - empty: N/A
  - error: 首页无 console error
  - disabled: reduced-motion 路径保持结构可读
  - success: cardless-terminal-shell

## Observability / Debug Plan
- Logs: 浏览器 console、舞台尺寸与对齐
- Error codes: 不新增
- Trace/metrics (optional): N/A
- Debug flags (optional): `prefers-reduced-motion`

## Risks & Rollback
- Risks:
  - 去卡片后页面可能显得过空
  - 左上角提示头可能在小屏下与主文案争抢空间
- Rollback plan:
  - 若终端布局不成立，优先回退 `index.html` 与 `styles.css` 到上一个带卡片版本

## Sequential Phases
- phase_execution: N/A  <!-- sequential | N/A -->
- phase_confirmation_policy: N/A  <!-- no-intermediate-confirmation | manual | N/A -->
- phase_stop_conditions:
  - N/A

## Execution Plan
- Steps:
  1. 调整首页 HTML 结构，分离左上提示头和中央输出区
  2. 去掉舞台卡片壳，保留纸面背景和交互层
  3. 做浏览器现实检查并同步 docs / evidence

## Definition of Done (DoD)
- `plan.md` / `testplan.md` / `README.md` 不保留 `TBD/INIT` 占位进入 DONE。
- 授权 owner、升级路径、风险与回滚已写明。
- 相关验证命令与结果可在 logs/plans 中追溯。

## Approval
- Approval needed: no  <!-- yes | no -->
- Approved: no-op（trivial 默认直行）
