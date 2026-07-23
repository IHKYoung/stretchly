# Task-ID: TID-20260723-release-014-github-publish

## Goals
- 官网下载按钮最终指向已发布的 0.1.4 Apple Silicon DMG。

## Non-Goals
- 不修改官网视觉、文案布局、动画或桌面端 UI。

## Screens & User Flows
- Primary flow: 用户点击首页“下载”，进入 `v0.1.4/Pauza_0.1.4_aarch64.dmg`。
- Fallback / secondary flow: GitHub latest API 不可用或返回旧版本时，按钮保持本地 pinned 0.1.4 URL。
- User-visible boundary: 仅下载目标版本变化。
- Entrypoints / handoff cues: `apps/site/index.html` 下载按钮与 `apps/site/download/targets.js`。

## Component Tree
- 静态下载按钮 -> latest release resolver -> pinned fallback URL。

## Interaction States
- hover: 沿用现有样式。
- active: 沿用现有样式。
- focus: 沿用现有样式。
- disabled: 不适用。
- loading: latest API 请求期间仍保留 pinned URL。
- empty: latest release 无匹配资产时保留 pinned URL。
- error: API 失败时保留 pinned URL。
- skeleton: 不适用。
- optimistic: 不适用。

## Responsive Rules
- 无布局变更。

## Accessibility (a11y)
- keyboard navigation: 沿用原生链接行为。
- focus order: 不变。
- aria labels: 保留现有“下载 Pauza”。
- contrast: 不变。
- reduced motion: 不变。

## Design Tokens / Tailwind Mapping
- typography: 不变。
- spacing: 不变。
- color usage: 不变。
- key classes: `.download-button` 不变。

## Micro-animations (optional)
- 不变。

## Edge Cases
- long text: 不适用。
- slow network: 初始 href 已是 0.1.4 pinned URL。
- empty datasets: 无匹配资产时不替换 pinned URL。
- permission denied: GitHub 下载失败由浏览器/GitHub 明确呈现。
- offline: 页面仍保留 URL，但下载需要网络。

## Acceptance Criteria (UI)
- `index.html` 与 `targets.js` 均指向相同 0.1.4 资产；latest API 的旧响应不会把链接回退到旧版本。

## Open Questions
- 无。
