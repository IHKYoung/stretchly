# Task-ID: TID-20260403-tauri-migration-foundation

## Goals
- 给 Tauri 迁移提供一个克制、清晰、可继续迭代的桌面工作台。
- 让新壳首先表达“宿主层已经在接管”，而不是复刻旧 Electron 设置页。

## Non-Goals
- 本轮不做完整产品设置中心。
- 本轮不呈现真正的 break session UI。
- 本轮不做数据分析 dashboard。

## Screens & User Flows
- Primary flow: 用户启动 Pauza Tauri dashboard，看到 hero、metrics、desktop shell capability、migration queue 和 legacy source basis，并可直接触发 focus session 或 launch on login。
- Fallback / secondary flow: 在浏览器中打开 preview 时，界面结构保持一致，但 CTA 使用本地模拟状态而不是原生 invoke。
- User-visible boundary: 只覆盖新 `apps/desktop` 的 dashboard，不覆盖旧 Electron 的 break / preferences / welcome 页面。
- Entrypoints / handoff cues: hero CTA、底部状态栏、tray 菜单、全局快捷键。

## Component Tree
- `App`
  - `hero`
  - `metrics`
  - `error-banner`（仅出错时）
  - `panel: shell capabilities`
  - `panel: migration queue`
  - `panel: legacy source basis`
  - `footer`

## Interaction States
- hover: CTA 与 panel footer button 提供轻量 hover 边框/背景变化。
- active: 按钮按下后立即进入忙碌态，focus session 和 autostart 文案同步更新。
- focus: 浏览器与 Tauri runtime 都保留标准按钮焦点态。
- disabled: `busyAction !== null` 时按钮降低透明度并禁用。
- loading: 首次快照加载期间 metrics 显示 `loading` 或默认文案。
- empty: 默认无 focus session 时显示 `No focus session active`。
- error: 若真实 runtime 调用失败，显示 error banner；若仅缺失 Tauri runtime，则自动进入 preview，不直接报错。
- skeleton: N/A
- optimistic (if applicable): preview mode 下 CTA 直接用本地状态模拟成功结果。

## Responsive Rules
- 桌面宽屏采用 hero 双栏、三列 metrics、三列 grid。
- `1100px` 以下统一折叠为单列。
- `720px` 以下收窄外边距，保持可读性。

## Accessibility (a11y)
- keyboard navigation: 所有 CTA 都使用原生 `button`，可被 Tab 访问。
- focus order: 从 hero CTA 到 metrics 再到 panels 与 footer，遵循 DOM 顺序。
- aria labels: 当前按钮文本已足够直接，不额外引入隐藏 label。
- contrast: 深墨色正文配暖白背景，主 CTA 使用深绿色底和浅米文字。
- reduced motion: 当前仅保留轻量 hover/disabled 过渡，无强动画。

## Design Tokens / Tailwind Mapping
- typography: 标题使用 `Iowan Old Style / Palatino / Georgia`，正文使用 `SF Pro Text / Segoe UI / sans-serif`。
- spacing: 外层 `32px`，卡片内 `22-30px`，统一圆角与留白节奏。
- color usage: 暖白背景、深墨色文字、低对比细边框、墨绿色主 CTA、灰蓝色 preview 状态。
- key classes: `.shell`、`.hero`、`.metrics`、`.panel`、`.status`、`.footer`。

## Micro-animations (optional)
- 当前仅保留按钮 hover / disabled 的轻量过渡，不加入额外动画。

## Edge Cases
- long text: capability 与 migration detail 以多行段落承接，不依赖单行截断。
- slow network: preview 模式本地可渲染，减少“空白页”体验。
- empty datasets: 无 focus session 即为默认 empty state。
- permission denied: autostart / shortcut 等宿主动作失败时通过 error banner 和 footer 暴露错误。
- offline: 当前无网络依赖。

## Acceptance Criteria (UI)
- 屏幕打开后，hero、metrics、grid 和 footer 的层级清晰，不依赖发光或重玻璃质感。
- preview mode 下页面无运行时崩溃，可供设计检查和截图取证。
- focus session、autostart 和 hide dashboard 的动作反馈能在 footer 或 metrics 中看见。

## Open Questions
- 下一轮是否把这个 dashboard 演进为正式的 today / focus 控制台。
