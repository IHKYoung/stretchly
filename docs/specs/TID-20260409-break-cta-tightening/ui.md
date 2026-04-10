# Task-ID: TID-20260409-break-cta-tightening

> 若本任务不涉及 UI/交互，可在此注明 `N/A` 并保持该文件存在（流程门禁要求）。  

## Goals
- 收紧 break 页面 CTA，只在合适时机显示用户能执行的动作。

## Non-Goals
- 不重做 break 页面视觉样式或按钮文案。

## Screens & User Flows
- Primary flow: 倒计时进行中不再看到 `Done`；如果当前还在前 10 秒，则能看到 `Later`，过了窗口后只剩允许的其他动作。
- Fallback / secondary flow: 若 manual finish 开启且计时结束，主按钮显示 `Resume work`。
- User-visible boundary: 仅影响 break prompt 底部 CTA。
- Entrypoints / handoff cues: `?window=break`

## Component Tree
- `App.tsx` break prompt CTA 行

## Interaction States
- hover: unchanged
- active: CTA 只按后端能力显示
- focus: unchanged
- disabled: busy action 时沿用现有 disabled
- loading: N/A
- empty: N/A
- error: command 不可用时沿用现有错误处理
- skeleton: N/A
- optimistic (if applicable): preview transform 继续只模拟允许的按钮

## Responsive Rules
- CTA 行的响应式布局保持现状。

## Accessibility (a11y)
- keyboard navigation: unchanged
- focus order: 只会落到当前允许的按钮上
- aria labels: unchanged
- contrast: unchanged
- reduced motion: unchanged

## Design Tokens / Tailwind Mapping
- typography: unchanged
- spacing: unchanged
- color usage: unchanged
- key classes: unchanged

## Micro-animations (optional)
- unchanged

## Edge Cases
- long text: unchanged
- slow network: N/A
- empty datasets: N/A
- permission denied: N/A
- offline: unchanged

## Acceptance Criteria (UI)
- 普通倒计时阶段不显示提前完成按钮。
- `Later` 只在 host 判定 `canPostpone = true` 的前 10 秒窗口内显示。
- `Resume work` 只在 `manualAwaiting` 阶段显示。

## Open Questions
- N/A
