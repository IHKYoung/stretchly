# Task-ID: TID-20260409-janh-key-rename

> 若本任务不涉及 UI/交互，可在此注明 `N/A` 并保持该文件存在（流程门禁要求）。  

## Goals
- N/A。本任务不新增或改变任何用户可见 UI，只统一 locale key 名与 legacy HTML 引用。

## Non-Goals
- 不改设置页布局
- 不改 break prompt
- 不改按钮、文案内容或视觉层次

## Screens & User Flows
- Primary flow: N/A
- Fallback / secondary flow: N/A
- User-visible boundary: 无新增用户可见变化
- Entrypoints / handoff cues: N/A

## Component Tree
- N/A

## Interaction States
- hover: 不变
- active: 不变
- focus: 不变
- disabled: 不变
- loading: N/A
- empty: N/A
- error: N/A
- skeleton: N/A
- optimistic (if applicable): N/A

## Responsive Rules
- N/A

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
- long text: 不变
- slow network: N/A
- empty datasets: N/A
- permission denied: N/A
- offline: N/A

## Acceptance Criteria (UI)
- 仓库中不会因为旧 key 名残留，导致 legacy i18n 引用与 locale source 不一致。
- 用户可见内容不发生变化。

## Open Questions
- 无
