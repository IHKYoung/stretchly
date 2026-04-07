# Task-ID: TID-20260404-fullscreen-break-close-fix

> 若本任务不涉及 UI/交互，可在此注明 `N/A` 并保持该文件存在（流程门禁要求）。  

## Goals
- 修复 fullscreen break 点击 `跳过` 后不再残留黑屏。

## Non-Goals
- 不改 break 页布局或按钮文案。

## Screens & User Flows
- Primary flow: fullscreen break 中点击 `跳过`，窗口退出 fullscreen 并彻底关闭。
- Fallback / secondary flow: `完成` / `稍后` 共享同一关闭链路。
- User-visible boundary: break fullscreen 退出行为。
- Entrypoints / handoff cues: break CTA `跳过 / 完成 / 稍后`。

## Component Tree
- N/A，本轮不改前端组件树。

## Interaction States
- hover: N/A
- active: N/A
- focus: N/A
- disabled: N/A
- loading: N/A
- empty: N/A
- error: 黑屏是宿主层关闭异常，本轮修复的就是这个状态
- skeleton: N/A
- optimistic (if applicable): N/A

## Responsive Rules
- N/A

## Accessibility (a11y)
- keyboard navigation: 按钮本身不变
- focus order: 不变
- aria labels: 不变
- contrast: 不变
- reduced motion: fullscreen 退出仍可能有系统级过渡动画

## Design Tokens / Tailwind Mapping
- N/A，本轮不改前端视觉层

## Micro-animations (optional)
- N/A

## Edge Cases
- fullscreen break 在 `跳过 / 完成 / 稍后` 任一关闭路径下都不应残留黑屏
- 非 fullscreen break 不应因 destroy 路径出现新回归

## Acceptance Criteria (UI)
- fullscreen break 关闭后不残留黑屏空间
- CTA 行为语义不变

## Open Questions
- 最终 fullscreen 现实确认仍需你本机手点一次
