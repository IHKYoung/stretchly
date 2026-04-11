# Task-ID: TID-20260411-site-landing-page

## Summary
- Title: 搭建单页官网与下载入口
- Date: 2026-04-11
- Level: moderate
- Lane: deep
- Execution Profile: single-task
- Status: DONE

## Requirement Brief
- Goal restatement: 为 Pauza 搭建一个现代、高级、简洁的单页官网，在一屏内讲清它是什么、它能做什么，并提供下载按钮与独立的下载跳转路由。
- In-scope:
  - 新增 `apps/site` 静态官网目录
  - 首页单页视觉和交互
  - `apps/site/download/**` 下载跳转页与配置文件
  - 根级开发入口、README 与 docs 索引更新
  - 本地浏览器现实检查与截图证据
- Out-of-scope:
  - 真实 Vercel 部署
  - GitHub Releases API / 自动抓取最新版本
  - 多页网站、博客、CMS、多语言官网
- Assumptions:
  - 当前官网优先承担“品牌说明 + 下载入口”职责，不需要复杂站点结构
  - 下载源后续大概率来自 GitHub Releases，因此需要站内稳定跳转层
  - 不新增依赖比复用复杂前端栈更重要
- Risks:
  - 视觉做得过满会和 Pauza 的“安静、克制”产品气质冲突
  - 如果下载地址未配置，又没有 fallback，会让 CTA 失效
- Interaction impact: direct  <!-- none | indirect | direct -->
- Primary visible flow: 用户打开首页，在同一屏读到产品定位、核心能力和下载按钮，然后进入平台下载跳转页。
- Fallback / secondary flow: 若下载目标未配置，跳转页显示 fallback 提示和手动打开入口，而不是空白页。
- User-visible boundary: 首页与下载跳转页
- Key visible states / transitions:
  - page load reveal
  - hover / spotlight response
  - CTA click
  - redirect loading
  - redirect fallback

## Goal
- 做一个足够成熟、可以直接拿去继续打磨和部署的 Pauza 单页官网雏形。

## Scope
- In-scope:
  - `apps/site/**`
  - `package.json`（网站本地预览脚本）
  - `README.md`
  - `docs/{RepositoryGuidelines,CodeMap,CHANGELOG}.md`
  - `docs/specs/TID-20260411-site-landing-page/*`
  - `docs/{plans,logs}/2026-04-11.md`
- Out-of-scope:
  - `apps/desktop/**`
  - `apps/desktop/src-tauri/**`
  - 真实下载源部署和发布动作

## Source Basis (Read Before Code)
- Related code/files reviewed:
  - `package.json`
- Related docs/specs/logs reviewed:
  - `docs/RepositoryGuidelines.md`
  - `docs/CodeMap.md`
  - `README.md`
  - `docs/CHANGELOG.md`
- Why these are sufficient:
  - 当前仓库尚无网站实现；只需要确认目录边界、根脚本现状和现有产品叙述口径即可为新站点建立独立实现。

## Acceptance Criteria (AC)
- AC1: 仓库新增一个独立的 `apps/site` 静态官网目录，能本地直接预览。
- AC2: 首页在单屏内用简洁文案表达 Pauza 的产品定位、能力摘要和下载入口。
- AC3: 下载按钮统一走 `apps/site/download/**` 稳定路由，并通过独立配置文件管理真实目标地址。
- AC4: 页面具有现代、高级、克制的视觉方向，并包含至少一组有意义的入场 / hover / pointer 动画。
- AC5: 完成浏览器现实检查和截图证据，docs / workflow validator 通过。

## Interaction Freeze (only when `interaction_impact != none`)
- Freeze status: frozen
- Primary flow: 首页首屏完成产品理解与下载决策，CTA 进入下载跳转页。
- Fallback / secondary flow: 跳转页若未配置真实地址，则停留在 fallback 状态并允许手动打开或后续替换链接。
- Interaction authority / ownership boundary: 本轮只改站点前台表现与下载路由，不接入真实 API 或部署系统。
- Visible entrypoints / handoff cues:
  - hero 下载按钮
  - 下载页 redirect 状态文案
- In-scope interactions:
  - 页面 reveal / hover / spotlight
  - capability reel
  - CTA click -> redirect
  - redirect success / fallback
- Out-of-scope interactions:
  - 用户账号
  - 站内搜索
  - 文档页 / 博客页
- Interaction acceptance criteria:
  - 用户无需滚动很长页面就能理解产品和找到下载入口
  - 下载路由在有无真实地址两种情况下都保持可解释的状态
- Validator expectation: direct interaction 字段已补齐。

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
- service_impact: 仅限新增官网静态目录、下载跳转页、根脚本与 docs 口径
- touches_running_service: no
- backup_required: no
- backup_plan: 依赖本地静态服务器、浏览器现实检查、截图证据与 workflow docs validator
- rollback_plan: 删除 `apps/site` 与相关 docs / scripts 接线后恢复
- destructive_operations: none
- operator_approval_required: no
- rationale: 不涉及运行中服务、数据迁移、提权、外部付费或破坏性操作

## Task Breakdown & Definition of Done (DoD)
- [x] Task-1: 新建官网目录并实现单页视觉
  - DoD: `apps/site/index.html`、`styles.css`、`script.js` 完成，首页具备完整视觉和下载 CTA
- [x] Task-2: 实现下载跳转层、文档与现实检查
  - DoD: `apps/site/download/**` 可工作，docs 更新，浏览器截图证据与验证命令通过

## Evidence Plan (UI / E2E)
- Evidence required: yes  <!-- yes | no | partial -->
- Owner: evidence_collector
- Artifact path: docs/specs/TID-20260411-site-landing-page/evidence/
- Interaction validation note: 需至少覆盖首页首屏和下载跳转页的 loading / fallback 状态。
- Required states to capture:
  - loading: 下载跳转页倒计时 / 正在跳转状态
  - empty: N/A
  - error: 下载目标未配置时的 fallback 提示
  - disabled: N/A
  - success: 首页首屏完整视觉与 CTA

## Observability / Debug Plan
- Logs: 下载页未配置目标时输出 `console.warn`
- Error codes: N/A（纯静态前台）
- Trace/metrics (optional): 本地浏览器 snapshot / screenshot
- Debug flags (optional): `targets.js` 中通过是否留空 URL 控制 redirect / fallback

## Risks & Rollback
- Risks:
  - 页面信息密度过低，用户读完仍不知道它能做什么
  - 视觉效果过重，破坏 Pauza 的产品气质
  - 下载跳转配置缺失导致 CTA 变死链
- Rollback plan:
  - 回退 `apps/site/**`、根脚本与 docs 口径
  - 重新执行 workflow docs validator

## Sequential Phases
- phase_execution: N/A  <!-- sequential | N/A -->
- phase_confirmation_policy: N/A  <!-- no-intermediate-confirmation | manual | N/A -->
- phase_stop_conditions:
  - N/A

## Execution Plan
- Steps:
  1. 实现官网单页首屏布局、文案、动画和 CTA
  2. 建立下载跳转文件夹和集中配置文件
  3. 更新根脚本与文档索引
  4. 本地预览并做浏览器现实检查与截图

## Definition of Done (DoD)
- `plan.md` / `testplan.md` / `README.md` 不保留 `TBD/INIT` 占位进入 DONE。
- 授权 owner、升级路径、风险与回滚已写明。
- 相关验证命令与结果可在 logs/plans 中追溯。

## Approval
- Approval needed: yes  <!-- yes | no -->
- Approved: yes（用户已明确要求我直接尝试做一个网站）
