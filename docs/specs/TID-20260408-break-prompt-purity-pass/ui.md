# Task-ID: TID-20260408-break-prompt-purity-pass

> 若本任务不涉及 UI/交互，可在此注明 `N/A` 并保持该文件存在（流程门禁要求）。  

## Goals
- 把 break prompt 从展示型的双栏界面收回成单列、纯净、可直接阅读的休息界面。
- 用一条交互语和一个倒计时建立主视觉，不再让标签、说明块和装饰性组件抢焦点。

## Non-Goals
- 不改 CTA 语义、提醒状态机或宿主窗口行为。
- 不新增自定义 prompt 编辑器。
- 不回退到 Electron 旧页面实现。

## Screens & User Flows
- Primary flow: break 弹出后，界面中央直接显示一条交互语，下方是大号数字倒计时和细条形进度；底部保留 Done / Later / Skip。
- Fallback / secondary flow: 关闭 prompt 轮播或 locale prompt 缺失时，界面改用按 break kind 的默认 prompt；manual finish 时倒计时归零并显示等待完成说明。
- User-visible boundary: 仅 break prompt 主界面和设置页里 `交互语` / 开始音相关命名。
- Entrypoints / handoff cues: 自动弹出的 `?window=break` 页面；偏好页里控制 `交互语` 和开始音的设置卡。

## Component Tree
- `BreakWindow`
  - optional top-right current clock
  - centered translucent surface
    - single prompt sentence
    - digital countdown
    - thin linear progress bar
    - optional manual-awaiting note
    - CTA row

## Interaction States
- hover: CTA 保持轻量 hover，不再给主视觉新增 hover 组件
- active: 倒计时数字与条形进度同步更新
- focus: CTA、switch、select 继续遵循当前键盘可达顺序
- disabled: busyAction 时 CTA 保持不可点击
- loading: 继承当前前台 loading，不新增独立 skeleton
- empty: prompt 轮播关闭时退回默认 prompt
- error: locale 缺失时必须显示默认 prompt，而不是直接露出 key
- skeleton: N/A
- optimistic (if applicable): 设置页保持现有自动保存体验，本轮不新增 optimistic UI 语义

## Responsive Rules
- break prompt 在 desktop 和较窄窗口里都保持单列；不再因为屏宽进入双栏。
- prompt 文案最大宽度约束在可读范围内，避免长句在超宽窗口下拉得过长。
- CTA 在窄宽度下纵向堆叠，在更宽布局下横向排列。

## Accessibility (a11y)
- keyboard navigation: CTA 与设置控件继续支持 Tab 导航
- focus order: 先 CTA，后顶部可选时钟之外的其他操作性元素；主 prompt 不参与焦点流
- aria labels: 倒计时本轮不新增复杂 aria；设置中的 select/switch 继续使用现有 label
- contrast: prompt 与倒计时使用高对比深色文本，背景保持轻半透明承托
- reduced motion: 移除分栏/圆环带来的额外动势，仅保留条形进度宽度变化

## Design Tokens / Tailwind Mapping
- typography: prompt 用大字号、紧字距；倒计时用更大的 tabular-nums，形成唯一视觉锚点
- spacing: 主界面依赖大面积留白和单层容器，不再叠多层卡片
- color usage: 背景继续沿用现有主题，但弱化为底层氛围；前景层以白色半透明 surface 承托
- key classes: `max-w-[760px]`、`rounded-[36px]`、`backdrop-blur-[20px]`、`tabular-nums`

## Micro-animations (optional)
- 仅保留条形进度的宽度过渡；不再依赖圆环或左右内容块切换形成“演示感”。

## Edge Cases
- long text: prompt 句子换行显示，但保持单句语气和居中阅读节奏
- slow network: N/A（本轮均为本地资源）
- empty datasets: prompt 数组为空时回退到默认 prompt
- permission denied: N/A（本轮不新增权限）
- offline: 全部本地资源，不依赖网络

## Acceptance Criteria (UI)
- break prompt 在视觉上只剩一条交互语、一个数字倒计时和一条细进度条作为主内容。
- break prompt 不再出现左右分栏、圆环倒计时、cue 卡片和 break kind 标签。
- 设置页不再露出 `ui.microbreakStartSound` / `ui.longBreakStartSound` 这类原始字段名。
- 交互语关闭时仍有自然的默认 prompt，不会出现空白 break 页面。

## Open Questions
- 暂无。本轮只做减法，不再继续扩展 break prompt 模块数量。
