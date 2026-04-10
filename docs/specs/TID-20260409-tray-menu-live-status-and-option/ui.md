# Task-ID: TID-20260409-tray-menu-live-status-and-option

## Goals
- 让用户看到“打开 tray 菜单后时间仍然在走”。
- 给用户一个明确的顶部倒计时显示开关。

## Non-Goals
- 不新增单独 tray 设置页。
- 不改变 tray 菜单的动作分组。

## Screens & User Flows
- Primary flow:
  - 用户右键 tray 图标查看状态，`status-detail` 中的剩余时间持续变化。
  - 用户在设置页 General 区切换“Show time to next break in tray”，顶部数字立即显示或隐藏。
- Fallback / secondary flow:
  - 阻塞态下菜单仍显示实时状态描述，但顶部 title 可为空。
- User-visible boundary:
  - 顶部 tray title
  - 右键 tray 菜单的两条信息项
  - 设置页 General 区新开关
- Entrypoints / handoff cues:
  - 系统 menubar / tray icon
  - 设置页 General section

## Component Tree
- SettingsCard / SettingsRow / Switch
- tray title
- tray menu `status`
- tray menu `status-detail`

## Interaction States
- hover:
  - 菜单信息项继续只读
- active:
  - 开关切换后即时保存并同步到 tray
- focus:
  - 设置页正常键盘切换开关
- disabled:
  - N/A
- loading:
  - 保存期间沿用现有 busy/auto-save 行为
- empty:
  - 顶部 title 关闭时为空
- error:
  - 沿用现有设置保存错误提示
- skeleton:
  - N/A
- optimistic (if applicable):
  - N/A

## Responsive Rules
- 设置页沿用现有偏好页布局；不新增复杂布局。

## Accessibility (a11y)
- keyboard navigation:
  - Switch 可键盘切换
- focus order:
  - 新开关位于 General 区 `Launch on login` 与 `Language` 之间
- aria labels:
  - 使用 `ui.showTimeToBreakInTray`
- contrast:
  - 沿用现有系统/前端组件
- reduced motion:
  - 不新增动画

## Design Tokens / Tailwind Mapping
- 沿用现有 `SettingsRow + Switch` 组合，不新增样式体系

## Micro-animations (optional)
- N/A

## Edge Cases
- long text:
  - 菜单详情可能改变长度，但只原地更新文本
- slow network:
  - N/A
- empty datasets:
  - 顶部 title 关闭或无 countdown 时为空
- permission denied:
  - N/A
- offline:
  - N/A

## Acceptance Criteria (UI)
- 菜单打开后状态时间不再冻结
- 顶部倒计时可在设置页开关控制
- 开关保存后立即生效

## Open Questions
- N/A
