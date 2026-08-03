# Task-ID: TID-20260803-site-download-015

## Goals
- 下载按钮在成功与回退路径都提供 v0.1.5。

## Non-Goals
- 不修改任何视觉、布局、文案或动效。

## Screens & User Flows
- Primary flow: 下载 -> loading -> latest v0.1.5 asset。
- Fallback / secondary flow: latest error/empty -> pinned v0.1.5；再次失败 -> visible error/retry。
- User-visible boundary: 首页下载按钮到浏览器开始下载。
- Entrypoints / handoff cues: 下载按钮、状态文案、浏览器下载。

## Component Tree
- 首页 download anchor -> targets config -> script resolver -> GitHub。

## Interaction States
- hover: 不变。
- active: 不变。
- focus: 不变。
- disabled: 请求期间既有行为不变。
- loading: 既有解析态不变。
- empty: latest 无资产时走 fixed。
- error: 双路径失败时可见。
- skeleton: 不适用。
- optimistic: 不使用。

## Responsive Rules
- 不修改，桌面与移动端保持现状。

## Accessibility (a11y)
- keyboard navigation: 原生链接行为不变。
- focus order: 不变。
- aria labels: `aria-label` 不变。
- contrast: 不变。
- reduced motion: 不新增动效。

## Design Tokens / Tailwind Mapping
- typography: 不变。
- spacing: 不变。
- color usage: 不变。
- key classes: 不变。

## Micro-animations
- 不修改。

## Edge Cases
- long text: 不适用。
- slow network: loading 后按既有超时/失败路径回退。
- empty datasets: latest assets 为空时 pinned。
- permission denied: 显示错误。
- offline: 显示错误并允许重试。

## Acceptance Criteria (UI)
- URL 版本更新但交互结构和状态语义不变；primary/fallback 同为 v0.1.5。

## Open Questions
- 无。
