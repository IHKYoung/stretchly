# Task-ID: TID-20260409-apps-single-source-refactor

## Goals
- N/A。本任务不新增或重排用户可见交互，只调整源码归属、locale 资产结构与文档口径。

## Non-Goals
- 不改设置页布局
- 不改 break prompt 交互契约
- 不新增视觉状态或组件

## Screens & User Flows
- Primary flow: N/A
- Fallback / secondary flow: N/A
- User-visible boundary: 无新增或变更
- Entrypoints / handoff cues: N/A

## Component Tree
- N/A。本任务不改组件树。

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
- keyboard navigation: 保持现状
- focus order: 保持现状
- aria labels: 保持现状
- contrast: 保持现状
- reduced motion: 保持现状

## Design Tokens / Tailwind Mapping
- typography: 不变
- spacing: 不变
- color usage: 不变
- key classes: 不变

## Micro-animations (optional)
- N/A

## Edge Cases
- long text: `break-message-copy.json` 文案仍通过现有 UI 容器承接，不新增专门交互
- slow network: N/A（本任务不引入网络）
- empty datasets: N/A
- permission denied: N/A
- offline: N/A

## Acceptance Criteria (UI)
- UI 契约保持不变，用户不会因为目录重构看到新的设置页或 break prompt 行为漂移。
- break 页面提示语的维护入口改到 `apps/desktop/src/locales/break-message-copy.json`，但展示结果保持与现有 UI 兼容。

## Open Questions
- 无
