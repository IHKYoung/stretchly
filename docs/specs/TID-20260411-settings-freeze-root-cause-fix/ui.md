# Task-ID: TID-20260411-settings-freeze-root-cause-fix

## Goals
- 让设置页在点击开关、分段控件、下拉和时间相关输入时保持响应，不再因宿主保存重入而冻结。

## Non-Goals
- 不改视觉样式。
- 不改设置项分组与文案。

## Screens & User Flows
- Primary flow:
  - 用户点击设置项或调整时间，UI 保持响应，保存异步完成。
- Fallback / secondary flow:
  - 保存尚未完成时继续修改，新的草稿排到下一轮保存，不与上一轮并发。
- User-visible boundary:
  - `节奏` 与 `偏好` 两个分类里的设置交互。
- Entrypoints / handoff cues:
  - 点击 preset / 开关 / Select / 输入 stepper。

## Component Tree
- `App.tsx` 主设置页
- `CompactNumber`
- `PresetChipGroup`
- autosave effect + `busyAction`

## Interaction States
- hover: 与现状一致
- active: 与现状一致
- focus: 数字输入仍支持 `Enter / Escape / blur`
- disabled: 保存中不再依赖“全页锁死”来保证一致性
- loading: `busyAction='save settings'`
- empty: N/A
- error: 继续显示既有 save error
- skeleton: N/A
- optimistic (if applicable): 草稿先留在前端，宿主按顺序落盘

## Responsive Rules
- 无新增响应式要求

## Accessibility (a11y)
- keyboard navigation: 保持原有 tab 顺序
- focus order: 不改变
- aria labels: 保持原有控件 aria
- contrast: 不变
- reduced motion: 不变

## Design Tokens / Tailwind Mapping
- typography: 不变
- spacing: 不变
- color usage: 不变
- key classes: 不变

## Micro-animations (optional)
- N/A

## Edge Cases
- long text: N/A
- slow network: Tauri 本地命令无网络，但慢保存时应保持后续编辑可排队
- empty datasets: N/A
- permission denied: 继续走既有错误 surfaced
- offline: N/A

## Acceptance Criteria (UI)
- 设置项点击与时间修改不会再因为保存链路重入导致界面冻结。
- 保存进行中继续编辑时，不会发起并发的第二个宿主保存。
- 数字输入已有的 `blur / Enter / Escape` 行为保持不变。

## Open Questions
- 如果用户本地 `settings.json` 因自定义壁纸过大仍有写盘卡顿，下一步需要把壁纸资产与主体设置拆开存储。
