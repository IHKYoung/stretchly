# Task-ID: TID-20260407-rhythm-chip-presets

## Summary
- Title: 节奏页核心时间改为预设芯片
- Date: 2026-04-07
- Level: moderate
- Lane: deep
- Execution Profile: single-task
- Status: DONE

## Requirement Brief
- Goal restatement: 将节奏标签页里最高频的四个时间设置从自由数字 stepper 改成预设芯片，让用户一击完成选择；提前提醒和延后时间继续保留低频 stepper。
- In-scope: `apps/desktop/src/App.tsx` 中节奏页顶部微休息/长休息两组设置的交互重排；新增可复用的预设芯片按钮样式；补齐本任务 docs 与 UI 证据。
- Out-of-scope: 不修改 `PauzaSettings` 数据结构；不新增新的节奏选项值；不改 break 窗口、托盘、Rust host、保存逻辑或其他分类布局。
- Assumptions: 用户提供的预设集合即为本轮唯一候选值；微休息保持 `5/10/15/20/30` 分钟间隔和 `15/20/30/60` 秒时长，长休息保持 `2/3/4/5` 轮频率和 `3/5/10/15` 分钟时长；次要设置仍以 stepper 足够表达。
- Risks: 芯片行在较窄窗口下可能换行，需要保持清晰的对齐和点击命中区；切换到芯片后需要确保选中态足够醒目，避免用户误判当前值。
- Interaction impact: direct  <!-- none | indirect | direct -->
- Primary visible flow: 用户进入“节奏”标签页，直接在微休息和长休息区块里点击预设芯片完成核心时间选择，并通过开关控制该类休息是否启用。
- Fallback / secondary flow: 用户继续在下方“提前提醒”和“延后”分区使用现有 stepper 微调低频设置，无需学习新控件。
- User-visible boundary: `apps/desktop/src/App.tsx` 的主设置页节奏分类，仅影响两组核心时间输入的交互表达和视觉态。
- Key visible states / transitions: 芯片默认态、hover、focus、selected；微休息/长休息开关切换；次要 stepper 保持原行为；保存状态仍沿用当前自动保存提示。

## Goal
- 让节奏页最常改的时间设置更像“拍板选择”，而不是“想一个数字再输入”。

## Scope
- In-scope:
  - 微休息区块改为“开关 + 间隔芯片 + 时长芯片”。
  - 长休息区块改为“开关 + 频率芯片 + 时长芯片”。
  - 选中芯片使用白底和阴影，未选中芯片使用浅灰底。
  - “提前提醒”和“延后”继续使用现有 `CompactNumber` stepper。
  - 为本次 direct-interaction 改造补齐 evidence、plans、logs、CHANGELOG/UI 文档。
- Out-of-scope:
  - 新增 preset 编辑能力或自由输入回退。
  - 修改微休息/长休息底层调度、通知或持久化契约。
  - 重排“偏好”标签页。

## Source Basis (Read Before Code)
- Related code/files reviewed:
  - `apps/desktop/src/App.tsx`
  - `apps/desktop/src/components/ui/segmented-control.tsx`
  - `apps/desktop/src/styles.css`
  - `apps/desktop/src/locales/zh-CN.json`
- Related docs/specs/logs reviewed:
  - `docs/RepositoryGuidelines.md`
  - `docs/CodeMap.md`
  - `docs/UI.md`
  - `docs/SettingsInventory.md`
  - `docs/plans/2026-04-07.md`
  - `docs/logs/2026-04-07.md`
- Why these are sufficient: 本轮只影响 Tauri 主设置页的前端交互与文档，不改宿主命令、设置 schema 或多语言 key；上述文件已覆盖当前布局、基础控件风格、节奏设置边界和当日执行上下文。

## Acceptance Criteria (AC)
- AC1: 节奏页的微休息和长休息核心时间设置改为预设芯片交互，分别只暴露用户指定的四组 preset 集合。
- AC2: 选中芯片具备白底 + 阴影的高亮态，未选中芯片保持浅灰底，并支持清晰的 hover / focus / pressed 反馈。
- AC3: “提前提醒”和“延后”继续使用现有 stepper，不被误改成芯片或自由输入。
- AC4: `npm --prefix apps/desktop run typecheck`、`npm --prefix apps/desktop run build` 和浏览器预览证据通过，能证明新交互稳定渲染。

## Interaction Freeze (only when `interaction_impact != none`)
- Freeze status: frozen for implementation
- Primary flow: 进入节奏页后，用户在两个区块内用一次点击选定微休息间隔/时长和长休息频率/时长。
- Fallback / secondary flow: 低频设置仍通过 stepper 做细调；不新增二级弹窗或额外说明层。
- Interaction authority / ownership boundary: 仅调整主设置页节奏分类的输入控件形态；设置项语义、选项值、自动保存和其余分类布局不在本轮改动范围。
- Visible entrypoints / handoff cues: 顶部标签切到“节奏”；区块标题左对齐、开关右对齐；每行标签后跟一组可点击 preset 芯片和单位。
- In-scope interactions: 点击 preset 芯片选值、切换微休息/长休息开关、继续使用 stepper 调整提前提醒和延后时间。
- Out-of-scope interactions: break prompt CTA、tray 操作、strict mode、smart pause、language/autostart。
- Interaction acceptance criteria: 用户不需要输入或思考具体数字，即可在节奏页完成最高频的核心时间设置；次要设置仍保留 stepper 以避免信息密度过高。

