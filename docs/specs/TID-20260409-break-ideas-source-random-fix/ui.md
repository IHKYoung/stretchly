# Task-ID: TID-20260409-break-ideas-source-random-fix

## Goals
- 让 break 页交互语真正来自 locale ideas 真源，而不是过渡 prompt 数组。
- 保持同一 break 内文案稳定，避免用户看到 prompt 跳变。

## Non-Goals
- 不改 break 页视觉样式、按钮布局、声音与背景。

## Screens & User Flows
- Primary flow: 用户进入 microbreak / long break 时，中央交互语从当前语言的 `miniBreakIdeas` / `longBreakIdeas` 选出一条并固定展示到本轮结束。
- Fallback / secondary flow: 当 `breakIdeasEnabled = false`，页面继续显示 `ui.breakCopy.defaultPrompt.{kind}`。
- User-visible boundary: 仅影响 break 页中央一句交互语的来源与轮换规律。
- Entrypoints / handoff cues: `App.tsx` break window 分支、`breakIdeasEnabled` 开关、当前语言设置。

## Component Tree
- `App.tsx`
  - `promptText`
    - `manualAwaiting`
    - `pickBreakPrompt(...)`
    - `ui.breakCopy.defaultPrompt.*`

## Interaction States
- hover: N/A
- active: N/A
- focus: N/A
- disabled: N/A
- loading: N/A
- empty: 当 ideas 列表为空时，回退到默认 prompt
- error: N/A
- skeleton: N/A
- optimistic (if applicable): N/A

## Responsive Rules
- 不涉及布局变化。

## Accessibility (a11y)
- keyboard navigation: 不变
- focus order: 不变
- aria labels: 不变
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
- long text: locale idea 文案较长时仍由现有 break prompt 文本容器自动换行。
- slow network: N/A
- empty datasets: 当 ideas 列表为空时，回退到 `ui.breakCopy.defaultPrompt.*`。
- permission denied: N/A
- offline: N/A

## Acceptance Criteria (UI)
- break 页面不再读取或依赖 `ui.breakCopy.prompts.*`。
- 交互语在一轮 break 内保持稳定，进入下一轮后可变化。
- 关闭 `breakIdeasEnabled` 后仍显示默认 prompt，不出现空白。

## Open Questions
- N/A
