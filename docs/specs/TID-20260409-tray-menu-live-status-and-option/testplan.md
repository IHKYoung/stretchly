# Task-ID: TID-20260409-tray-menu-live-status-and-option

## Test Strategy
- Unit:
  - Rust 纯函数测试覆盖 title 开关行为
- Integration:
  - locale sync
  - TypeScript typecheck
  - Rust tests
  - desktop build
- E2E (if applicable):
  - N/A

## Acceptance Criteria Coverage Map (AC → Tests)
- AC1 -> 编译通过 + `shell.rs` menu item updater 代码审查
- AC2 -> `shell::tests::tray_title_respects_setting_toggle`
- AC3 -> `PauzaSettings::default()` + `#[serde(default)]` 代码审查
- AC4 -> `python3 scripts/sync_desktop_locales.py`、`npm --prefix apps/desktop run typecheck`、`cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`、`npm --prefix apps/desktop run build`、`python3 scripts/validate_workflow_docs.py --mode manual`

## Interaction Contract Coverage
- Interaction impact: none
- Primary flow -> tests/evidence: N/A
- Fallback / secondary flow -> tests/evidence: N/A
- Visible states / transitions -> tests/evidence: N/A
- Validator expectation: 当 `interaction_impact != none` 时，本节三项与 Evidence Capture 的 `Required` 不得继续保留 `N/A/no/TBD`

## Governance Gates
- Agent Config Validation: `python3 scripts/validate_agent_configs.py`
- Workflow Docs Validation: `python3 scripts/validate_workflow_docs.py --mode manual`
- Approval Escalation Owner: orchestrator

## False-pass Cases
- title 开关可用，但菜单状态仍冻结
- 菜单实时更新通过重建整个菜单实现，导致原生菜单闪烁
- 英文 locale 未补齐，导致设置页回退到中文或 key
- 新增字段没有默认值，老配置读出来变成 `false`

## Evidence Capture (UI / E2E)
- Required: no
- Owner: evidence_collector
- Artifacts path: docs/specs/TID-20260409-tray-menu-live-status-and-option/evidence/
- What to capture:
  - Screenshots: N/A
  - Video/trace (optional): N/A
  - HAR/console logs (optional): N/A

## Quality Gates (Non-functional)
- a11y: 新开关需保留 aria label
- perf budget: 不允许退化为每秒重建菜单
- error handling / observability: 沿用现有设置保存错误边界
- security / privacy: 不新增权限

## Boundary / Invalid Input Cases
- 旧 settings 文件缺失字段
- 开关关闭时 title 应为空
- 菜单项 handle 更新应在菜单重建后自动替换

## Concurrency / Race Cases (if applicable)
- engine 每秒 tick 与菜单重建之间不能让 updater 指向旧句柄太久

## Mocks & Test Data
- 直接使用 `PauzaSettings::default()` 与 shell 纯函数测试

## Commands to Run
- `python3 scripts/sync_desktop_locales.py`
- `npm --prefix apps/desktop run typecheck`
- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`
- `npm --prefix apps/desktop run build`
- `python3 scripts/validate_workflow_docs.py --mode manual`

## Expected Results
- PASS criteria:
  - locale registry 重生成成功
  - typecheck / Rust tests / build / docs validator 全部通过
- Outputs to keep (10~20 lines snippet):
  - `[OK] Built desktop locale registry: 50 languages`
  - `test shell::tests::tray_title_respects_setting_toggle ... ok`
  - `test result: ok. 16 passed`
