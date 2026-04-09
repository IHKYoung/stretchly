# Task-ID: TID-20260408-break-prompt-parity-upgrade

> 若本任务不涉及 UI/交互，可在此注明 `N/A` 并保持该文件存在（流程门禁要求）。  

## Goals
- 让 Tauri break prompt 重新具备更完整的休息氛围，而不是只剩基础计时器。
- 让偏好页重新暴露和 break 体验直接相关的背景、声音与 cue 配置入口。

## Non-Goals
- 不回退到 Electron 的原 HTML/CSS 页面
- 不改 break CTA 语义、strict mode 和 reminder state machine
- 不做用户自定义 cue 列表编辑器

## Screens & User Flows
- Primary flow: 用户在偏好页选择背景主题或上传自定义壁纸，配置随机交互语和开始音后，下一次 break 以增强版 prompt 展示 cue card、线性倒计时和圆形倒计时。
- Fallback / secondary flow: 若未上传自定义壁纸或移除已上传图片，break 自动回退到预设主题；选择静音或音量为 0 时，不播放进场音。
- User-visible boundary: 仅偏好页 break surface / break sounds 配置区和 break prompt 的展示内容。
- Entrypoints / handoff cues: 主窗口 `偏好` 分类下的 `休息窗口` / `休息音效` 卡片；运行时自动打开的 `?window=break` 窗口。

## Component Tree
- `BreakWindow`
  - top badge + optional current clock
  - primary content card
    - break title / detail
    - cue card（随机交互语）
    - linear countdown meter
  - circular countdown surface
  - CTA row
  - optional sound hint row
- `Preferences`
  - `休息窗口`
    - display mode segmented control
    - backdrop theme select
    - custom wallpaper chooser + preview + remove
    - random cue toggle
    - current clock toggle
  - `休息音效`
    - microbreak sound select
    - long break sound select
    - volume slider

## Interaction States
- hover: 按钮、select 和上传入口沿用当前 shadcn 样式
- active: break 中 cue card、线性倒计时和圆形倒计时同步更新
- focus: CTA、select、switch、上传按钮保持键盘可达
- disabled: 保存中或执行命令中，配置按钮和 CTA 继续遵循现有 disabled 规则
- loading: 继承当前前台 loading 状态
- empty: 没有自定义壁纸时，设置卡展示空态说明并回退默认主题
- error: 自定义壁纸解码失败时，通过现有错误条提示
- skeleton: N/A
- optimistic (if applicable): 设置页仍是自动保存，前端先更新表单再写回 snapshot

## Responsive Rules
- break prompt 保持当前“两列内容 + 右侧圆形计时器”的宽屏结构，小屏下退化为上下堆叠。
- cue card 和线性倒计时都位于左侧主信息卡内部，不再依赖额外弹层。
- 自定义壁纸预览在设置页中保持紧凑小卡，不放大为独立编辑页。

## Accessibility (a11y)
- keyboard navigation: 上传按钮、select、switch 和 CTA 均可通过 Tab 到达
- focus order: 继续遵循设置卡从上到下、从左到右的自然顺序
- aria labels: 新增上传按钮和音量 slider 继续复用明确的 `aria-label`
- contrast: cue card / 倒计时 / CTA 保持深色文本和浅色宿主层的对比
- reduced motion: 仍沿用现有轻量过渡，不新增强动画

## Design Tokens / Tailwind Mapping
- typography: 标题沿用 break prompt 当前的大字号紧凑字距；cue card 和 meter 使用更轻的辅助文本等级
- spacing: 主信息卡、cue card 和 meter 采用 `rounded-[22px~28px]` 的较大圆角与紧凑内边距
- color usage: 预设主题通过 `BreakScene` 控制背景渐变、cue tag、meter fill 和圆形计时器 accent；自定义壁纸叠加统一半透明深色罩层
- key classes: `animate-surface-in`、`animate-orb-float`、`backdrop-blur-xl`、自定义 preview 的 `bg-cover bg-center`

## Micro-animations (optional)
- 保留已有 `surface-in` 和 `orb-float`
- 线性倒计时条继续使用 `transition-[width] duration-700 ease-out`

## Edge Cases
- long text: cue 文案长度按当前卡片宽度换行，避免挤压右侧圆形计时器
- slow network: N/A（本轮全部是本地资源）
- empty datasets: cue 关闭时不显示 cue card；自定义壁纸为空时显示空态说明
- permission denied: 若系统文件选择器未返回图片，保持原设置不变
- offline: 全部资源本地化，不依赖网络

## Acceptance Criteria (UI)
- 偏好页可完成背景主题、自定义壁纸、随机交互语和开始音配置
- break prompt 默认态展示 cue card、线性倒计时和圆形倒计时
- 自定义壁纸能在设置页显示预览，并在后续 break 中成为背景
- 静音或音量为 0 时，不显示误导性的音效状态

## Open Questions
- 后续是否需要再补一个“用户自定义 cue 列表”的设置页编辑器，以继续对齐旧版 ideas 能力
