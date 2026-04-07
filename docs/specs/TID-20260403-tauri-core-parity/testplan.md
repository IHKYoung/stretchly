# Task-ID: TID-20260403-tauri-core-parity

## Test Strategy
- Unit:
  - `apps/desktop/src-tauri/src/state.rs` 覆盖 pre-break notification、manual finish、postpone limit 三条关键状态机测试。
- Integration:
  - `cargo check --manifest-path apps/desktop/src-tauri/Cargo.toml`
  - `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`
  - `npm --prefix apps/desktop run typecheck`
  - `npm --prefix apps/desktop run build`
- E2E (if applicable):
  - `npm run desktop:dev` 人工启动到 Tauri runtime，验证默认根入口与 break window 运行路径。
  - 本任务尚未补单独 Playwright/截图工件，证据以相邻任务结构快照和本任务 evidence README 旁证为主。

## Acceptance Criteria Coverage Map (AC → Tests)
- AC1 -> `cargo check` + `cargo test`；检查 `state.rs` 状态机、`commands.rs` 命令边界、`shell.rs` tray/shortcut/break window 行为已连通。
- AC2 -> `npm --prefix apps/desktop run typecheck` + `npm --prefix apps/desktop run build`；人工 `desktop:dev` 验证主设置页与 break prompt 能启动。
- AC3 -> 读取根 `package.json` 脚本并执行 `npm run desktop:dev` / `npm run desktop:build` 作为默认链路证明。
- AC4 -> `python3 scripts/validate_workflow_docs.py --mode manual` + 本任务 spec/logs/plans/changelog/codemap/architecture/ui 落盘。

## Interaction Contract Coverage
- Interaction impact: direct
- Primary flow -> tests/evidence:
  - 构建与类型检查证明主设置页可编译。
  - 人工 `desktop:dev` 证明根入口走 Tauri。
  - 仍缺主设置页完成态截图。
- Fallback / secondary flow -> tests/evidence:
  - tray/shortcut 逻辑由 `shell.rs` + `commands.rs` 审阅与 `cargo check` 覆盖编译连通。
  - 仍缺 tray submenu 与快捷键生效的独立录屏/截图。
- Visible states / transitions -> tests/evidence:
  - `state.rs` 单测覆盖 notification、manual-awaiting、postpone limit。
  - 仍缺 strict break / manual-awaiting 的可视化证据。

## Governance Gates
- Agent Config Validation: `python3 scripts/validate_agent_configs.py`
- Workflow Docs Validation: `python3 scripts/validate_workflow_docs.py --mode manual`
- Approval Escalation Owner: orchestrator

## False-pass Cases
- 根入口虽然切到了 Tauri，但 `legacy:*` 脚本或 Electron 源码仍被误写成“已删除”。
- 前台虽然可编译，但仍是展示页而不是极简设置页。
- `state.rs` 新增设置字段未同步到 `DesktopSnapshot` / 命令层 / 前台表单。
- tray 菜单编译通过，但 strict mode / autostart / focus / pause 子菜单没有真实交互证据。
- 文档宣称 evidence 完整，但实际上缺少本任务目录下的证据报告。

## Evidence Capture (UI / E2E)
- Required: partial
- Owner: evidence_collector
- Artifacts path: docs/specs/TID-20260403-tauri-core-parity/evidence/
- What to capture:
  - Screenshots:
    - 主设置页中文默认态
    - Advanced 展开态
    - break prompt 正常倒计时态
    - break prompt `manualAwaiting` 态
    - tray 菜单的 skip/focus/pause 子菜单
  - Video/trace (optional):
    - pause -> resume
    - focus -> break 到期
  - HAR/console logs (optional):
    - `desktop:dev` 启动日志
    - 前台控制台日志

## Quality Gates (Non-functional)
- a11y:
  - 主设置页必须支持键盘切换 checkbox/select/input。
  - break prompt 主 CTA 在普通态和 `manualAwaiting` 态都应保持焦点清晰。
- perf budget:
  - 主窗口不依赖 Electron/Chromium，默认入口应基于 Tauri runtime 启动。
- error handling / observability:
  - `App.tsx` 必须能展示 `error-banner`。
  - Rust host 错误通过字符串返回到前台。
- security / privacy:
  - app exclusion 仅保存本地命令片段字符串，不上传远端。

## Boundary / Invalid Input Cases
- shortcut 输入为空时应回退到默认值或空字符串，不应写入异常格式。
- notification/postpone/duration 数值越界时应被 `sanitized()` clamp。
- `targetScreen` 非法值应回退到 `primary`。
- pause minutes 超过命令层上限时应被 clamp 到最大允许值。

## Concurrency / Race Cases (if applicable)
- engine tick 与前台轮询 snapshot 并发发生时，`RuntimeState` 通过 `Mutex` 保证串行访问。
- break window 关闭、tray action、shortcut action 都可能同时触发，需要以 `PauzaState` 的当前 break 状态为真源。

## Mocks & Test Data
- 使用 `PauzaSettings::default()` 作为默认测试数据。
- 浏览器 preview 使用 `previewSnapshot()` 作为无 Tauri runtime 时的 UI 兜底数据。

## Commands to Run
- `cargo check --manifest-path apps/desktop/src-tauri/Cargo.toml`
- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`
- `npm --prefix apps/desktop run typecheck`
- `npm --prefix apps/desktop run build`
- `python3 scripts/validate_workflow_docs.py --mode manual`

## Expected Results
- PASS criteria:
  - Rust host 编译通过且单测全绿。
  - 前台类型检查与生产构建通过。
  - 文档门禁通过。
  - evidence 目录存在并明确记录已采/未采的交互证明。
- Outputs to keep (10~20 lines snippet):
  - `cargo test` 的 `3 passed; 0 failed`
  - `vite build` 产物摘要
  - workflow docs validation 结果
