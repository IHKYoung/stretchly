# Task-ID: TID-20260424-break-ideas-asset-migration

> 若本任务不涉及 UI/交互，可在此注明 `N/A` 并保持该文件存在（流程门禁要求）。  

## Goals
- N/A（本轮不改 UI 交互，只调整 break ideas 资产来源）

## Non-Goals
- 不新增设置项
- 不修改 break prompt 的视觉样式或交互状态

## Screens & User Flows
- Primary flow: N/A
- Fallback / secondary flow: N/A
- User-visible boundary: 无新的用户可见入口；break prompt 行为应保持与迁移前一致
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
- long text: 长文案仍由既有 `break-copy-layout` 与 typewriter 逻辑处理，本轮不改
- slow network: N/A（纯本地静态资产）
- empty datasets: 若未来某语言 bundle 不存在，应沿 fallback 链读取下一个可用 ideas bundle
- permission denied: N/A
- offline: N/A

## Acceptance Criteria (UI)
- 用户可见 break prompt 行为与迁移前一致，不因 source path 变化退回 default prompt 或空文案

## Open Questions
- N/A
