# Task-ID: TID-20260803-break-ideas-doubling-management

## Summary
- Title: 扩充并治理休息提示语
- Date: 2026-08-03
- Level: moderate
- Lane: deep
- Execution Profile: single-task
- Status: DONE

## Requirement Brief
- Goal restatement: 在保留旧文案和运行时行为的前提下，将三份正式语言提示语精确翻倍，并建立可重复的批次/内容校验方式。
- In-scope: `zh-CN/zh-TW/en` 新增 `364/244`、批次 manifest、break-ideas README、生成器校验、内容测试、generated registry、CHANGELOG/UI/CodeMap/Architecture 与 task evidence。
- Out-of-scope: 修改旧文案、legacy locale、随机/轮播算法、设置 schema、用户自定义 ideas、官网精选同步、发布/commit。
- Assumptions: 现有 `364/244` 是认可基线；official locale 必须 key parity；新增完整休息可保持约 80-150 个中文字的较长口语正文。
- Risks: 批量创作模板化、三语翻译腔、重复/漏 ID、完整休息相邻同质、长文本布局、registry 体积增长。
- Interaction impact: direct  <!-- none | indirect | direct -->
- Primary visible flow: break 开始后从扩大后的当前语言池选取提示；微休息单条稳定，完整休息依序轮换。
- Fallback / secondary flow: ideas 关闭或列表为空继续使用 `ui.breakCopy.defaultPrompt.*`；legacy locale 使用原有 bundle/fallback。
- User-visible boundary: break prompt 中央标题与正文。
- Key visible states / transitions: microbreak selected -> typed -> stable；full break title/body typed -> 60s hold -> next interleaved entry。

## Goal
- 交付 `728` 条微休息与 `488` 条完整休息/正式语言，并让后续扩充能按批次追踪和自动验收。

## Scope
- In-scope: 正式语言内容、内容管理元数据、源边界校验、前端内容测试与文档。
- Out-of-scope: runtime state、Tauri host、旧条目改写、非正式语言翻译和外部动作。

## Source Basis (Read Before Code)
- Related code/files reviewed: `break-ideas/messages/{zh-CN,zh-TW,en}.json`、`registry{,.generated}.json`、`scripts/sync_desktop_break_ideas.py`、`lib/break-ideas.ts`、`App.tsx` typewriter/line layout、现有 break tests。
- Related docs/specs/logs reviewed: `docs/{UI,CodeMap,Architecture,RepositoryGuidelines,CHANGELOG}.md`、`TID-20260424-break-ideas-asset-migration`、`TID-20260723-release-014-github-publish`、用户本线程对语气/长度/管理的确认。
- External source basis: AOA computer vision syndrome/20-20-20；HSE DSE work routine、short frequent breaks、away from screen、stretch/change posture/look distant/blink guidance。
- Why these are sufficient: 覆盖正文真源、runtime 消费、ID/locale 生成、排版时序、历史扩充方式和本轮内容硬约束。

## Acceptance Criteria (AC)
- AC1: official locale 各为 micro `728`、long `488`，新增 ID 分别连续为 `aoa..bbz`、`ajk..ast`，三语 key/shape 完全一致。
- AC2: 批次 manifest 精确记录 608 个新 ID、类别、目标和长度预算；不保存正文，不成为第二文本源。
- AC3: 生成器在写 registry 前拒绝 official key 漂移、缺字段、空值、重复、manifest 漏/重 ID、长度越界和完整休息类别连续堆叠。
- AC4: 新文案保持现有语气组合；微休息短促，完整休息较长且自然；简中、繁中、英文均为可读本地化，不出现批量模板重复。
- AC5: registry 幂等，内容测试、全量 Vitest、typecheck、build、diff/workflow checks 通过；最长/最短和类别分布写入 evidence。

## Interaction Freeze (only when `interaction_impact != none`)
- Freeze status: locked
- Primary flow: current locale bundle -> stable selection -> existing typewriter/hold behavior。
- Fallback / secondary flow: missing/disabled ideas -> existing default prompt；legacy locale 不变。
- Interaction authority / ownership boundary: 文本事实源为 `messages/*.json`；批次 manifest 只治理；runtime helper 只消费 generated registry。
- Visible entrypoints / handoff cues: break window central `Pauza>` prompt、完整休息标题与正文。
- In-scope interactions: 文案池内容和相邻完整休息内容多样性。
- Out-of-scope interactions: 按钮、倒计时、轮换时长、选择权重、设置开关。
- Interaction acceptance criteria: 不改变选择/计时；新增正文在现有 line splitter 和 viewport 内可呈现。

