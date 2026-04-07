# Task-ID: TID-20260402-modern-break-experience

> 若本任务不涉及 UI/交互，可在此注明 `N/A` 并保持该文件存在（流程门禁要求）。  

## Goals
- 用更现代、更克制的视觉把 break 体验从“拦截”变成“邀请”。
- 在不增加复杂性的前提下，让设置页能清晰表达不同干预风格。

## Non-Goals
- 不重写为前端框架。
- 不新增复杂数据面板或 Apple 生态入口。

## Screens & User Flows
- Primary flow: 用户工作中收到 gentle/balanced/immersive 对应风格的 break 卡片，优先感知“稍后回来”与“完成休息”动作。
- Fallback / secondary flow: 用户通过 tray 手动进入 Focus session，主动保护心流；strict mode 下继续保留更强约束。
- User-visible boundary: `welcome.html`、`preferences.html`、`microbreak.html`、`break.html`、tray menu。
- Entrypoints / handoff cues: 首次启动欢迎页、设置页中的 interruption style、tray 中的 focus session、break card action 区。

## Component Tree
- Welcome hero + language panel + action card
- Preferences shell + top nav + settings cards + schedule cards + theme/about sections
- Break shell + floating/centered card + title/idea/text + progress + action buttons

## Interaction States
- hover: 按钮与导航卡片具有轻量高亮和阴影变化
- active: 主按钮按下有更深色背景和轻微位移反馈
- focus: 交互控件保留清晰 focus ring
- disabled: strict mode 或时机不允许时隐藏/禁用 postpone、skip
- loading: N/A
- empty: N/A
- error: N/A
- skeleton: N/A
- optimistic (if applicable): N/A

## Responsive Rules
- Welcome / preferences 在较窄窗口下退化为单列卡片布局
- Break gentle micro card 保持紧凑；long break 保持中心卡片比例而非全屏铺满

## Accessibility (a11y)
- keyboard navigation: break action / nav / form controls 均可键盘触达
- focus order: 从主要 CTA 到次要设置自然流动
- aria labels: 复用现有语义元素和按钮文本
- contrast: 深浅主题下按钮与正文保持足够对比
- reduced motion: 动画仅限轻量过渡，不依赖强运动表达

## Design Tokens / Tailwind Mapping
- typography: 使用现有 Noto 字体族，提升字重对比与大标题层次
- spacing: 更大的卡片内边距、24/32/40 为主节奏
- color usage: 暖色高光 + 深色背景渐变 + 半透明卡片
- key classes: `break-card`、`break-shell`、`welcome-panel`、`preferences-panel`、`panel-card`

## Micro-animations (optional)
- 卡片进入阴影和轻微位移过渡；按钮 hover/focus 过渡；避免夸张动画

## Edge Cases
- long text: break 文案支持自动换行与较宽文本容器
- slow network: N/A
- empty datasets: 无 idea 时仍显示标题、动作和进度
- permission denied: N/A
- offline: N/A

## Acceptance Criteria (UI)
- welcome / preferences / break card 三处视觉语言统一
- gentle 模式明显减轻“全屏挡住你”的压迫感
- 设置页中 interruption style 容易理解并可直接修改

## Open Questions
- 暂不引入额外多语言 UI 文案润色，只保证新增文案语义清晰可用
