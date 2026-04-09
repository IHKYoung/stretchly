# Task-ID: TID-20260408-reminder-mode-simplification

## Goals
- 把设置页用户心智收敛成 `智能提醒 / 强制提醒 / 自然休息`。
- 去掉重复的 strict mode 开关和 break style 入口。

## Non-Goals
- 不重做整体视觉风格。
- 不新增第三个“中间提醒模式”。

## Screens & User Flows
- Primary flow:
  - 用户在 `偏好` 分类中选择 `智能提醒` 或 `强制提醒`
  - 同一区域继续管理 `自然休息`
- Fallback / secondary flow:
  - break window 仍通过 `canPostpone / canSkip` 自动决定按钮可见性
- User-visible boundary:
  - 设置页
  - break window 按钮能力
  - tray / status 文案
- Entrypoints / handoff cues:
  - `App.tsx` 偏好页 segmented control

## Component Tree
- `SettingsCard`
  - `SettingsRow(reminder mode)`
    - `SegmentedControl(smart | forced)`

## Interaction States
- disabled:
  - `forced` 下 break window 的 `Later` / `Skip` 不再可见
- success:
  - 选中 `smart` 后到点先等待空档
  - 选中 `forced` 后到点立即严格进入 break

## Responsive Rules
- 继续沿用当前窄偏好窗口布局，不新增新列。

## Accessibility (a11y)
- keyboard navigation:
  - reminder mode 通过 segmented control 可键盘切换
- aria labels:
  - segmented control 使用 locale 文案作为 `ariaLabel`

## Design Tokens / Tailwind Mapping
- key classes:
  - 复用现有 segmented control，不引入新视觉组件

## Edge Cases
- long text:
  - `reminderModeHint` 需在中文和英文下都能在单行/双行内自然换行
- permission denied:
  - N/A

## Acceptance Criteria (UI)
- 设置页不再显示 `Strict modes` 区块。
- 设置页不再显示 `Break style` 区块。
- 偏好页新增 `Reminder mode` 区块。

## Open Questions
- N/A