## Agent Governance
- Approval Owner: orchestrator
- Execution Profile: single-task
- Orchestrator Execution Profile: aggressive baseline uses `sandbox_mode = "danger-full-access"` + `approval_policy = "never"`
- Required Roles: orchestrator,ui_designer,architect,coder,tester,scribe
- Execution Mode Policy: multi-agent preferred; `single-agent-fallback` only when warmup is BLOCKED and scope / reason code are explicitly bounded
- Subagent Approval Policy: non-orchestrator agents must use `approval_policy = "never"`
- Escalation Route: subagent -> Orchestrator -> direct local execution | user (only for destructive or external-impact decisions)
- Safe-local Command Route: subagent -> Orchestrator -> bare repo-local command (for example `git add -- <explicit paths...>`)
- Approval Packet Fields: command / purpose / risk / rollback

## Execution Safety Block
- service_impact: 仅限主设置页节奏分类的输入控件表达和视觉反馈。
- touches_running_service: no
- backup_required: no
- backup_plan: 以现有 Git 差异、`typecheck/build`、浏览器预览证据和任务 docs 为回滚边界。
- rollback_plan: 回退 `apps/desktop/src/App.tsx` 与本任务 docs / evidence / changelog / UI 摘要。
- destructive_operations: 用 preset 芯片替换节奏页顶部四个核心时间 stepper。
- operator_approval_required: no
- rationale: 本轮是本地前端 UI 改造，不涉及数据删除、权限、外部资源、付费成本或运行中服务影响。

## Task Breakdown & Definition of Done (DoD)
- [x] Task-1: 为节奏页实现预设芯片控件
  - DoD: `App.tsx` 中出现可复用的芯片组，支持 preset 渲染、选中态与单位展示。
- [x] Task-2: 重排节奏页顶部微休息/长休息区块
  - DoD: 微休息与长休息均切换为“标题+开关+两行 preset”的结构，且下方 stepper 分区保持不变。
- [x] Task-3: 验证并补齐证据
  - DoD: typecheck/build 通过，浏览器预览截图和控制台日志落盘，docs 完成结案字段。

## Evidence Plan (UI / E2E)
- Evidence required: yes  <!-- yes | no | partial -->
- Owner: orchestrator（single-agent-fallback）
- Artifact path: docs/specs/TID-20260407-rhythm-chip-presets/evidence/
- Interaction validation note: 当 `interaction_impact != none` 时，此处不应保持 `no`，且需覆盖 primary / fallback / visible states
- Required states to capture:
  - loading: N/A（浏览器预览直接展示已加载设置页）
  - empty: N/A（本页无空数据态）
  - error: 不主动制造错误，仅保留现有错误条能力
  - disabled: 开关关闭时区块仍保持可见，证据需体现开关与 preset 共存的可读性
  - success: 节奏页默认态截图，覆盖微休息/长休息 preset 与下方 stepper 分区

## Observability / Debug Plan
- Logs: 不新增运行时日志；主要依赖浏览器控制台和构建输出确认无渲染异常。
- Error codes: N/A
- Trace/metrics (optional): N/A
- Debug flags (optional): N/A

## Risks & Rollback
- Risks:
  - 固定 preset 可能与部分旧值不完全对齐，但本轮按用户指定值集直接收敛为受控选择。
  - 芯片组件若尺寸过大，会挤压窗口内的其他分区；需要用较紧凑布局控制换行。
- Rollback plan:
  - 回退 `App.tsx` 中新增的芯片组件和节奏页布局。
  - 重新运行 `typecheck/build` 与预览，确认回退没有引入额外问题。

## Sequential Phases
- phase_execution: N/A  <!-- sequential | N/A -->
- phase_confirmation_policy: N/A  <!-- no-intermediate-confirmation | manual | N/A -->
- phase_stop_conditions:
  - N/A

## Execution Plan
- Steps:
  1. 在 `App.tsx` 中抽出 preset 芯片行组件，并定义四组固定 options。
  2. 替换节奏页顶部微休息/长休息 stepper，为“开关 + preset 行”布局。
  3. 保持提前提醒和延后分区不变，执行 typecheck/build。
  4. 用浏览器预览采集截图与控制台日志，更新 docs / changelog / UI。

## Definition of Done (DoD)
- `plan.md` / `testplan.md` / `README.md` 不保留 `TBD/INIT` 占位进入 DONE。
- 授权 owner、升级路径、风险与回滚已写明。
- 相关验证命令与结果可在 logs/plans 中追溯。

## Approval
- Approval needed: yes  <!-- yes | no -->
- Approved: user direct request in-thread
