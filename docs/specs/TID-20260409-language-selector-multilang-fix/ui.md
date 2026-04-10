# Task-ID: TID-20260409-language-selector-multilang-fix

## Goals
- 将设置页语言选择从只适合 2 个选项的 segmented control 改成适合完整 locale 列表的下拉选择。

## Non-Goals
- 不重做设置页布局。

## Screens & User Flows
- Primary flow: 用户打开设置页 -> `通用` -> `语言` -> 打开下拉 -> 选择任意 locale -> 自动保存
- Fallback / secondary flow: 无
- User-visible boundary: 仅设置页语言行
- Entrypoints / handoff cues: 设置页 `通用` 分组中的 `语言`

## Accessibility (a11y)
- keyboard navigation: 使用原有 `Select` 组件键盘交互
- aria labels: 继续使用 `ui.language`

## Acceptance Criteria (UI)
- 语言控件可容纳完整语言列表，不发生横向挤压或只显示两项。