## Agent Governance
- Approval Owner: orchestrator
- Execution Profile: single-task
- Orchestrator Execution Profile: 当前会话为本地 unrestricted filesystem / no-approval；仍只执行用户授权的静态内容和验证范围。
- Required Roles: orchestrator,architect,coder,tester,scribe,reality_checker
- Execution Mode Policy: 会话规则仅在用户明确要求 delegation 时允许子 agent；本任务采用 single-agent fallback。
- Subagent Approval Policy: non-orchestrator agents must use `approval_policy = "never"`；本任务未启动子 agent。
- Escalation Route: subagent -> Orchestrator -> direct local execution | user（仅范围变化、破坏性或外部动作）。
- Safe-local Command Route: 主 agent 使用 repo-local commands；手工文件通过 apply_patch，机械 JSON 合并/格式化通过受审脚本。
- Approval Packet Fields: command / purpose / risk / rollback

## Execution Safety Block
- service_impact: none。
- touches_running_service: no。
- backup_required: no。
- backup_plan: Git 提供基线；批次 manifest 记录新增 ID 范围；生成前后统计保留 evidence。
- rollback_plan: 回退三个 official message 新 ID、batch/README、sync validator/tests、generated registry/docs。
- destructive_operations: none（旧条目不删除/改写）。
- operator_approval_required: no。
- rationale: 纯静态内容和本地验证，不改变用户数据或外部系统。

## Task Breakdown & Definition of Done (DoD)
- [x] Task-1: 审计并锁定内容/运行时边界
  - DoD: 数量、ID、真源、official/legacy、轮换与长度现状已记录。
- [x] Task-2: 建立批次 manifest 和生成门禁
  - DoD: 管理 README、manifest schema、sync validation 与 tests 可失败显性。
- [x] Task-3: 完成三语内容扩充
  - DoD: 608 个概念、1824 条本地化正文落地并通过人工/自动 QA。
- [x] Task-4: registry、全量验证与交接
  - DoD: 所有门禁、统计和证据完成，工作树只含任务范围。

## Evidence Plan (UI / E2E)
- Evidence required: partial  <!-- yes | no | partial -->
- Owner: orchestrator
- Artifact path: docs/specs/TID-20260803-break-ideas-doubling-management/evidence/
- Interaction validation note: 自动化覆盖内容结构/长度/轮换/构建；尝试本地 break preview，browser backend 若仍不可用则记录缺口。
- Required states to capture:
  - loading: 不变。
  - empty: default prompt fallback tests 不变。
  - error: invalid batch/source 由 generator 显性失败。
  - disabled: breakIdeasEnabled=false 不变。
  - success: 新 ID 可被 runtime 读取，最长文案可完成 typewriter/line split。

## Observability / Debug Plan
- Logs: batch validator summary、count/category/length report、Vitest/typecheck/build/workflow output。
- Error codes: generator 使用 SystemExit 明确文件、kind、locale、ID 和失败原因。
- Trace/metrics (optional): before/added/after counts、category counts、min/p50/p90/max chars、registry bytes/build gzip。
- Debug flags (optional): `?window=break` preview。

## Risks & Rollback
- Risks: 文案主观质量无法完全自动化；大批量英译可能失去口语；精确翻倍增加 bundle 体积。
- Rollback plan: 使用 manifest ID 范围机械移除本批新增 ID并回退治理文件；旧内容和 runtime 无需迁移。

## Sequential Phases
- phase_execution: N/A  <!-- sequential | N/A -->
- phase_confirmation_policy: N/A  <!-- no-intermediate-confirmation | manual | N/A -->
- phase_stop_conditions:
  - N/A

## Execution Plan
- Steps:
  1. 新增管理 README、批次 manifest 与 sync/content tests。
  2. 以交错类别创作简中 micro/long；人工朗读筛除模板句。
  3. 生成繁中基础并逐项校正，独立本地化英文。
  4. 合并到三个 message 真源、重建 registry，运行统计/重复/长度门禁。
  5. 跑全量 tests/typecheck/build/workflow，更新 docs/evidence。

## Definition of Done (DoD)
- `plan.md` / `testplan.md` / `README.md` 在结案前不保留未填写占位。
- 授权 owner、升级路径、风险与回滚已写明。
- 相关验证命令与结果可在 logs/plans 中追溯。

## Approval
- Approval needed: yes  <!-- yes | no -->
- Approved: 用户于 2026-08-03 明确要求“现在来扩充吧，这次扩充我希望可以好好管理一下”。
