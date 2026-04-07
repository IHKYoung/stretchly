# Task-ID: TID-20260403-settings-fixed-split

## Goals
- 把主设置页恢复成稳定的桌面 split view，而不是全局三栏压缩主体。
- 左侧 sidebar 占主窗口约四分之一，右侧主体占其余约四分之三。
- 保存/状态动作继续明显，但应并入主体而不是再占第三列。
- 收紧当前过度圆角和冗长介绍文案，让界面回到工具化表达。

## Non-Goals
- 不改 break prompt。
- 不新增或删除任何设置项。
- 不重做分类模型、文案体系或 overview 数据结构。

## Screens & User Flows
- Primary flow:
  - 打开设置页
  - 在左侧固定 sidebar 中选择分类
  - 在右侧主体顶部看到 save/status dock
  - 在主体中部与下方查看当前分类详情并执行保存
- Fallback / secondary flow:
  - 浏览器 preview 在小于 `1280px` 的窄视口下允许退回堆叠，仅作为开发旁证
- User-visible boundary:
  - `main` 主设置页
- Entrypoints / handoff cues:
  - 左侧分类按钮组
  - 主体顶部保存 dock
  - 当前分类 hero + settings section cards

## Component Tree
- `main settings shell`
- `sidebar card (1/4)`
- `main column (3/4)`
- `save/status dock`
- `category hero`
- `section shells`

## Interaction States
- hover:
  - sidebar 分类按钮与主操作按钮延续当前 hover 高亮
- active:
  - 当前分类在 sidebar 中使用深色实底高亮
- focus:
  - 继续沿用 shadcn/tailwind ring 语义
- disabled:
  - `dirty=false` 时保存与撤销按钮禁用
- loading:
  - 页面初始使用现有 loading 文案
- empty:
  - N/A
- error:
  - 顶部 error banner 保留
- skeleton:
  - N/A
- optimistic (if applicable):
  - N/A

## Responsive Rules
- Tauri 主窗口默认 `1440x810`，最小 `1280x720`，桌面交付态始终保持 split view。
- split view 使用约 `1fr : 3fr` 的外层 grid，左侧接近 `25%`，右侧接近 `75%`。
- save/status dock 不再作为全局第三列；仅在主体内部布局。
- 浏览器 preview 窄视口可以继续堆叠，但不作为桌面交付态验收标准。

## Accessibility (a11y)
- keyboard navigation:
  - sidebar 继续使用 button 语义，可 Tab 聚焦并切换当前分类
- focus order:
  - sidebar -> 主体顶部保存 dock -> 当前分类内容
- aria labels:
  - 沿用现有 Switch / SegmentedControl / Select 的无障碍标签
- contrast:
  - 深色激活态与浅色主体保持高对比
- reduced motion:
  - 不新增额外动画，只保留已有轻微 surface-in / orb-float

## Design Tokens / Tailwind Mapping
- typography:
  - 继续沿用 `SF Pro Text / Display` 与当前标题字号层级，但 hero 回收为标题型头部
- spacing:
  - 外层改为 `gap-6`，sidebar 与主体直接形成稳定双栏
- color usage:
  - 延续当前浅蓝玻璃化背景、深蓝激活态与绿色 synced badge
- key classes:
  - `xl:grid-cols-[minmax(280px,1fr)_minmax(0,3fr)]`
  - `xl:min-h-[calc(100vh-2rem)]`
  - `xl:grid-cols-[minmax(0,1fr)_300px]`
  - `rounded-[20px]` / `rounded-[16px]` / `rounded-[14px]` / `rounded-[12px]`

## Micro-animations (optional)
- 仅保留已有 `animate-surface-in`，不新增复杂 motion。

## Edge Cases
- long text:
  - 分类说明与状态文案允许自然换行
- slow network:
  - N/A（本页主要读本地 host snapshot）
- empty datasets:
  - N/A
- permission denied:
  - 若 host 调用失败，继续显示顶部 error banner
- offline:
  - 无外网依赖

## Acceptance Criteria (UI)
- 主设置页在桌面交付态下呈现稳定的 `1/4 + 3/4` 双栏结构。
- 用户不会再看到全局第三列单独压缩主体内容。
- 保存动作、状态摘要与当前分类详情都能在右侧主体区域内完成闭环。
- 顶部 hero / section / sidebar 不再铺大段介绍性文案。
- `1440x810` 视口截图能直观看到 split view 生效且圆角已明显收紧。

## Open Questions
- 当前无阻塞性 UI open question；若后续仍觉得顶部 save/status dock 信息过多，再单独做内容减法任务。
