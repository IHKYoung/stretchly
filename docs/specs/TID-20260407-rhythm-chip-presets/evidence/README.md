# Task-ID: TID-20260407-rhythm-chip-presets

## Evidence Summary
- Artifact: `schedule-preset-snapshot.md`
- Artifact: `browser-console.log`
- Capture source: 2026-04-07 浏览器 preview 的可访问性快照与控制台输出。

## AC Mapping
- AC1: `schedule-preset-snapshot.md` 显示微休息/长休息的四组核心时间已改为 button group，而非顶部 `CompactNumber`。
- AC2: 快照中同一组按钮同时出现 `[pressed]` / 非选中态，证明芯片选中态与未选中态并存。
- AC3: 快照保留“提前提醒 / 延后”的 `spinbutton`，证明低频设置仍是 stepper。
- AC4: `browser-console.log` 仅包含 React DevTools 提示，无新的显著错误；配合 `typecheck/build` 作为本轮验证闭环。

## Interaction Freeze Check
- Primary flow: 用户在“节奏”页点击 preset 芯片即可完成微休息/长休息核心时间选择。
- Fallback flow: 用户继续通过 `spinbutton` 调整“提前提醒 / 延后”。
- Visible states: 快照覆盖 preset 的 selected / unselected、开关可见状态、stepper 保留态。
