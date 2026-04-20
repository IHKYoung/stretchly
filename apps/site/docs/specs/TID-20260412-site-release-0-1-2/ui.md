# Task-ID: TID-20260412-site-release-0-1-2

## Goals
- 让用户点击首页右上角“下载”按钮时获得 `0.1.2` 版本。
- 在 release 切换窗口期保持按钮目标稳定，不被旧 latest API 响应覆盖。

## Non-Goals
- 不改页面版式、字体、动效、文案轮播与互动粒子效果。
- 不新增版本选择器、平台切换器或下载状态提示组件。

## Screens & User Flows
- Primary flow: 打开首页后，下载按钮加载完成并最终指向 `Pauza_0.1.2_aarch64.dmg`；点击按钮后跳转到该资产。
- Fallback / secondary flow: 若 GitHub latest API 不可用，或仍返回 `0.1.1` 资产名，则按钮继续保持 pinned `0.1.2` URL。
- User-visible boundary: 仅首页右上角下载按钮的目标 URL 与点击结果。
- Entrypoints / handoff cues: 用户进入首页即可见下载按钮，无额外弹窗、toast 或二次确认。

## Component Tree
- `body`
- `a.download-button[data-download-button]`
- `script.js -> getDownloadConfig() -> resolveLatestDownloadUrl()`

## Interaction States
- hover: 沿用现有 hover 样式，不改视觉反馈。
- active: 沿用现有点击按压反馈；点击后跳转到最终解析出的 URL。
- focus: 保留既有键盘 focus-visible 样式。
- disabled: 不新增 disabled 态；按钮始终可点并至少有 pinned URL 作为兜底。
- loading: 不新增独立 loading UI；页面加载期间按钮先持有固定 href，异步校验完成后可能保持原值。
- empty: 不允许出现空链接；若 API 无资产则继续保留 pinned URL。
- error: 不展示新错误态；错误仅以控制台警告记录。
- skeleton: 不涉及。
- optimistic (if applicable): 不涉及。

## Responsive Rules
- 下载按钮在桌面端与移动端都沿用现有右上角布局，不改尺寸与断点逻辑。
- 本次交互变更不引入新的响应式组件或条件渲染。

## Accessibility (a11y)
- keyboard navigation: 保留 `<a>` 原生键盘可达性。
- focus order: 下载按钮继续保持在页面首个可聚焦入口之一。
- aria labels: 维持现有 `aria-label="下载 Pauza"`。
- contrast: 不改颜色与对比度。
- reduced motion: 不新增下载相关动画，现有 reduced-motion 逻辑保持不变。

## Design Tokens / Tailwind Mapping
- typography: 沿用 `LXGW WenKai Screen`。
- spacing: 下载按钮外距与定位规则保持现状。
- color usage: 不改色板。
- key classes: `download-button`

## Micro-animations (optional)
- 不新增下载相关微动效。

## Edge Cases
- long text: 不涉及。
- slow network: 按钮先保留 pinned href，避免等待 GitHub API 才可下载。
- empty datasets: 若 release 资产列表未命中目标后缀，则继续使用 pinned href。
- permission denied: GitHub API 公开读取，不涉及鉴权。
- offline: 无法请求 API 时继续保留 pinned href；真正下载是否成功取决于用户网络。

## Acceptance Criteria (UI)
- 首屏渲染后，下载按钮最终 href 为 `Pauza_0.1.2_aarch64.dmg`。
- 旧 latest API 响应不会把按钮改回 `Pauza_0.1.1_aarch64.dmg`。
- hover / focus / 点击手感与现有页面保持一致，无额外 UI 回归。

## Open Questions
- 无。
