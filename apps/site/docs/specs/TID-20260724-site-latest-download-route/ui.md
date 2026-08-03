# Task-ID: TID-20260724-site-latest-download-route

> 若本任务不涉及 UI/交互，可在此注明 `N/A` 并保持该文件存在（流程门禁要求）。

## Goals
- 保持现有页面视觉不变，只让右上角下载按钮导航到 latest release 的 Apple Silicon DMG。

## Non-Goals
- 不改按钮文案、样式、布局、动效、响应式规则或其它页面交互。

## Screens & User Flows
- Primary flow: 页面首屏提供可点击 pinned href；latest API 成功后透明更新为 `releases/latest/download/<asset-name>`；点击进入 GitHub 下载。
- Fallback / secondary flow: API 失败或缺少 DMG 时继续使用 pinned v0.1.4。
- User-visible boundary: 用户只感知点击“下载”后获得当前版本；不展示内部请求状态。
- Entrypoints / handoff cues: 首页右上角 `<a data-download-button aria-label="下载 Pauza">`。

## Component Tree
- `body > a.download-button[data-download-button]`；下载状态来自 `window.PAUZA_DOWNLOAD_TARGETS`，由 `script.js` 更新 href。

## Interaction States
- hover: 保持现有样式。
- active: 保持现有样式并由原生链接导航。
- focus: 保持原有键盘 focus。
- disabled: 不存在；按钮始终保留 fallback。
- loading: 不新增视觉态；href 保持 pinned 可用。
- empty: 无匹配资产时保持 pinned。
- error: API 失败时保持 pinned，控制台显性 warning。
- skeleton: 不适用，不新增骨架屏。
- optimistic (if applicable): 初始 pinned 是安全回退，不伪造 latest。

## Responsive Rules
- 无变化，沿用现有下载按钮定位与移动端规则。

## Accessibility (a11y)
- keyboard navigation: 原生 anchor 可 Tab/Enter 激活。
- focus order: 不变。
- aria labels: 保留 `aria-label="下载 Pauza"`。
- contrast: 不变。
- reduced motion: 下载解析不依赖动画；现有 reduced-motion 行为不变。

## Design Tokens / Tailwind Mapping
- typography: 不变。
- spacing: 不变。
- color usage: 不变。
- key classes: `.download-button` 不变。

## Micro-animations (optional)
- 不新增动画。

## Edge Cases
- long text: 资产名不展示给用户，仅做 URL 编码。
- slow network: 请求完成前 pinned 仍可点击。
- empty datasets: 无匹配 asset 回退 pinned。
- permission denied: 公共 GitHub API；被限流/拒绝时回退 pinned。
- offline: 页面若已加载，点击 fallback 仍需网络；不伪装离线下载能力。

## Acceptance Criteria (UI)
- 不发生视觉回归；正常路径点击 latest，失败路径点击 pinned；按钮始终保持原生链接语义。

## Open Questions
- 无。
