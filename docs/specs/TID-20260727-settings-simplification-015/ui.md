# Task-ID: TID-20260727-settings-simplification-015

> 若本任务不涉及 UI/交互，可在此注明 `N/A` 并保持该文件存在（流程门禁要求）。

## Goals
- 默认页只承载高频决策，并提供清晰、直接的细调入口。
- 统一“微休息 / 完整休息”术语，让节奏摘要直接展示实际时间。

## Non-Goals
- 不重做 break prompt 视觉、不新增设置控件、不恢复快捷键编辑或 dashboard。

## Screens & User Flows
- Primary flow: 设置首页选择三档节奏或直接进入自定义节奏、提醒、外观、自动适应、系统页面；返回直接到首页。
- Fallback / secondary flow: 非标准参数显示“自定义”并保留原值；保存或宿主错误继续显示现有错误条。
- User-visible boundary: Tauri 主设置窗口与完整休息 break prompt 文案轮换。
- Entrypoints / handoff cues: 首页 link row + chevron、主题页左上返回箭头、节奏 profile radiogroup。

## Component Tree
- `App -> overview -> RhythmProfilePicker / SettingsCard / SettingsLinkRow`
- `App -> rhythm|reminders|appearance|automation|system`
- `BreakWindow -> resolveBreakPromptSequence -> typeField -> hold -> switch gap`

## Interaction States
- hover: link row 与 icon button 使用既有轻量背景反馈。
- active: profile 以白底和阴影标记选中；主题页标题与 route 一致。
- focus: radiogroup、link row、返回按钮保留可见 focus ring。
- disabled: 复用既有 switch/button 禁用态；本轮无新增禁用逻辑。
- loading: snapshot 未加载时显示既有 loading 文案。
- empty: 自定义壁纸/应用搜索继续使用既有空态；本轮不改变。
- error: 宿主加载或 autosave 错误继续在标题下显式显示。
- skeleton: 不适用，设置 snapshot 使用文字 loading。
- optimistic (if applicable): profile patch 先更新本地 form，再走既有串行 autosave。

## Responsive Rules
- 维持 `960x640` 默认、`800x600` 最小窗口；内容区可纵向滚动，固定 header 不重排。
- link row 摘要截断而不挤压 chevron，三档 profile 使用稳定三列网格。

## Accessibility (a11y)
- keyboard navigation: 所有入口和 profile 均为原生 button；返回按钮可键盘触发。
- focus order: header 返回 -> profile -> 主题入口 -> 常用设置 -> 更多设置。
- aria labels: profile 容器使用 `radiogroup` + label，返回按钮和 switches 保留 aria-label。
- contrast: 复用现有 foreground/muted/ring tokens。
- reduced motion: break prompt 在 reduced-motion 下完整静态显示，不轮换。

## Design Tokens / Tailwind Mapping
- typography: compact settings 使用 11-14px 既有层级。
- spacing: header `h-14`，卡片 row `42-52px`，页面 `space-y-3`。
- color usage: 暖灰 background、白色 settings card、低对比边框。
- key classes: `grid-cols-3` profile、`focus-visible:ring-2`、`overflow-y-auto`。

## Micro-animations (optional)
- 完整休息提示语 title/body 完整打出后停留 `60s`，清空后经过 `520ms` 再打下一条。

## Edge Cases
- long text: prompt 按标点分行；设置摘要 truncate；完整 locale 名仍由 select 承载。
- slow network: 桌面设置无远端依赖；宿主 snapshot 加载失败显式报错。
- empty datasets: break ideas 无内容时回退 `ui.breakCopy.defaultPrompt.*`。
- permission denied: 与本轮无关，沿用宿主显式错误。
- offline: 功能不依赖网络。

## Acceptance Criteria (UI)
- 五个主题均从首页直达，返回不经过中转页；自定义节奏入口明确可见。
- 简中/繁中 long break 显示“完整休息”，英文显示 `Full break`。
- 完整休息提示语只在完整呈现后开始 60 秒 hold；微休息不轮换。

## Open Questions
- 自动化浏览器后端不可用，截图与真实 60 秒观察留作发布前人工验收。
