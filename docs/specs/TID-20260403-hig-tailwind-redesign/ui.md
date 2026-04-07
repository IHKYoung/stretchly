# Task-ID: TID-20260403-hig-tailwind-redesign

## Goals
- 将主设置页重构为更符合 Apple HIG 的 grouped settings，减少“控制台感”和展示型噪音。
- 将前台视觉层迁到 Tailwind CSS v4 + shadcn 风格组件，统一 token、圆角、玻璃面板和交互状态。
- 将 break prompt 收敛为更聚焦的单卡片体验，用环形倒计时和清晰 CTA 替代旧版普通内容卡片。

## Non-Goals
- 不重写 Rust host 或 tray/menu 行为。
- 不新增统计、历史记录、图表等展示型模块。
- 不引入暗黑模式或多主题系统。

## Screens & User Flows
- Primary flow:
  - 打开主窗口
  - 先看到 overview hero、live overview 和三张摘要卡
  - 继续向下按 `节奏 / 信号 / 通用 / 高级` 调整设置
  - 通过独立 save rail 执行 `保存 / 撤销 / 默认值`
- Fallback / secondary flow:
  - 通过 `?window=break` 进入独立 break prompt
  - 查看当前时间、环形倒计时和 break 文案
  - 根据能力显示 `完成 / 稍后 / 跳过`
- User-visible boundary:
  - `main` 主设置页
  - `?window=break` break prompt
- Entrypoints / handoff cues:
  - dirty/synced badge
  - save rail
  - break prompt 顶部 label 与主按钮

## Component Tree
- `App`
  - `Hero Card`
    - product/runtime badges
    - hero headline + subtitle
    - `MetricTile` x3
    - quick action buttons
    - live overview side panel
  - `SectionShell(schedule)`
    - `SettingRow(microbreaks)`
    - `SettingRow(longBreaks)`
  - `SectionShell(signals)`
    - `SettingRow(naturalBreaks)`
    - `SettingRow(dnd)`
    - `SettingRow(breakSurface)`
  - `SectionShell(general)`
    - `SettingRow(language)`
    - `SettingRow(launchOnLogin)`
  - `Advanced Card`
    - `Accordion(notifications | postpone | strict | app-exclusions | shortcuts)`
  - `Save Rail`
  - `Live Signals Rail`
  - `Break Surface Rail`
- `BreakWindow`
  - header row
  - circular timer panel
  - title + detail
  - action buttons

## Interaction States
- hover:
  - button / segmented item / accordion trigger 使用更亮的玻璃层背景
- active:
  - segmented control 激活项为深色胶囊
  - primary CTA 为深蓝填充
- focus:
  - 所有交互控件统一使用较柔和的 `ring`，避免刺眼蓝边
- disabled:
  - save 按钮在 `dirty=false` 时禁用
  - busyAction 期间 CTA 禁用
- loading:
  - 初始仍保留单页 loading 文案
- empty:
  - `currentBreak=null` 时 break prompt 展示 cleared 文案
- error:
  - 主设置页顶部展示 error banner
- skeleton:
  - 不引入 skeleton；以极简 loading 文案代替
- optimistic (if applicable):
  - browser preview 下命令通过 previewTransform 立即更新本地 snapshot

## Responsive Rules
- `xl` 以上：hero 右侧展示 live overview，主体左主右辅双列
- `xl` 以下：rail 卡片堆叠到主列之后
- 摘要卡、快捷操作和高级区在窄屏下自动纵向堆叠
- 控件保持 44px 左右的最小可点按高度

## Accessibility (a11y)
- keyboard navigation:
  - 保留原生 button / input / textarea / radix switch/select/accordion 键盘行为
- focus order:
  - overview -> grouped settings -> advanced -> save rail
- aria labels:
  - segmented control 带 `ariaLabel`
  - switch 通过行内 label 或 chip label 提供可读文本
- contrast:
  - 深蓝主按钮与深色标题保证主信息对比度
  - 次信息统一使用 muted 文本，避免过低透明度
- reduced motion:
  - 仅使用轻量 surface-in 与 orb-float；即便关闭动画也不影响信息结构

## Design Tokens / Tailwind Mapping
- typography:
  - `SF Pro Text / SF Pro Display / Inter` 混合字体栈，标题采用更紧的负字距
- spacing:
  - 主卡 `p-6`，setting row `px-5 py-5`，rail 卡片以 `gap-4/5` 组织
- color usage:
  - 背景使用冷白到浅蓝渐变
  - 卡片为半透明白色玻璃面板
  - 主色为深蓝 `#1f3a78`
- key classes:
  - `rounded-[32px]`
  - `border-white/70`
  - `bg-white/72`
  - `backdrop-blur-2xl`
  - `shadow-[0_28px_70px_-42px_rgba(15,23,42,0.42)]`

## Micro-animations
- `animate-surface-in`：页面进入时的卡片上浮
- `animate-orb-float`：背景大光斑的缓慢漂浮

## Edge Cases
- long text:
  - 使用更宽的多列布局和 `leading-6/7`，避免中文拥挤
- slow network:
  - preview fallback 不依赖网络；Tauri 命令失败时走 error banner
- empty datasets:
  - 无 active break 时展示 cleared state
- permission denied:
  - autostart / Tauri invoke 失败通过 error banner 回显
- offline:
  - 本页不依赖外网资源

## Acceptance Criteria (UI)
- 主设置页默认第一屏能同时看到 hero、三张摘要卡和 live overview。
- 默认设置项按 `节奏 / 信号 / 通用` 三组呈现，高级项折叠。
- 所有高频交互控件收敛为 segmented control、switch chip、glass card、accordion。
- break prompt 保持单卡片、环形倒计时、清晰主按钮与当前时间显示。

## Open Questions
- 无；本轮在既定 Tauri command boundary 内完成。
