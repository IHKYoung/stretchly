# Task-ID: TID-20260409-break-message-copy-centralization

## Goals
- 保持现有 break 消息页的视觉与交互不变，同时把可见提示语收口到一个明确、可编辑的位置。

## Non-Goals
- 不调整 break 页布局、按钮层级、动效或倒计时表现。
- 不新增新的设置入口或语言切换交互。

## Screens & User Flows
- Primary flow: break 页面继续显示主提示语、倒计时和 Done / Later / Skip 操作，但这些文案统一从 `break-message-copy.ts` 读取。
- Fallback / secondary flow: 当 `breakIdeasEnabled` 关闭时，页面继续显示按 break kind 的默认提示语；当语言回退时继续走现有 `normalizeLanguage()` 结果。
- User-visible boundary: 仅限 `?window=break` 的消息文案，不涉及设置页布局和运行时动作逻辑。
- Entrypoints / handoff cues: break 页面中央提示语、手动完成等待文案、底部 CTA 标签。

## Component Tree
- `BreakWindow`
- `promptText`
- `countdownText`
- `Done / Later / Skip` CTA

## Interaction States
- hover: 沿用现有 Button 视觉，无新增 hover 状态。
- active: 沿用现有 Button active 状态，无新增交互分支。
- focus: 焦点顺序与键盘可达性保持不变。
- disabled: CTA 的禁用逻辑保持当前实现，只替换标签来源。
- loading: 不新增 loading UI。
- empty: 无活动休息时继续显示 `clearedDetail`。
- error: 不新增页面级错误态；若语言缺失则回退到 `normalizeLanguage()` 对应语言。
- skeleton: 不涉及。
- optimistic (if applicable): 不涉及。

## Responsive Rules
- 断点与排版保持现状；本任务不修改 break 页的响应式结构。

## Accessibility (a11y)
- keyboard navigation: 现有按钮键盘导航保持不变。
- focus order: 不改 DOM 结构，不调整焦点顺序。
- aria labels: 不新增 aria 属性需求。
- contrast: 不改现有配色。
- reduced motion: 不新增动画。

## Design Tokens / Tailwind Mapping
- typography: 沿用现有 break 页字体与字号层级。
- spacing: 沿用现有 break 页间距。
- color usage: 沿用现有主题与玻璃卡片配色。
- key classes: 无新增关键类名要求。

## Micro-animations (optional)
- 不涉及。

## Edge Cases
- long text: 提示语应保持简短，避免破坏现有 `max-w-[22ch]` 文案块。
- slow network: 不涉及。
- empty datasets: 若随机提示语列表为空，继续回退到 default prompt。
- permission denied: 不涉及。
- offline: 不涉及。

## Acceptance Criteria (UI)
- break 页面结构和视觉表现不变。
- 主提示语、默认提示语、等待完成文案与 CTA 标签都能从单一文件维护。
- 关闭随机提示语时，页面仍显示对应 break kind 的默认提示语。

## Open Questions
- 无。
