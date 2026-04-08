# Task-ID: TID-20260408-macos-icon-fidelity

## Summary
- Title: 修复 macOS 顶部与 Dock 图标清晰度和尺寸
- Date: 2026-04-08
- Level: moderate
- Lane: deep
- Execution Profile: single-task
- Status: DONE

## Requirement Brief
- Goal restatement: 修复 macOS 菜单栏 tray icon 发糊且偏大、Dock icon 偏大的问题，并让图标资源链路以后可以稳定重生成。
- In-scope: Tauri macOS 宿主图标接入、tray 小尺寸 SVG 几何、图标生成脚本、再生的 Tauri icon 产物与本任务文档。
- Out-of-scope: 产品业务逻辑、tray 菜单内容、break 调度、Electron legacy 的功能语义和非 macOS 平台视觉策略。
- Assumptions: 当前问题主要发生在 macOS Retina 环境；用户更在意原生尺寸/清晰度而不是进一步重做品牌图形。
- Risks: 若用户本机实际菜单栏观感仍偏大，可能还需要继续微调 tray SVG 的几何比例；当前终端未授予 Screen Recording，系统级截图证据无法在本轮获取。
- Interaction impact: none  <!-- none | indirect | direct -->
- Primary visible flow: N/A
- Fallback / secondary flow: N/A
- User-visible boundary: N/A
- Key visible states / transitions: N/A

## Goal
- 让 Pauza 的 macOS tray icon 使用正确的原生入口与 Retina 表示，不再依赖全局 status item hack。
- 让 Dock icon 使用更符合 macOS 资源习惯的 multi-representation app icon 资源。
- 修复图标生成脚本中的真实故障，确保图标链路可重复执行。

## Scope
- In-scope:
  - `apps/desktop/src-tauri/src/shell.rs`
  - `graphics/tray-icon.svg`
  - `graphics/generate_icon_assets.py`
  - `apps/desktop/src-tauri/icons/{icon.png,icon.icns,tray-iconTemplate.png,tray-iconTemplate@2x.png}`
  - `docs/specs/TID-20260408-macos-icon-fidelity/*`
  - `docs/plans/2026-04-08.md`
  - `docs/logs/2026-04-08.md`
  - `docs/CHANGELOG.md`
- Out-of-scope:
  - `graphics/app-icon.svg` 的品牌造型大改
  - 非 macOS 平台的托盘 / Dock / 任务栏视觉策略
  - 任何业务功能、设置项或调度状态机语义

## Source Basis (Read Before Code)
- Related code/files reviewed:
  - `apps/desktop/src-tauri/src/shell.rs`
  - `graphics/generate_icon_assets.py`
  - `graphics/tray-icon.svg`
  - `graphics/app-icon.svg`
  - `~/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tray-icon-0.21.3/src/platform_impl/macos/mod.rs`
- Related docs/specs/logs reviewed:
  - `docs/specs/TID-20260407-app-icon-refresh/*`
  - `docs/RepositoryGuidelines.md`
  - `docs/CodeMap.md`
  - Apple High Resolution Guidelines 中关于 `@2x` / multi-representation images 的说明
- Why these are sufficient:
  - 已覆盖当前图标生成链、Tauri macOS tray 接入实现、前一轮未收口的 icon 任务和 Apple 的高分辨率图像约束，足以判断“糊 + 偏大”分别来自宿主层和资源层。

## Acceptance Criteria (AC)
- AC1: macOS tray icon 不再通过枚举全部 status items 的方式替换图像，而是只 patch Pauza 自己的 `NSStatusItem`，并使用 1x/2x 表示的 template image。
- AC2: Dock icon 不再用原始 `icon.png` 覆盖应用图标，而是改用 bundled `icon.icns` multi-representation 资源。
- AC2a: release `.app` 不再执行 Dock runtime patch，优先使用系统 bundle icon。
- AC3: `graphics/generate_icon_assets.py` 能完整跑通并成功生成 tray / package icon 产物。
- AC4: Rust host 和前端构建均通过，不引入新的编译或测试失败。

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
- Required Roles: orchestrator,ui_designer,architect,coder,tester,scribe
- Execution Mode Policy: multi-agent preferred; `single-agent-fallback` only when warmup is BLOCKED and scope / reason code are explicitly bounded
- Subagent Approval Policy: non-orchestrator agents must use `approval_policy = "never"`
- Escalation Route: subagent -> Orchestrator -> direct local execution | user (only for destructive or external-impact decisions)
- Safe-local Command Route: subagent -> Orchestrator -> bare repo-local command (for example `git add -- <explicit paths...>`)
- Approval Packet Fields: command / purpose / risk / rollback

