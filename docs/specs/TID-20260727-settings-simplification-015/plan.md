# Task-ID: TID-20260727-settings-simplification-015

## Summary
- Title: 收口 0.1.5 设置体验改版
- Date: 2026-07-27
- Level: moderate
- Lane: deep
- Execution Profile: single-task
- Status: DONE

## Requirement Brief
- Goal restatement: 降低设置首页认知负担，修复中转式返回，提供可解释节奏方案与自定义入口，并收口为本地 `0.1.5` commit。
- In-scope: React 设置路由/profile、Rust 新安装默认值、完整休息提示语轮换、简中/繁中/英文术语、locale registry、测试、README/CHANGELOG 与版本真源。
- Out-of-scope: 已有配置迁移、smart reminder 算法、安装包、签名、公证、tag、push、GitHub Release、site 下载地址。
- Assumptions: `0.1.5` 是下一补丁版本；公开版仍为 `0.1.4`；用户已有值必须保留。
- Risks: 低频设置失去入口、返回路径绕行、旧用户被静默改值、前后端默认漂移、轮换计时起点错误、误暂存 site 工作。
- Interaction impact: direct  <!-- none | indirect | direct -->
- Primary visible flow: 设置首页直接选择节奏或进入五个主题，详情返回首页；完整休息文案打完后停留 60 秒。
- Fallback / secondary flow: 非标准节奏显示“自定义”；locale 缺失按现有 fallback；break ideas 空时使用默认 prompt。
- User-visible boundary: 桌面设置窗口、tray/runtime long-break 标签、break prompt。
- Key visible states / transitions: overview -> topic -> overview；preset/custom；typing -> 60s hold -> 520ms gap -> next prompt。

## Goal
- 形成信息架构清晰、默认值一致、兼容旧配置且可验证的 `0.1.5` 设置体验提交。

## Scope
- In-scope: `apps/desktop/src{,-tauri}` 相关代码/locale、根测试与 docs、七个版本真源。
- Out-of-scope: `apps/site/**`、`.gitignore`、`scratch/**` 现有用户改动和所有外部发布动作。

## Source Basis (Read Before Code)
- Related code/files reviewed: `App.tsx` 路由/typewriter、`settings-controls.ts`、`break-ideas.ts`、Rust settings/tests/persistence、三语言消息、locale generators、package/Cargo/Tauri manifests。
- Related docs/specs/logs reviewed: `docs/UI.md`、`SettingsInventory.md`、`CHANGELOG.md`、`RepositoryGuidelines.md`、0.1.4 发布 spec、workflow lifecycle/gates 与 handover-commit skill。
- Why these are sufficient: 覆盖用户可见入口、状态所有权、持久化兼容、调度默认、语言真源、版本出口、提交 hooks 与验证链路。

## Acceptance Criteria (AC)
- AC1: 设置首页可直接进入五个主题，任意主题返回首页，自定义节奏入口明确存在。
- AC2: 三档分别为 `30m/20s/2`、`20m/20s/3`、`10m/30s/6`，完整休息均约 60 分钟；新安装默认为均衡，旧配置不迁移。
- AC3: long break 主语言术语统一；完整休息 prompt 在完整呈现后等待 60 秒，微休息不轮换。
- AC4: 根/desktop npm、Cargo、Tauri 与 lockfile 统一为 `0.1.5`，changelog/README 对齐且 site 保持 `0.1.4` 公共下载。
- AC5: generators、122 项前端测试、29 项 Rust 测试、typecheck、frontend build 与 workflow validators 通过或缺口显式记录。

## Interaction Freeze (only when `interaction_impact != none`)
- Freeze status: locked
- Primary flow: overview 直接进入 rhythm/reminders/appearance/automation/system，返回 overview。
- Fallback / secondary flow: custom 保留旧值；缺失 prompt 走默认文案；reduced-motion 静态显示。
- Interaction authority / ownership boundary: React 持有 route/typewriter；Rust snapshot 持有设置与运行状态。
- Visible entrypoints / handoff cues: profile radiogroup、link row chevron、header back icon。
- In-scope interactions: profile 选择、主题导航、返回、prompt rotation。
- Out-of-scope interactions: tray 动作、break CTA、autosave 协议、系统权限。
- Interaction acceptance criteria: 无中转路由；所有入口可达；60 秒从完整打字后开始。

