# Task-ID: TID-20260412-settings-lock-inversion-deadlock-doc-commit

> 若本任务不涉及 UI/交互，可在此注明 `N/A` 并保持该文件存在（流程门禁要求）。  

## Goals
- N/A。本任务不新增或重设计 UI，只针对宿主层并发死锁做代码与文档纠偏。

## Non-Goals
- 不改设置页视觉、交互流程或 break 页面表现。

## Screens & User Flows
- Primary flow: N/A
- Fallback / secondary flow: N/A
- User-visible boundary: 仅修复“保存设置时出现 macOS 彩球卡死”的宿主层行为，不引入新页面或新状态。
- Entrypoints / handoff cues: N/A

## Component Tree
- N/A

## Interaction States
- hover: N/A
- active: N/A
- focus: N/A
- disabled: N/A
- loading: N/A
- empty: N/A
- error: N/A
- skeleton: N/A
- optimistic (if applicable): N/A

## Responsive Rules
- N/A

## Accessibility (a11y)
- keyboard navigation: N/A
- focus order: N/A
- aria labels: N/A
- contrast: N/A
- reduced motion: N/A

## Design Tokens / Tailwind Mapping
- typography: N/A
- spacing: N/A
- color usage: N/A
- key classes: N/A

## Micro-animations (optional)
- N/A

## Edge Cases
- long text: N/A
- slow network: N/A
- empty datasets: N/A
- permission denied: N/A
- offline: N/A

## Acceptance Criteria (UI)
- 不新增任何 UI 语义承诺；用户唯一可见变化是设置保存时不应再因该死锁出现卡死。

## Open Questions
- 无。