## Execution Safety Block
- service_impact: 仅限 macOS 图标显示与本地资源生成链
- touches_running_service: no
- backup_required: no
- backup_plan: 以 Git diff、重新生成资源和构建链结果为边界
- rollback_plan: 回退 `shell.rs`、`graphics/*` 与再生的 icon 产物
- destructive_operations: 替换 tray / dock 图标接入方式并刷新 icon 资源
- operator_approval_required: no
- rationale: 不涉及数据、权限、外部副作用或提权

## Task Breakdown & Definition of Done (DoD)
- [x] Task-1: 定位 macOS tray / dock 图标异常的宿主层根因
  - DoD: 明确 `tray-icon` 的 `18pt` 固定逻辑高度、现有全局 status item hack 的问题，以及 Dock 覆盖逻辑对原始 `icon.png` 的依赖
- [x] Task-2: 重写 macOS icon patch 并重画 tray 小尺寸几何
  - DoD: `shell.rs` 使用定向 `NSStatusItem` patch；tray SVG 采用更保守的小尺寸几何；重新生成 icon 产物
- [x] Task-3: 收紧 Dock icon 视觉体积与 bundle 行为
  - DoD: `app-icon.svg` 加入安全边距；release `.app` 不再执行 Dock runtime patch
- [x] Task-4: 修复图标生成脚本并完成验证/文档
  - DoD: `generate_icon_assets.py` 能跑通；构建/测试通过；spec / plans / logs / changelog 收口

## Evidence Plan (UI / E2E)
- Evidence required: no  <!-- yes | no | partial -->
- Owner: evidence_collector
- Artifact path: docs/specs/TID-20260408-macos-icon-fidelity/evidence/
- Interaction validation note: 当 `interaction_impact != none` 时，此处不应保持 `no`，且需覆盖 primary / fallback / visible states
- Required states to capture:
  - loading: N/A
  - empty: N/A
  - error: N/A
  - disabled: N/A
  - success: tray icon 资源预览与再生产物存在；系统级截图因 Screen Recording 未授权而未采集

## Observability / Debug Plan
- Logs: 通过 `cargo check` / `cargo test` / `npm --prefix apps/desktop run build` / `python3 graphics/generate_icon_assets.py` 的命令输出追踪资源与宿主层变更
- Error codes: 无新增产品级错误码；重点观察图标生成脚本与 Rust 编译错误
- Trace/metrics (optional): N/A
- Debug flags (optional): N/A

## Risks & Rollback
- Risks:
  - 无系统级截图时，最终“观感”只能依赖资源预览与实现逻辑推断
  - tray glyph 如果用户本机仍觉得偏大，仍需继续调 `graphics/tray-icon.svg`
- Rollback plan:
  - 回退 `apps/desktop/src-tauri/src/shell.rs`
  - 回退 `graphics/tray-icon.svg` 与 `graphics/generate_icon_assets.py`
  - 重新生成并恢复原始 `apps/desktop/src-tauri/icons/*`

## Sequential Phases
- phase_execution: N/A  <!-- sequential | N/A -->
- phase_confirmation_policy: N/A  <!-- no-intermediate-confirmation | manual | N/A -->
- phase_stop_conditions:
  - N/A

## Execution Plan
- Steps:
  1. 阅读 Tauri / tray-icon / graphics 现状，区分资源层与宿主层根因。
  2. 改写 `shell.rs` 的 macOS tray / dock icon patch。
  3. 调整 `graphics/tray-icon.svg` 并修复 `graphics/generate_icon_assets.py`。
  4. 重新生成 icon 资源并跑构建 / 测试。
  5. 落盘 docs / changelog 并跑 workflow gate。

## Definition of Done (DoD)
- `plan.md` / `testplan.md` / `README.md` 不保留 `TBD/INIT` 占位进入 DONE。
- 授权 owner、升级路径、风险与回滚已写明。
- 相关验证命令与结果可在 logs/plans 中追溯。

## Approval
- Approval needed: yes  <!-- yes | no -->
- Approved: 已在对话中直接批准修复 macOS 顶部与 Dock 图标问题
