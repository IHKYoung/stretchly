# Task-ID: TID-20260411-site-typewriter-redesign

## Summary
- Title: 官网重设计为极简打字机单页
- Date: 2026-04-11
- Level: moderate
- Lane: deep
- Execution Profile: single-task
- Status: DONE

## Requirement Brief
- Goal restatement: 将刚做好的官网首页重构成更极简的单页版本：纯白纸面背景、中央命令行式打字机文案展示、右上角一个下载按钮。
- In-scope:
  - 重构 `apps/site/index.html`、`styles.css`、`script.js`
  - 新增站点文案源 `apps/site/copy.js`
  - 新增本地字体资产 `apps/site/fonts/LXGWWenKaiScreen.ttf`
  - 保留并复用既有下载稳定路由
  - docs / evidence / workflow validator 同步
- Out-of-scope:
  - 接入真实 GitHub 下载地址
  - 改动 `apps/site/download/**` 的整体结构
  - 多页官网、多语言官网、部署脚本
- Assumptions:
  - 官网首页的最佳表达不是信息堆叠，而是让提醒文案本身成为主视觉
  - 用户说的“落霞孤鹜”可通过仓库中已存在的 `LXGW WenKai Screen` 字体实现
  - 下载按钮当前只需作为占位入口，后续再接 GitHub 链接
- Risks:
  - 过度极简后，首页可能显得信息不足
  - 文案过长或打字节奏不佳会影响阅读体验
  - 复制字体资产若路径错误会导致视觉风格不成立
- Interaction impact: direct  <!-- none | indirect | direct -->
- Primary visible flow: 用户打开首页，观看动态提醒文案，感知 Pauza 的语气，然后点击右上角下载按钮。
- Fallback / secondary flow: 用户点击下载按钮后进入 `/download/releases/`；若真实地址未配置，下载页显示 fallback。
- User-visible boundary: 官网首页与下载稳定路由页
- Key visible states / transitions:
  - initial empty frame
  - typing
  - hold
  - deleting
  - CTA click
  - download fallback

## Goal
- 让 Pauza 官网首页从“介绍产品”变成“直接让人感受到产品”。

## Scope
- In-scope:
  - `apps/site/index.html`
  - `apps/site/styles.css`
  - `apps/site/script.js`
  - `apps/site/copy.js`
  - `apps/site/fonts/LXGWWenKaiScreen.ttf`
  - `apps/site/README.md`
  - `docs/{CHANGELOG,CodeMap,UI,Architecture}.md`
  - `docs/specs/TID-20260411-site-typewriter-redesign/*`
  - `docs/{plans,logs}/2026-04-11.md`
- Out-of-scope:
  - `apps/site/download/**` 的行为协议
  - `apps/desktop/**` 的运行逻辑
  - 线上部署与真实下载源

## Source Basis (Read Before Code)
- Related code/files reviewed:
  - `apps/site/index.html`
  - `apps/site/styles.css`
  - `apps/site/script.js`
  - `apps/site/download/targets.js`
  - `apps/desktop/src/locales/messages/zh-CN.json`
  - `apps/desktop/src/styles.css`
- Related docs/specs/logs reviewed:
  - `apps/site/README.md`
  - `docs/CHANGELOG.md`
  - `docs/CodeMap.md`
  - `docs/UI.md`
  - `docs/Architecture.md`
  - `docs/specs/TID-20260411-site-landing-page/README.md`
  - `docs/specs/TID-20260411-site-landing-page/evidence/README.md`
- Why these are sufficient:
  - 已覆盖当前官网实现、下载路由边界、App 内嵌提醒文案真源、现有字体资源位置，以及上一个官网任务留下的实现和证据上下文，足以支撑本轮重设计。

## Acceptance Criteria (AC)
- AC1: 首页改成纯白纸面背景上的极简单舞台结构，中央主文案成为唯一核心视觉。
- AC2: 页面通过打字机效果循环展示来自 App 内嵌提醒文案的精选句子，并遵循“输入 -> 停留约 5 秒 -> 退格 -> 下一句”。
- AC3: 首页主文案优先使用本地 `LXGW WenKai Screen` 字体资源，不依赖外部字体 CDN。
- AC4: 页面只保留一个简洁下载按钮，并继续链接到站内稳定下载路由。
- AC5: 完成浏览器现实检查、截图证据与 workflow docs 校验。

## Interaction Freeze (only when `interaction_impact != none`)
- Freeze status: frozen
- Primary flow: 用户进入首页 -> 看到中央打字机文案 -> 点击右上角下载按钮 -> 进入下载稳定路由
- Fallback / secondary flow: 下载稳定路由未配置真实地址时显示 fallback，而不是首页做禁用态
- Interaction authority / ownership boundary: 本轮只重构官网首页展示层，不改下载页协议和真实下载地址配置
- Visible entrypoints / handoff cues:
  - 中央打字机文案
  - 右上角下载按钮
  - 下载页 fallback 提示