## Agent Governance
- Approval Owner: orchestrator
- Execution Profile: single-task
- Orchestrator Execution Profile: 当前产品权限提供 unrestricted filesystem 且无需审批；不额外宣称模板外权限。
- Required Roles: orchestrator,architect,coder,tester,scribe,reality_checker
- Execution Mode Policy: multi-agent preferred; `single-agent-fallback` only when warmup is BLOCKED and scope / reason code are explicitly bounded
- Subagent Approval Policy: non-orchestrator agents must use `approval_policy = "never"`
- Escalation Route: subagent -> Orchestrator -> direct local execution | user (only for destructive or external-impact decisions)
- Safe-local Command Route: subagent -> Orchestrator -> bare repo-local command (for example `git add -- <explicit paths...>`)
- Approval Packet Fields: command / purpose / risk / rollback

## Execution Safety Block
- service_impact: none（仅本地源码与 commit）。
- touches_running_service: no。
- backup_required: no。
- backup_plan: 提交前保留完整 diff/status 与验证记录。
- rollback_plan: 后续普通 `git revert`；schema 未变，无数据迁移回滚。
- destructive_operations: none。
- operator_approval_required: no（用户已明确要求版本更新与 commit；外部发布另需授权）。
- rationale: 本地版本 bump 与非破坏性 commit 在用户授权范围内。

## Task Breakdown & Definition of Done (DoD)
- [x] Task-1: 设置导航与 profile 重构
  - DoD: 首页直达五主题、返回首页、自定义入口与 profile tests 完成。
- [x] Task-2: 节奏、术语与轮换收口
  - DoD: 默认值/兼容策略、60 秒 hold、三语言与 Rust/前端测试完成。
- [x] Task-3: 0.1.5 版本与交接
  - DoD: 版本真源、README、CHANGELOG、workflow docs、精确 staging 与 commit 完成。

## Evidence Plan (UI / E2E)
- Evidence required: partial  <!-- yes | no | partial -->
- Owner: orchestrator
- Artifact path: docs/specs/TID-20260727-settings-simplification-015/evidence/
- Interaction validation note: 导航/计时/语言由自动化测试覆盖；本地服务 HTTP 200；浏览器后端不可用，未采集截图或 60 秒实时时序视频。
- Required states to capture:
  - loading: snapshot loading 文案保持既有行为。
  - empty: custom profile 与 break-idea fallback 有源码/测试覆盖。
  - error: autosave/load error 保持显式错误条，未改协议。
  - disabled: 既有开关与 runtime action disabled 行为不变。
  - success: 五主题直达、profile 值、术语与计时顺序测试 PASS。

## Observability / Debug Plan
- Logs: Vitest/Rust/build/generator/workflow 输出与 commit audit。
- Error codes: 保留命令退出码；浏览器 discovery 空列表单独记录。
- Trace/metrics (optional): 122 frontend tests、29 Rust tests、50 locale bundles、版本 0.1.5。
- Debug flags (optional): 浏览器 preview 与 `?window=break`。

## Risks & Rollback
- Risks: existing custom settings 显示变化、缺少自动截图、Vite 大 chunk 既有 warning。
- Rollback plan: revert 本提交；不触碰已有 settings 文件或远端状态。

## Sequential Phases
- phase_execution: N/A  <!-- sequential | N/A -->
- phase_confirmation_policy: N/A  <!-- no-intermediate-confirmation | manual | N/A -->
- phase_stop_conditions:
  - N/A

## Execution Plan
- Steps:
  1. 重构 overview 与直接导航，补 profile/domain helpers。
  2. 校准默认节奏与提示语时序，统一语言并生成 registry。
  3. 更新测试/docs/version，运行门禁并精确提交。

## Definition of Done (DoD)
- `plan.md` / `testplan.md` / `README.md` 不保留 `TBD/INIT` 占位进入 DONE。
- 授权 owner、升级路径、风险与回滚已写明。
- 相关验证命令与结果可在 logs/plans 中追溯。

## Approval
- Approval needed: yes  <!-- yes | no -->
- Approved: 用户于 2026-07-27 明确要求更新版本、补充 changelog 并 commit；不包含外部发布。
