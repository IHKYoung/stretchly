# Task-ID: TID-20260403-tauri-i18n-resource-layer

## Goals
- 在不增加页面复杂度的前提下，恢复设置页语言选择。
- 让设置页、break prompt、tray 与状态文案在切换语言后保持一致。

## Non-Goals
- 不新增欢迎页、展示页或多余说明卡片。
- 不改变现有设置页结构、按钮层级和视觉方向。

## Screens & User Flows
- Primary flow: 用户打开设置页，在 `语言` 下拉框中切换 `简体中文 / English`，保存后主界面和宿主层文案同步切换。
- Fallback / secondary flow: 在浏览器 preview 中切换语言时，页面即时切换到对应 locale，用于视觉检查。
- User-visible boundary: 主设置页、break prompt、tray 菜单、状态文案。
- Entrypoints / handoff cues: 设置页顶部主面板中的语言字段。

## Component Tree
- `App.tsx`
  - `header`
  - `toolbar`
  - `panel`
    - `Group/Row`
    - language select
    - settings fields
  - `footer`
- `?window=break`
  - `prompt`

## Interaction States
- hover: 沿用现有按钮/输入 hover。
- active: 沿用现有按钮 active。
- focus: 语言选择与输入框保持浏览器原生 focus ring。
- disabled: busyAction 时按钮与 checkbox 保持禁用态。
- loading: 使用 `ui.loading`。
- empty: 无休息时使用 `ui.break.cleared*`。
- error: 原错误 banner 保留。
- skeleton: 无。
- optimistic (if applicable): preview 模式下采用本地 snapshot 预览切换结果。

## Responsive Rules
- 不新增新的响应式断点；继续沿用当前极简设置页布局。

## Accessibility (a11y)
- keyboard navigation: 语言选择与所有输入继续使用原生表单组件。
- focus order: 维持 DOM 顺序。
- aria labels: 继续依赖 label 包裹输入。
- contrast: 沿用当前暖白 + 深墨配色。
- reduced motion: 无新增动画。

## Design Tokens / Tailwind Mapping
- typography: 维持当前桌面工具风格。
- spacing: 不增加新层级，只在原分组内放入语言项。
- color usage: 无新增颜色语义。
- key classes: `group`、`row`、`prompt`、`toolbar`、`footer`。

## Micro-animations (optional)
- 无。

## Edge Cases
- long text: 英文和中文长度均应落在当前控件宽度内。
- slow network: 不涉及。
- empty datasets: 不涉及。
- permission denied: 不涉及。
- offline: 不涉及。

## Acceptance Criteria (UI)
- 设置页出现最小语言选择入口。
- 默认进入应用时显示中文。
- 切换并保存后，设置页与 break prompt 文案同步变化，不出现混合语言。

## Open Questions
- 暂无。
