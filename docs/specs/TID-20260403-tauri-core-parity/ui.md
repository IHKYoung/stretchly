# Task-ID: TID-20260403-tauri-core-parity

## Goals
- 把 Tauri 前台从“迁移展示页”收敛为真正的工具型设置界面。
- 保持默认视觉简洁、克制、低干扰，让设置界面比功能列表更像系统工具。
- break prompt 保持单任务、低认知负担，兼容普通倒计时与 manual-awaiting 两种状态。

## Non-Goals
- 不做 dashboard、数据大屏、营销式展示卡片。
- 不做欢迎页或引导式叙事。
- 不在主界面塞入 today stats、图表或多层导航。

## Screens & User Flows
- Primary flow:
  - 打开主设置窗口。
  - 查看状态条与下一次休息状态。
  - 在 General / Schedule / Signals 修改关键设置。
  - 需要时展开 Advanced 修改通知、postpone、surface、strict、app exclusions、shortcuts。
  - 点击 Save 将配置写回宿主。
- Fallback / secondary flow:
  - 不打开主窗口，直接通过 tray 菜单或全局快捷键 pause/focus/resume/skip/reset。
  - 休息触发后进入独立 break prompt，执行 done / later / skip。
- User-visible boundary:
  - 主设置页
  - toolbar
  - Advanced 折叠区
  - break prompt
  - tray 菜单
- Entrypoints / handoff cues:
  - 主窗口标题、状态文案、toolbar 三个动作按钮
  - break prompt 的单主按钮与次操作按钮
  - tray submenu

## Component Tree
- Main window
  - `header`
  - `toolbar`
  - `panel`
    - `Group(General)`
    - `Group(Schedule)`
    - `Group(Signals)`
    - `details.advanced`
- Break window
  - `prompt`
    - `prompt__eyebrow`
    - title
    - detail
    - `prompt__time`
    - `prompt__actions`

## Interaction States
- hover:
  - button、checkbox、select、text field 保持系统工具式轻反馈
- active:
  - toolbar / prompt actions 触发时进入 `busyAction`，禁用重复提交
- focus:
  - 表单控件与 break 主 CTA 可键盘聚焦
- disabled:
  - runtime 命令执行中按钮 disabled
  - 不允许 postpone/skip 时隐藏次按钮
- loading:
  - snapshot 未加载时显示 `app--loading`
- empty:
  - 无当前 break 时主设置页显示基础状态，不展示 prompt
- error:
  - `error-banner` 暴露加载/命令错误
- skeleton:
  - N/A
- optimistic (if applicable):
  - 浏览器 preview 用 `previewTransform` 做轻量乐观更新

## Responsive Rules
- 主设置页保持单栏、受控宽度，适合桌面工具而非营销页面。
- Advanced 内容纵向堆叠，避免在窄窗口下形成复杂栅格。
- break prompt 在独立窗口内保持单卡片布局，不引入多层信息区域。

## Accessibility (a11y)
- keyboard navigation:
  - 所有输入控件、toolbar 按钮、details summary 可用键盘访问
- focus order:
  - header 后进入 toolbar，再进入设置分组，再进入 Advanced
- aria labels:
  - 依赖原生表单标签与文本说明
- contrast:
  - 深色正文 + 浅底界面，保证工具型可读性
- reduced motion:
  - 当前无依赖复杂动画

## Design Tokens / Tailwind Mapping
- typography:
  - 系统工具风格，优先清晰层级而不是品牌展示
- spacing:
  - 以大块留白和分组间距替代花哨卡片堆叠
- color usage:
  - 暖白底、深墨文字、克制边界与弱强调按钮
- key classes:
  - `app`
  - `header`
  - `toolbar`
  - `panel`
  - `group`
  - `row`
  - `advanced`
  - `prompt`
  - `button`

## Micro-animations (optional)
- 不强调动效；只保留浏览器/系统默认交互反馈。

## Edge Cases
- long text:
  - locale 文案增长后仍应以单栏和换行为主
- slow network:
  - N/A，主界面不依赖远程接口
- empty datasets:
  - 无当前 break、无 app exclusion 时保持干净空态
- permission denied:
  - autostart / shortcut /系统权限失败时通过 `error-banner` 或 `lastAction` 呈现
- offline:
  - N/A，核心功能本地运行

## Acceptance Criteria (UI)
- 主窗口不再呈现展示型 dashboard，且默认中文。
- 用户无需滚过大量花哨区域就能完成主要设置。
- Advanced 默认折叠，保证“简单设置保持简单”。
- break prompt 的主 CTA 在普通倒计时与 manual-awaiting 两种状态下都明确。

## Open Questions
- 下一轮是否继续压缩 Advanced 的字段数量，把 shortcut/postpone 分成二级弹层，而不是继续堆在同一页里。
