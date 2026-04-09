# Task-ID: TID-20260408-tauri-window-fullscreen-fix

## Summary
- Title: 修复 Tauri 窗口尺寸边界与 break 宿主显示模式
- Date: 2026-04-08
- Level: moderate
- Lane: deep
- Execution Profile: single-task
- Status: DONE

## Requirement Brief
- Goal restatement: 修正 Tauri 打包后主窗口过小、break windowed 缩成右下角小浮窗，以及 break fullscreen 顶部留白问题，让主窗口至少受 `800x600` 约束，并让 break 宿主在 windowed/fullscreen 两种模式下都符合当前预期。
- In-scope:
  - `apps/desktop/src-tauri/tauri.conf.json` 主窗口默认/最小尺寸
  - `apps/desktop/src-tauri/src/shell.rs` break windowed profile 与 fullscreen 进入/退出 helper
  - 相关 workflow/spec 文档与 changelog
- Out-of-scope:
  - break 页面 React 结构、CTA、倒计时样式
  - reminder state machine、settings schema、tray/menu 行为
- Assumptions:
  - 顶部空白主要来自 macOS native fullscreen/titlebar 语义，而不是 break 页 CSS 留白
  - 主设置窗口当前 `480x440` 的最小尺寸已低于可用下限
  - 当前 windowed microbreak 之所以仍是右下角小浮窗，是因为 `shell.rs` 单一 profile 在移除 `breakPromptStyle` 后保留了旧的 microbreak 分支
- Risks:
  - macOS fullscreen 退出过渡仍可能有系统动画
  - 若后续继续混用 `set_fullscreen()` / `set_simple_fullscreen()`，close path 可能再次分叉
  - 若后续再次把 microbreak windowed 单独分叉回角落卡片，当前居中大窗预期会回归
- Interaction impact: direct
- Primary visible flow: 用户触发 windowed break 时，窗口应居中显示，并约占当前工作区 `80% x 80%`，不再缩在右下角。
- Fallback / secondary flow: 用户触发 fullscreen break 时，窗口应无标题栏地铺满屏幕，不再出现顶部空白区域；主设置窗口缩小时仍应在 `800x600` 停止。
- User-visible boundary: 仅主设置窗口宿主尺寸与 break window 的 windowed/fullscreen 宿主行为。
- Key visible states / transitions:
  - 主窗口 resize constrained at `800x600`
  - break windowed centered at roughly `80%` of work area
  - break fullscreen visible without top gap
  - break fullscreen exits cleanly before destroy

## Goal
- 让 Tauri 主窗口边界和 break windowed/fullscreen 宿主策略对齐用户预期，并消除当前配置漂移。

## Scope
- In-scope:
  - `apps/desktop/src-tauri/tauri.conf.json`
  - `apps/desktop/src-tauri/src/shell.rs`
  - `docs/specs/TID-20260408-tauri-window-fullscreen-fix/*`
  - `docs/{plans,logs}/2026-04-08.md`
  - `docs/{UI,CHANGELOG}.md`
- Out-of-scope:
  - `apps/desktop/src/App.tsx`
  - `apps/desktop/src-tauri/src/state.rs`
  - Playwright/系统截图自动化

## Source Basis (Read Before Code)
- Related code/files reviewed:
  - `apps/desktop/src-tauri/tauri.conf.json`
  - `apps/desktop/src-tauri/src/shell.rs`
  - `apps/desktop/src/App.tsx`
- Related docs/specs/logs reviewed:
  - `docs/UI.md`
  - `docs/logs/2026-04-04.md`
  - `docs/specs/TID-20260404-fullscreen-break-close-fix/*`
- Why these are sufficient:
  - 主窗口尺寸边界只在 `tauri.conf.json`
  - break windowed 位置/尺寸与 fullscreen 进入/退出都集中在 `shell.rs`
  - `App.tsx` 已确认 break 页本身没有顶部 safe-area/padding 逻辑，问题主要在宿主层

## Acceptance Criteria (AC)
- AC1: 主设置窗口默认/最小尺寸调整到不小于 `960x640` / `800x600`。
- AC2: break 在 windowed 模式下统一居中显示，宿主窗口尺寸约占工作区 `80% x 80%`，不再保留 microbreak 右下角小浮窗逻辑。
- AC3: break fullscreen 在 macOS 走 simple fullscreen 路径，而不是继续直接依赖 native fullscreen。
- AC4: break close path 在销毁前能兜底退出 simple/native fullscreen，不依赖 `is_fullscreen()` 单一路径。
- AC5: `cargo check --manifest-path apps/desktop/src-tauri/Cargo.toml` 与 `npm --prefix apps/desktop run build` 通过。

## Interaction Freeze (only when `interaction_impact != none`)
- Freeze status: frozen before code
- Primary flow: windowed break centered and large enough for direct countdown reading
- Fallback / secondary flow: fullscreen break visible without title bar gap; main window resize stops at `800x600`
- Interaction authority / ownership boundary: 仅宿主窗口壳层；React break 内容与 settings 内容不改
- Visible entrypoints / handoff cues: windowed/fullscreen break 自动弹出；主设置窗口拖拽 resize
- In-scope interactions:
  - break windowed entry
  - fullscreen break entry
  - fullscreen break close/skip/finish teardown
  - main window minimum resize boundary
