# Task-ID: TID-20260403-tauri-usable-core

## Goals
- 让 Tauri 前台变成真正可用的工具设置页，而不是迁移展示页。
- 用一个独立、安静的 break prompt 承接休息动作，不用复杂视觉或过多信息打扰用户。

## Non-Goals
- 不做营销首页、迁移进度、卡片墙或复杂 dashboard。
- 不在本轮引入统计图表、成就系统或过度品牌化视觉。

## Screens & User Flows
- Primary flow:
  - 用户打开主设置窗口
  - 顶部看到当前状态、下一次休息、Pause/Focus/Resume 动作
  - 中部在 Rhythm / Behavior / Signals / Apps 四组里调整设置
  - 底部通过 Revert / Defaults / Save 提交更改
- Fallback / secondary flow:
  - 后台触发休息时弹出 break prompt
  - 用户在小窗中执行 Done / Postpone / Skip
  - 浏览器 preview 用同一套页面结构做静态预览
- User-visible boundary:
  - 设置页是唯一长期工作台
  - break prompt 是唯一中断式界面
- Entrypoints / handoff cues:
  - 主窗口
  - tray
  - 全局快捷键
  - 自动弹出的 break prompt

## Component Tree
- `App`
  - `Topbar`
  - `Summary`
  - `ActionRow`
  - `MainColumn`
    - `Section(Rhythm)`
    - `Section(Behavior)`
    - `Section(Signals)`
    - `Section(Apps)`
  - `SideColumn`
    - `Section(Status)`
    - `Section(Last action)`
  - `FooterActions`
- `BreakMode`
  - `BreakSheet`
    - eyebrow
    - title
    - detail
    - timer
    - action buttons

## Interaction States
- hover: 按钮只做轻微边框/底色变化
- active: 使用原生按钮按压反馈
- focus: 依赖原生表单焦点 ring
- disabled: Save / Revert 在未修改或 busy 时禁用
- loading: 初始仅显示 `Loading Pauza…`
- empty: 无 active break 时显示默认设置页和下一次休息状态
- error: 顶部以下出现 error banner
- skeleton: N/A
- optimistic (if applicable): preview 模式下直接本地模拟状态变化

## Responsive Rules
- 大屏：主内容两栏，右侧状态栏固定为窄列
- 窄屏：summary、field、layout 自动折成单列
- break prompt 始终保持小尺寸单卡片结构

## Accessibility (a11y)
- keyboard navigation: 依赖原生表单与按钮顺序
- focus order: top actions -> main sections -> side sections -> footer actions
- aria labels: 优先使用原生 label/checkbox/select/textarea 语义
- contrast: 深墨文字配暖白背景，状态色保持足够对比
- reduced motion: 当前无强动画

## Design Tokens / Tailwind Mapping
- typography:
  - 标题：`SF Pro Display`
  - 正文：`SF Pro Text`
- spacing:
  - 主窗 `14px / 24px / 28px` 为主
- color usage:
  - 背景：暖白 `#f4f0ea`
  - 主按钮：深绿 `#1f4a42`
  - 文本：墨色 `#181512`
- key classes:
  - `.topbar`
  - `.summary`
  - `.section`
  - `.field`
  - `.button`
  - `.break-sheet`

## Micro-animations (optional)
- 无额外动效，仅保留轻微 hover 反馈

## Edge Cases
- long text: textarea 支持多行输入 app exclusion commands
- slow network: N/A，本轮无远程请求
- empty datasets: preview / idle 状态下仍能渲染默认内容
- permission denied: 宿主命令失败时通过 error banner 呈现
- offline: 不影响本地功能

## Acceptance Criteria (UI)
- 主窗口不再出现迁移队列、legacy source、能力卡等展示型内容
- 结构收敛为少量状态 + 四组设置 + 底部保存动作
- break prompt 可独立渲染且动作语义清晰
- 浏览器 preview 与 Tauri runtime 使用同一套 UI 骨架

## Open Questions
- 下一轮是否将右侧状态栏扩成 today 统计卡片
- break prompt 是否需要加入更细的 “not now” 原因语义
