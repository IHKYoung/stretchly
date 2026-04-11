# Task-ID: TID-20260411-site-landing-page

> 若本任务不涉及 UI/交互，可在此注明 `N/A` 并保持该文件存在（流程门禁要求）。  

## Goals
- 用一句话讲清 Pauza 是什么，并让页面第一屏就足够用户决定是否下载。
- 通过克制但有质感的动画和氛围，传达“不是粗暴打断，而是更安静的久坐提醒”。

## Non-Goals
- 不做长页面滚动叙事。
- 不做价格、FAQ、博客、团队介绍或截图画廊。

## Screens & User Flows
- Primary flow: 用户打开首页，第一眼看到产品定位和能力摘要，然后在同一屏完成下载决策。
- Fallback / secondary flow: 用户点击下载按钮后进入站内跳转页；若真实下载地址未配置，跳转页给出清晰 fallback 提示。
- User-visible boundary: `apps/site/index.html` 与 `apps/site/download/**`
- Entrypoints / handoff cues: 首页 hero CTA、下载架构说明、redirect loading state

## Component Tree
- ambient background
- spotlight layer
- hero copy block
- capability chips
- download CTA cluster
- feature rail / product surface panel
- redirect status card

## Interaction States
- hover: 按钮和 feature card 有轻微抬升与高光位移
- active: CTA 有压缩反馈
- focus: 所有可点元素有明确 focus ring
- disabled: 当下载地址未配置时由跳转页接管，不在首页直接禁用
- loading: redirect page 显示正在跳转状态
- empty: N/A
- error: redirect page 显示“尚未配置真实下载地址”
- skeleton: N/A
- optimistic (if applicable): N/A

## Responsive Rules
- Desktop: 单屏双栏或 6:5 分栏，原则上不需要滚动
- Mobile: 改为纵向堆叠，允许轻微滚动，但首屏仍保留产品定位与下载入口
- CTA 在移动端必须维持易点击尺寸

## Accessibility (a11y)
- keyboard navigation: 从品牌区到主标题、功能标签、下载按钮、跳转页按钮线性可达
- focus order: 文案后进入 CTA，再到次级链接
- aria labels: 下载按钮需明确平台语义
- contrast: 深浅叠加背景上的主要文案和 CTA 达到可读对比
- reduced motion: 对 `prefers-reduced-motion` 降级，保留静态背景和无位移动画

## Design Tokens / Tailwind Mapping
- typography: display 用衬线/人文风格，正文用更克制的 sans
- spacing: 大留白，块间距以 16 / 24 / 40 / 72 为主
- color usage: 冷灰底 + 青铜/冰蓝点缀，不走常见紫色 SaaS 风格
- key classes: N/A（本轮不用 Tailwind）

## Micro-animations (optional)
- hero 元素 stagger reveal
- pointer spotlight 跟随
- capability reel 文字轮换
- CTA shimmer / gradient drift

## Edge Cases
- long text: 文案必须保持短句，不让首屏高度失控
- slow network: 纯静态页面先显示结构和文案，不依赖远端数据
- empty datasets: N/A
- permission denied: 若浏览器阻止跳转，redirect 页需保留手动打开链接按钮
- offline: 首页仍可浏览；下载跳转页若离线则明确告知不可访问下载源

## Acceptance Criteria (UI)
- 首页在单屏内清晰表达 Pauza 的产品定位与能力摘要。
- 页面风格现代、克制、有高级感，不落入普通 SaaS landing page 模板感。
- 下载按钮显著可见，且跳转流程有明确 loading / fallback 状态。

## Open Questions
- 真实下载地址配置完之后，是否需要在首页再显示版本号与文件大小，可留待下一轮。
