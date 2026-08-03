# Task-ID: TID-20260803-release-015-github-publish

## Goals
- 保持官网现有单按钮下载体验，只把 latest 结果和固定回退收口到 `v0.1.5`。
- 下载解析失败时保留明确、可重试的反馈，不静默下载旧版。

## Non-Goals
- 不调整官网视觉、版式、文案层级、动画或桌面端界面。

## Screens & User Flows
- Primary flow: 官网首页点击下载 -> 显示既有解析态 -> 获取 GitHub latest release -> 跳转 `Pauza_0.1.5_aarch64.dmg`。
- Fallback / secondary flow: latest API 不可用或无匹配资产 -> 使用 pinned `v0.1.5` URL；固定链接失败 -> 显示既有错误并允许重试。
- User-visible boundary: 官网首页下载按钮到浏览器开始下载，之后交给 macOS DMG 安装流程。
- Entrypoints / handoff cues: 主下载按钮、按钮忙碌/错误文案、浏览器下载记录、DMG Finder 窗口。

## Component Tree
- `apps/site/index.html` 下载按钮与内联解析逻辑。
- `apps/site/download/targets.js` 固定下载目标。
- GitHub latest release API 与 DMG asset。

## Interaction States
- hover: 沿用现有按钮 hover，不修改。
- active: 沿用现有点击反馈，不修改。
- focus: 沿用现有键盘焦点样式，不修改。
- disabled: 解析期间沿用现有防重复触发状态。
- loading: 点击后沿用现有正在解析/下载反馈。
- empty: latest release 无匹配 arm64 DMG 时自动进入 pinned fallback。
- error: latest 与 pinned 均不可用时显示现有错误提示。
- skeleton: 不适用，单按钮跳转没有内容骨架。
- optimistic: 不使用；只有解析出可用 URL 后才跳转。

## Responsive Rules
- 不修改布局；现有移动端与桌面端按钮行为保持一致。

## Accessibility (a11y)
- keyboard navigation: 现有原生按钮/链接键盘操作保持。
- focus order: 不新增节点，不改变焦点顺序。
- aria labels: 沿用现有可见按钮文本和标签。
- contrast: 不修改颜色。
- reduced motion: 不新增动画。

## Design Tokens / Tailwind Mapping
- typography: 不修改。
- spacing: 不修改。
- color usage: 不修改。
- key classes: 不修改。

## Micro-animations
- 不新增；沿用现有解析与错误反馈。

## Edge Cases
- long text: 不新增用户文案。
- slow network: loading 状态持续，API 失败后进入固定回退。
- empty datasets: latest response 无匹配 DMG 时进入固定回退。
- permission denied: GitHub asset 返回拒绝/错误时显示失败，不伪造成功。
- offline: latest 和固定链接均失败，停留官网并显示可重试错误。

## Acceptance Criteria (UI)
- 官网 primary/fallback 两条路径均指向 `v0.1.5`，无 `v0.1.4` 下载残留。
- 线上点击入口可解析到存在的 GitHub asset，错误路径仍为可见失败。
- 本次不产生视觉和布局回归。

## Open Questions
- 无；本次沿用既有交互，仅更新受控发布目标。
