# Task-ID: TID-20260403-settings-sidebar-layout

## Goals
- 把 Tauri 设置页从 overview hero + stacked sections 收敛成更像原生偏好页的侧边栏分类布局。
- 保持当前已存在的 Tauri 设置项全部可见、可编辑、可保存，不自行新增新设置。
- 让分类切换比当前长页面更清楚，降低“需要滚很多才能找到设置”的负担。

## Non-Goals
- 不改 break prompt 的结构和视觉语言。
- 不恢复旧 Electron 页面里的全部主题/声音等宿主尚未支持的字段。
- 不新增 dashboard、统计、历史或其他展示型模块。

## Screens & User Flows
- Primary flow:
  - 打开主设置页
  - 在左侧边栏看到分类列表
  - 选择分类后，在主内容区查看该分类下的设置分组和字段
  - 在右侧保存/状态区完成保存、撤销、恢复默认值
- Fallback / secondary flow:
  - 调整窗口尺寸到窄屏后，侧边栏和内容区按单列堆叠，仍能浏览和编辑各分类
- User-visible boundary:
  - `main` 主设置页
  - `?window=break` 保持现状，仅作为不回归的旁路流
- Entrypoints / handoff cues:
  - 左侧分类导航
  - 主内容区分类标题和说明
  - 右侧 save rail 的 dirty/synced 状态

## Category Model
- `概览`：运行状态、快捷动作和摘要信息
- `节奏`：microbreak / long break 的启用、间隔、时长
- `提醒与延后`：提前提醒与 postpone 相关字段
- `打断与显示`：break prompt style、屏幕位置、当前时间、strict/manual finish、tray strict visibility
- `智能暂停`：natural breaks、DND、app exclusions
- `通用与快捷键`：语言、开机启动、全局快捷键

## Interaction States
- active:
  - 当前分类在侧边栏中高亮
- hover:
  - 侧边栏分类卡片和设置行有轻量高亮
- focus:
  - 保持当前 shadcn/tailwind ring 语义
- disabled:
  - `dirty=false` 时保存按钮禁用
  - `showBreaksOnAllScreens=true` 时目标屏幕选择器隐藏
- error:
  - 顶部 error banner 保留
- loading:
  - 页面初始继续使用简单 loading 文案

## Responsive Rules
- `xl` 以上：左侧 sidebar、中间内容区、右侧 save/status rail 三栏布局
- `xl` 以下：save/status rail 下沉；`lg` 以下 sidebar 与内容区堆叠
- 分类导航在窄屏下保持一行可读，不做复杂 drawer

## Accessibility (a11y)
- 使用原生 button 语义承载侧边栏分类切换
- 当前分类通过视觉高亮和辅助标签共同表达
- 保持现有 form controls 的键盘导航能力

## Acceptance Criteria (UI)
- 左侧边栏可以明确切换设置类别，且当前类别状态清晰。
- 当前 `PauzaSettings` 中已暴露的设置项都仍可在某个分类中找到。
- 保存、撤销、默认值动作仍保持明显 handoff，不因布局变化而丢失。
- `?window=break` 入口不出现视觉或交互回归。