- Out-of-scope interactions:
  - break CTA 语义
  - settings 页面内容编排
  - tray / shortcut
- Interaction acceptance criteria:
  - windowed break 不再缩成右下角小浮窗
  - windowed break 居中且大致占工作区 `80% x 80%`
  - fullscreen break 顶部不再保留标题栏式空白
  - 主设置窗口不能再缩到 `800x600` 以下

## Agent Governance
- Approval Owner: orchestrator
- Execution Profile: single-task
- Orchestrator Execution Profile: aggressive baseline uses `sandbox_mode = "danger-full-access"` + `approval_policy = "never"`
- Required Roles: orchestrator,architect,coder,tester,scribe
- Execution Mode Policy: multi-agent preferred; `single-agent-fallback` only when warmup is BLOCKED and scope / reason code are explicitly bounded
- Subagent Approval Policy: non-orchestrator agents must use `approval_policy = "never"`
- Escalation Route: subagent -> Orchestrator -> direct local execution | user (only for destructive or external-impact decisions)
- Safe-local Command Route: subagent -> Orchestrator -> bare repo-local command (for example `git add -- <explicit paths...>`)
- Approval Packet Fields: command / purpose / risk / rollback

## Execution Safety Block
- service_impact: 仅桌面窗口宿主边界与 break windowed/fullscreen 进入/退出策略
- touches_running_service: no
- backup_required: no
- backup_plan: 依赖 VCS、`cargo check`、前端 build 与代码路径审查
- rollback_plan: 回退 `apps/desktop/src-tauri/{tauri.conf.json,src/shell.rs}` 与本任务 docs
- destructive_operations: 替换当前 break windowed/fullscreen 宿主策略与主窗口尺寸边界
- operator_approval_required: no
- rationale: 本轮是本地桌面壳层修复，不涉及数据迁移、提权、外部副作用或运行中服务

## Task Breakdown & Definition of Done (DoD)
- [x] Task-1: 调整 `tauri.conf.json` 主窗口默认/最小尺寸
  - DoD: 主窗口默认/最小尺寸更新为 `960x640` / `800x600`
- [x] Task-2: 调整 break 的 windowed profile
  - DoD: `shell.rs` 不再保留 microbreak 右下角小浮窗，windowed break 统一居中且约占工作区 `80% x 80%`
- [x] Task-3: 抽取 fullscreen helper 并修正 break close path
  - DoD: `shell.rs` 不再直接混用 `is_fullscreen()` + `set_fullscreen(true)`，macOS 改走 `set_simple_fullscreen()`
- [x] Task-4: 运行编译验证并同步 docs
  - DoD: `cargo check`、前端 build 和 workflow docs 记录完成

## Evidence Plan (UI / E2E)
- Evidence required: partial
- Owner: evidence_collector
- Artifact path: docs/specs/TID-20260408-tauri-window-fullscreen-fix/evidence/
- Interaction validation note: 本轮以代码路径和编译验证为主；真实 windowed/fullscreen 视觉确认仍需用户本机手动 spot-check。
- Required states to capture:
  - constrained main window shell
  - centered windowed break
  - fullscreen break visible
  - fullscreen break closed cleanly

## Observability / Debug Plan
- Logs: 无新增 runtime logger；调试入口统一收敛到 `break_window_profile()` 与 `set_break_window_fullscreen()`
- Error codes: 无新增错误码
- Trace/metrics (optional): N/A
- Debug flags (optional): N/A

## Risks & Rollback
- Risks:
  - macOS simple fullscreen 仍可能保留系统级过渡动画
  - 若后续有人重新改回 `set_fullscreen(true)`，顶部空白问题可能回归
  - 若后续再次把 microbreak windowed 分叉回角落卡片，居中大窗会回归
- Rollback plan:
  - 回退 `tauri.conf.json` 尺寸边界和 `shell.rs` 的 break profile / fullscreen helper

## Sequential Phases
- phase_execution: N/A  <!-- sequential | N/A -->
- phase_confirmation_policy: N/A  <!-- no-intermediate-confirmation | manual | N/A -->
- phase_stop_conditions:
  - N/A

## Execution Plan
- Steps:
  1. 核对主窗口尺寸边界和 break fullscreen 进入/退出路径
  2. 更新 `tauri.conf.json` 主窗口最小尺寸
  3. 在 `shell.rs` 清理旧的 microbreak 右下角小浮窗 profile，统一改为居中大窗
  4. 在 `shell.rs` 抽 fullscreen helper，并让 macOS 改走 simple fullscreen
  5. 运行 `cargo check` 和前端 build
  6. 同步 specs / logs / plans / UI / changelog

## Definition of Done (DoD)
- `plan.md` / `testplan.md` / `README.md` 不保留 `TBD/INIT` 占位进入 DONE。
- 授权 owner、升级路径、风险与回滚已写明。
- 相关验证命令与结果可在 logs/plans 中追溯。

## Approval
- Approval needed: yes
- Approved: 用户在当前线程先明确要求“改进一下”，随后又基于截图明确指出 window 模式 break 仍错误缩在右下角，要求改为居中且至少占屏幕 `80%`；无额外 operator approval 要求
