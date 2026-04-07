# Task-ID: TID-20260403-break-surface-mode-restore Evidence

## Summary
- 证据目标：证明设置页已重新暴露 break surface mode，并且该设置已重新接回 Tauri host。
- 采集时间：2026-04-03
- 采集环境：浏览器 preview，窗口 `1440x810`

## Artifacts
- DOM snapshot: `./settings-break-surface-snapshot-1440x810.md`
- Console log: `./browser-console.log`

## AC Coverage
- AC2：snapshot 中可见 `提醒与打断` active、`休息窗口` 分组、`显示方式` 与对应 tablist。
- AC5：console log 无新增错误，配合 `cargo check`、`typecheck`、`build` 通过，证明入口恢复且前后端编译链闭环。

## Gaps
- Playwright screenshot 在当前页面上持续卡在 fonts wait，未产出稳定图片文件；因此本轮以 DOM snapshot 作为 UI 旁证。
