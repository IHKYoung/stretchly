# Task-ID: TID-20260407-microbreak-break-labels

## Summary
- Title: 统一微休息与休息命名
- Date: 2026-04-07
- Level: moderate
- Lane: deep
- Execution Profile: single-task
- Status: DONE

## Requirement Brief
- Goal restatement: 将当前产品里与两类休息相关的用户可见命名统一为“微休息 / 休息”，英文对应 `Microbreak / Break`，同时保留内部历史标识以避免兼容性回归。
- In-scope: `apps/desktop` 与 legacy app 的现行 locale 文案、README / `docs/CHANGELOG.md` / `docs/UI.md` / `docs/SettingsInventory.md` / `docs/CodeMap.md`、必要的 `net.hovancik.Pauza.metainfo.xml` 描述、浏览器 preview 证据，以及本任务 docs/plans/logs。
- Out-of-scope: 重命名 `longBreak*` / `miniBreak*` 内部字段、Tauri / Electron 命令名、设置 schema、持久化 key、历史 spec/logs/evidence 归档。
- Assumptions: 当前共享 locale 已覆盖主设置页、tray、runtime break surface 的用户可见文案；保持内部兼容命名比彻底 rename 更重要；历史审计文档不应因一次术语调整被批量重写。
- Risks: 若仍有零散 hard-coded 文案未走 locale，会出现新旧术语混用；若文档没有明确“仅 UI 术语变化”，可能让人误以为设置 schema 也已改名。
- Interaction impact: direct  <!-- none | indirect | direct -->
- Primary visible flow: 用户打开 `apps/desktop` 主设置页，在“节奏”“提前提醒”“延后”等区域直接看到“微休息 / 休息”而非旧叫法。
- Fallback / secondary flow: tray 的“跳到”子菜单和 break runtime 标题通过同一套 locale 使用 `Microbreak / Break`、`微休息 / 休息`，不改变原有交互路径。
- User-visible boundary: 当前活跃的设置页标签、strict / shortcut / tray / runtime 文案、legacy app 可见 copy、README 与软件中心截图 caption。
- Key visible states / transitions: 设置页默认态下两类休息标题与子分区标签；tray skip 目标 label；runtime break title / kind 文案；文档与元数据说明中对兼容命名的明确说明。

## Goal
- 让当前产品里的命名体系更符合用户直觉，同时不为了术语统一去破坏兼容性边界。

## Scope
- In-scope:
  - 更新 `apps/desktop/src/locales/{zh-CN,en}.json` 中与两类休息相关的现行文案。
  - 更新 `app/locales/{zh-CN,en}.json` 中 legacy app 的现行文案。
  - 更新 `README.md`、`docs/CHANGELOG.md`、`docs/UI.md`、`docs/SettingsInventory.md`、`docs/CodeMap.md` 与 `net.hovancik.Pauza.metainfo.xml` 的当前说明。
  - 采集一份浏览器 preview 快照和控制台摘要，证明主设置页术语已落地。
  - 补齐本任务 spec、当日 plan/log。
- Out-of-scope:
  - 修改内部类型、字段、命令、迁移或测试断言里的历史命名。
  - 为旧版 spec、旧 evidence、历史日志做全文替换。
  - 重新设计设置页布局、托盘结构或 break prompt 交互。

## Source Basis (Read Before Code)
- Related code/files reviewed:
  - `apps/desktop/src/locales/zh-CN.json`
  - `apps/desktop/src/locales/en.json`
  - `app/locales/zh-CN.json`
  - `app/locales/en.json`
  - `apps/desktop/src/App.tsx`
- Related docs/specs/logs reviewed:
  - `README.md`
  - `docs/CHANGELOG.md`
  - `docs/UI.md`
  - `docs/SettingsInventory.md`
  - `docs/CodeMap.md`
  - `net.hovancik.Pauza.metainfo.xml`
  - `docs/plans/2026-04-07.md`
  - `docs/logs/2026-04-07.md`
- Why these are sufficient: 本轮不改调度、宿主命令或存储契约，重点是当前共享 locale 和现行文档；上述文件已覆盖主设置页、tray/runtime 文案来源、legacy app copy、当前说明文档与当天执行上下文。

## Acceptance Criteria (AC)
- AC1: `apps/desktop` 当前可见标签统一使用“微休息 / 休息”和 `Microbreak / Break`，覆盖节奏页、strict/shortcut 标签、tray skip 文案和 runtime break title/kind。
- AC2: legacy app 的当前可见 locale 文案同步使用“微休息 / 休息”和 `Microbreak / Break`，不再保留 `小憩` 或 `Long break / Mini break` 的用户可见叫法。
- AC3: 当前 README / 现行文档 / 元数据与产品术语对齐，并明确内部仍保留 `miniBreak*` / `longBreak*` 等兼容标识。
- AC4: 浏览器 preview 快照能直接看到主设置页的“微休息 / 休息”标签；`npm test`、`npm --prefix apps/desktop run typecheck`、`npm --prefix apps/desktop run build`、agent config gate 与 workflow docs gate 通过。