- In-scope interactions:
  - typewriter loop
  - caret blink
  - CTA hover / active
  - CTA click -> redirect route
- Out-of-scope interactions:
  - 平台选择器
  - 多按钮 CTA
  - 官网多节滚动
- Interaction acceptance criteria:
  - 首页不再被多块信息分散注意力
  - 动态文案可读、节奏自然
  - 下载入口始终清晰可见
- Validator expectation: direct interaction 字段已补齐，Evidence required 必须为 yes。

## Agent Governance
- Approval Owner: orchestrator
- Execution Profile: single-task
- Orchestrator Execution Profile: aggressive baseline uses `sandbox_mode = "danger-full-access"` + `approval_policy = "never"`
- Required Roles: orchestrator,ui_designer,architect,coder,tester,scribe,evidence_collector
- Execution Mode Policy: multi-agent preferred; `single-agent-fallback` only when warmup is BLOCKED and scope / reason code are explicitly bounded
- Subagent Approval Policy: non-orchestrator agents must use `approval_policy = "never"`
- Escalation Route: subagent -> Orchestrator -> direct local execution | user (only for destructive or external-impact decisions)
- Safe-local Command Route: subagent -> Orchestrator -> bare repo-local command (for example `git add -- <explicit paths...>`)
- Approval Packet Fields: command / purpose / risk / rollback

## Execution Safety Block
- service_impact: 仅限 `apps/site` 首页重构、字体/文案资源新增与 docs 同步
- touches_running_service: no
- backup_required: no
- backup_plan: 通过本地静态服务器、浏览器现实检查、截图证据与 workflow docs validator 验证
- rollback_plan: 回退 `apps/site` 首页相关文件和本任务 docs 后恢复到上一版站点
- destructive_operations: none
- operator_approval_required: no
- rationale: 不涉及线上服务、数据迁移、权限提升或外部付费动作

## Task Breakdown & Definition of Done (DoD)
- [x] Task-1: 重构官网首页为极简纸面打字机布局
  - DoD: `apps/site/index.html` 与 `styles.css` 改为单舞台结构，右上角只保留一个下载按钮
- [x] Task-2: 接入文案真源、字体资源与交互脚本
  - DoD: `copy.js`、`script.js`、`fonts/` 完成，打字机循环工作正常
- [x] Task-3: 完成文档、证据与现实检查
  - DoD: docs 更新、截图落盘、workflow docs validator 通过

## Evidence Plan (UI / E2E)
- Evidence required: yes  <!-- yes | no | partial -->
- Owner: evidence_collector
- Artifact path: docs/specs/TID-20260411-site-typewriter-redesign/evidence/
- Interaction validation note: 需覆盖首页桌面端、首页移动端、以及下载路由 fallback 状态
- Required states to capture:
  - loading: 打字机正在展示文案中的中间状态（可通过现实检查描述）
  - empty: 首次进入前的空白 frame（可选）
  - error: 下载地址未配置时的 fallback 提示
  - disabled: N/A
  - success: 首页桌面端与移动端完成态

## Observability / Debug Plan
- Logs: 下载页继续使用 `console.warn` 提示缺失下载目标；首页不应产生额外错误
- Error codes: N/A（纯静态页面）
- Trace/metrics (optional): 本地浏览器 snapshot / screenshot
- Debug flags (optional): `copy.js` 的文案数组和 `targets.js` 的 URL 留空状态

## Risks & Rollback
- Risks:
  - 页面太空，用户只看到“好看”却不知道这是什么
  - 字体资产路径出错，导致主视觉气质丢失
  - 动态文案换行在移动端失控
- Rollback plan:
  - 回退 `apps/site` 首页相关文件
  - 保持 `apps/site/download/**` 不变
  - 重新执行 workflow docs validator

## Sequential Phases
- phase_execution: N/A  <!-- sequential | N/A -->
- phase_confirmation_policy: N/A  <!-- no-intermediate-confirmation | manual | N/A -->
- phase_stop_conditions:
  - N/A

## Execution Plan
- Steps:
  1. 精选一组来自 App 的提醒文案，建立首页 `copy.js`
  2. 复制本地字体资产到 `apps/site/fonts/`
  3. 重写首页 HTML / CSS / JS 为极简打字机单页
  4. 更新 docs 并做现实检查与截图

## Definition of Done (DoD)
- `plan.md` / `testplan.md` / `README.md` 不保留 `TBD/INIT` 占位进入 DONE。
- 授权 owner、升级路径、风险与回滚已写明。
- 相关验证命令与结果可在 logs/plans 中追溯。

## Approval
- Approval needed: yes  <!-- yes | no -->
- Approved: yes（用户已明确给出新的官网设计想法并要求整体设计实现）
