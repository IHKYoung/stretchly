# Task-ID: TID-20260409-smart-reminder-ui-simplification

> 若本任务不涉及 UI/交互，可在此注明 `N/A` 并保持该文件存在（流程门禁要求）。  

## Goals
- 降低设置页复杂度，把 smart reminder 的策略参数重新收回到内部默认。

## Non-Goals
- 不修改 smart reminder 的行为文案。
- 不改侧栏分类结构。

## Screens & User Flows
- Primary flow: 用户进入 `偏好 -> 智能暂停` 时，只看到 natural breaks / DND，不再看到额外的 `等待空档` 数字控件。
- Fallback / secondary flow: 无。
- User-visible boundary: 仅影响设置页可见项数量。
- Entrypoints / handoff cues: `SectionLabel = 智能暂停 / Smart pause` 下不再出现额外数值输入。

## Component Tree
- `App.tsx`
  - `SectionLabel`
  - `SettingsCard`
  - `SettingsRow(naturalBreaks)`
  - `SettingsRow(dnd)`

## Interaction States
- hover:
  - 沿用现有控件
- active:
  - 无新增交互
- focus:
  - 无新增交互
- disabled:
  - 无
- loading:
  - 无
- empty:
  - 无
- error:
  - 无
- skeleton:
  - 无
- optimistic (if applicable):
  - 无

## Responsive Rules
- 智能暂停分组内容更短，窄宽度下不再需要容纳额外数值控件。

## Accessibility (a11y)
- keyboard navigation:
  - 因为少了一个控件，键盘焦点序更简单
- focus order:
  - smart pause 分组只保留现有 switch/number 组合
- aria labels:
  - 沿现有控件
- contrast:
  - 不变
- reduced motion:
  - 不变

## Design Tokens / Tailwind Mapping
- typography:
  - 不变
- spacing:
  - 分组垂直高度减少
- color usage:
  - 不变
- key classes:
  - 不变

## Micro-animations (optional)
- 无

## Edge Cases
- long text:
  - 无新增
- slow network:
  - N/A
- empty datasets:
  - N/A
- permission denied:
  - N/A
- offline:
  - N/A

## Acceptance Criteria (UI)
- 设置页中不再出现 `等待空档 / Wait for a pause` 控件。
- smart pause 分组仍正常显示 natural breaks / DND。

## Open Questions
- 无