## Interaction Freeze (only when `interaction_impact != none`)
- Freeze status: frozen for implementation
- Primary flow: 用户进入主设置页后，直接在“节奏”“提前提醒”“延后”里看到“微休息 / 休息”的成对命名。
- Fallback / secondary flow: tray 的“跳到”子菜单和 break runtime 标题也复用相同术语，但本轮不额外改动其交互顺序或控制逻辑。
- Interaction authority / ownership boundary: 只调整当前用户可见术语；不改按钮位置、控件结构、保存逻辑、skip/postpone 行为或 tray 层级。
- Visible entrypoints / handoff cues: 设置页分类导航、微休息/休息分区标题、tray skip label、runtime break title、README 里的功能描述与软件中心截图 caption。
- In-scope interactions: 打开设置页阅读标签、通过 tray 识别“跳到微休息 / 跳到休息”、识别 break prompt 的类型文案。
- Out-of-scope interactions: 任何 schema rename、migration、快捷键逻辑变化、调度行为变化、历史证据回写。
- Interaction acceptance criteria: 现行用户看到的两类休息在中英文本中都应是一对稳定且对称的术语，“微休息 / 休息”与 `Microbreak / Break` 不再与旧叫法混用。

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
- service_impact: 仅限当前 UI / 文档 / 元数据的用户可见术语，不改变实际调度和设置行为。
- touches_running_service: no
- backup_required: no
- backup_plan: 以当前 Git diff、浏览器 preview 快照、构建输出和任务 docs 为回滚边界。
- rollback_plan: 回退 locale、README/docs、metainfo 与本任务 evidence / logs / plans。
- destructive_operations: 替换当前可见文案，但不删除 schema 或兼容字段。
- operator_approval_required: no
- rationale: 本轮不涉及运行中服务、权限、外部请求、付费成本、数据删改或提权。

## Task Breakdown & Definition of Done (DoD)
- [x] Task-1: 替换当前 locale / 文档 / 元数据里的现行术语
  - DoD: `apps/desktop` locale、legacy app locale、README 和活跃文档都使用“微休息 / 休息”与 `Microbreak / Break`。
- [x] Task-2: 采集最小 UI 证据
  - DoD: 浏览器 preview 快照与控制台摘要落盘到本任务 `evidence/`。
- [x] Task-3: 执行验证并结案
  - DoD: 测试、typecheck/build、agent config gate、workflow docs gate 通过，daily plan/log 与 spec 完整无占位。

## Evidence Plan (UI / E2E)
- Evidence required: yes  <!-- yes | no | partial -->
- Owner: orchestrator（single-agent-fallback）
- Artifact path: docs/specs/TID-20260407-microbreak-break-labels/evidence/
- Interaction validation note: 当 `interaction_impact != none` 时，此处不应保持 `no`，且需覆盖 primary / fallback / visible states
- Required states to capture:
  - loading: N/A（本地 preview 直接展示已加载设置页）
  - empty: N/A（本页无空数据态）
  - error: N/A（本轮不主动制造错误态）
  - disabled: 设置页中微休息 / 休息开关可见，证明分区标签与开关组合成立
  - success: 设置页默认态快照，能同时看到“微休息”“休息”“提前提醒”“延后”里的新术语

## Observability / Debug Plan
- Logs: 不新增运行时日志；主要依赖浏览器 preview 控制台、`npm test` / `typecheck` / `build` 输出和 locale diff 排查问题。
- Error codes: N/A
- Trace/metrics (optional): N/A
- Debug flags (optional): N/A

## Risks & Rollback
- Risks:
  - 零散 hard-coded 文案若未走 locale，可能留下新旧术语混用。
  - 文档若没有写清兼容边界，后续维护者可能误以为内部 key 也应同步改名。
- Rollback plan:
  - 回退本轮 locale / docs / metadata 修改。
  - 重新运行测试、构建与 preview，确认回到旧术语但不引入额外问题。

## Sequential Phases
- phase_execution: N/A  <!-- sequential | N/A -->
- phase_confirmation_policy: N/A  <!-- no-intermediate-confirmation | manual | N/A -->
- phase_stop_conditions:
  - N/A

## Execution Plan
- Steps:
  1. 扫描当前活跃 locale、README 和现行文档中的旧术语命中。
  2. 将可见文案统一替换为“微休息 / 休息”和 `Microbreak / Break`，并保留兼容性的内部旧标识。
  3. 复用现成 `43179` 预览服务采集设置页快照和控制台摘要。
  4. 跑测试、构建、workflow gates，并更新本任务 docs / daily plan / daily log。

## Definition of Done (DoD)
- `plan.md` / `testplan.md` / `README.md` 不保留 `TBD/INIT` 占位进入 DONE。
- 授权 owner、升级路径、风险与回滚已写明。
- 相关验证命令与结果可在 logs/plans 中追溯。

## Approval
- Approval needed: yes  <!-- yes | no -->
- Approved: user direct request in-thread
