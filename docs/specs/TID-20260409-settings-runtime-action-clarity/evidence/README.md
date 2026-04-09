# Task-ID: TID-20260409-settings-runtime-action-clarity

## Evidence Summary
- 目标：证明设置页已把“恢复提醒”和“重置节奏”拆成不同层级，且 `恢复提醒` 只在暂停态出现。
- 环境：浏览器 preview（`http://127.0.0.1:43179`），通过 `App.tsx` 的 fallback mock 路径验证默认态 / paused / focus 三种状态。
- 结论：PASS。默认态只看到独立的 `节奏控制 -> 重置节奏`；暂停态顶部出现 `恢复提醒`；focus 态顶部出现 `结束专注`。

## Artifacts
- `settings-default.png`：默认运行态，顶部无恢复动作，只保留独立的 `节奏控制 / 重置节奏`。
- `settings-paused.png`：暂停态，顶部 `当前状态` 卡出现 `恢复提醒`，同时 `重置节奏` 仍独立存在。
- `settings-focus.png`：focus 态，顶部动作变为 `结束专注`，不再复用“恢复提醒”命名。

## Command Trace
- `python3 scripts/sync_desktop_locales.py`
  - `[OK] Built desktop locale registry: 54 languages, registry -> apps/desktop/src/locales/registry.generated.json`
- `npm --prefix apps/desktop run typecheck`
  - `tsc --noEmit` PASS
- `npm --prefix apps/desktop run build`
  - `vite v7.3.1 building client environment for production...`
  - `✓ 1824 modules transformed.`
  - `✓ built in 1.57s`
- `python3 scripts/validate_workflow_docs.py --mode manual`
  - `[OK] Workflow docs validation passed for 2026-04-09 ✅`
- `python3 scripts/validate_agent_configs.py`
  - `[SKIP] Agent config validation skipped: no multi-agent config detected ✅`
- Playwright CLI
  - `open http://127.0.0.1:43179`
  - `screenshot --full-page --filename .../settings-default.png`
  - `goto "http://127.0.0.1:43179/?preview=paused"`
  - `screenshot --full-page --filename .../settings-paused.png`
  - `goto "http://127.0.0.1:43179/?preview=focus"`
  - `screenshot --full-page --filename .../settings-focus.png`
  - `console info` -> `Total messages: 3 (Errors: 0, Warnings: 0)`

## Interaction Coverage
- Primary flow -> evidence
  - `settings-paused.png` 证明暂停态顶部只出现 `恢复提醒`。
- Fallback / secondary flow -> evidence
  - `settings-focus.png` 证明 focus 态顶部改为 `结束专注`，默认态不再复用 `恢复提醒`。
- Visible states / transitions -> evidence
  - `settings-default.png` 与 `settings-paused.png` 共同证明 `重置节奏` 已独立收进 `节奏控制` 区，并附带“不自动解除暂停”的说明。

## Notes
- 浏览器 preview 的 `?preview=paused|focus` 只用于证据采集，不会改动 Tauri runtime 或持久化设置。
