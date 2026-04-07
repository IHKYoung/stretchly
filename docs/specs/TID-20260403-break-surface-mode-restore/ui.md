# Task-ID: TID-20260403-break-surface-mode-restore

> 若本任务不涉及 UI/交互，可在此注明 `N/A` 并保持该文件存在（流程门禁要求）。  

## Goals
- 在当前极简设置页里恢复 break surface mode 的可见入口。
- 让“窗口 / 全屏”和“打断风格”重新成为两组不同控制，而不是相互替代。

## Non-Goals
- 不恢复旧 Electron 的大而全 settings 页面。
- 不新增 `show regular windows`、宽高百分比等更细高级项。
- 不改 break prompt React 内容。

## Screens & User Flows
- Primary flow: 用户进入“提醒与打断”，在“休息窗口 → 显示方式”中切换 `窗口 / 全屏` 并保存。
- Fallback / secondary flow: 若用户不保存，只看到 dirty 状态变化；break prompt 页面本身不新增设置入口。
- User-visible boundary: 设置页 `提醒与打断` 分类和 break window 的宿主显示模式。
- Entrypoints / handoff cues: 左侧“提醒与打断”导航、`休息窗口` 分组、`显示方式` segmented control。

## Component Tree
- `App`
- `SettingsGroup(休息窗口)`
- `SettingRow(显示方式)`
- `SegmentedControl(window/fullscreen)`
- `SettingsGroup(休息风格)`
- `SegmentedControl(gentle/balanced/immersive)`

## Interaction States
- hover: segmented control 使用现有轻量 hover/active 状态。
- active: `窗口 / 全屏` 当前选项高亮。
- focus: 继续复用 segmented control focus ring。
- disabled: 无改动时 `保存 / 撤销` disabled。
- loading: 初次 snapshot 前显示加载态。
- empty: N/A
- error: 顶部错误条承载命令失败信息。
- skeleton: 无
- optimistic (if applicable): 本地 form 先变脏，保存后与 snapshot 同步。

## Responsive Rules
- 延续当前极简 split view 规则；`显示方式` 使用单行 segmented control，在桌面宽度下保持水平排列。

## Accessibility (a11y)
- keyboard navigation: segmented control 仍可键盘聚焦。
- focus order: `显示方式` 紧跟严格模式组之后。
- aria labels: `ariaLabel` 使用 `显示方式`。
- contrast: 当前选中项与未选中项对比保持清晰。
- reduced motion: 无新增动效。

## Design Tokens / Tailwind Mapping
- typography: 继续沿用极简工具式标题体系。
- spacing: 使用现有 `SettingRow` 和 `SettingsGroup` 节奏，不引入新视觉块。
- color usage: 保持当前暖米色背景 + 白色主面板。
- key classes: 沿用 `SegmentedControl` 与现有 settings row 布局。

## Micro-animations (optional)
- 无新增动画。

## Edge Cases
- long text: 中英文“窗口 / 全屏 / Full screen”都应在 segmented control 中稳定显示。
- slow network: 设置页仍先显示加载态。
- empty datasets: N/A
- permission denied: N/A
- offline: N/A

## Acceptance Criteria (UI)
- “提醒与打断”页重新出现 `显示方式`。
- `显示方式` 与 `休息风格` 分成两组，不再混成一个设置。
- UI 不破坏当前极简 settings 架构。

## Open Questions
- 后续是否还要恢复 `show regular windows` 这类更细 surface 选项，暂不展开。
