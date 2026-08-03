# Task-ID: TID-20260803-current-worktree-handover

## Goals
- 让用户在智能提醒已到点但等待自然空档时，从 tray 看到真实剩余等待时间。
- 让节奏详情页以自然完整句式表达“多久一次、持续多久、每几个提醒周期出现完整休息”。
- 建立可复用的设置页字体层级，同时保持霞鹜文楷和既有整体布局。
- 保持官网视觉不变，只更新下载链接解析行为。

## Non-Goals
- 不改变设置首页结构、profile 数值、break 页面、tray 菜单动作或官网视觉。
- 不新增 loading spinner、进度条、保存按钮或下载错误 UI。

## Screens & User Flows
- Primary flow: 用户选择 preset 或点击“自定义”输入数值；完整休息区域即时显示折算分钟和周期语义。smart reminder 到点后，tray title 显示 backend wait countdown，空档出现或预算耗尽后进入 break。
- Fallback / secondary flow: custom 输入留空/Escape 恢复当前值；blocker/forced 不展示 waiting countdown；官网 API 失败时继续导航 pinned v0.1.4。
- User-visible boundary: 设置窗口节奏详情/文字层级、macOS tray title、官网右上角下载按钮 href。
- Entrypoints / handoff cues: 节奏 preset chips 与“自定义”输入、tray icon/title、`.download-button` 原生 anchor。

## Component Tree
- Settings: `App -> SchedulePresetCard -> PresetChipRow -> PresetChipGroup`，以及 `SectionLabel / SettingsRow / SettingsLinkRow` 字体语义类。
- Tray: `DesktopSnapshot -> tray_detail_mode/tray_title -> Tauri tray`。
- Site: `a.download-button -> targets.js -> script.js`。

## Interaction States
- hover: custom placeholder、preset、设置 link row 延续既有 hover；官网按钮样式不变。
- active: preset 维持高对比选中；custom 编辑时显示输入框边框；latest href 成功后透明替换。
- focus: custom 输入保留 ring，聚焦 preset 值时清空草稿供输入；原生 anchor/controls 键盘行为不变。
- disabled: 常规 tray 倒计时关闭不隐藏 smart waiting 临时反馈；其它开关/控件 disabled 行为不变。
- loading: site latest 请求期间 pinned href 可点击；设置 snapshot loading 行为不变。
- empty: custom 空 draft 回退现值；site 无匹配 DMG 保留 pinned。
- error: site 请求失败只记 console warning；设置 load/save error 保持现有显性错误。
- skeleton: 不新增。
- optimistic (if applicable): site 初始 pinned 是真实可用 fallback，不伪装为 latest；设置不做 optimistic schema 变更。

## Responsive Rules
- 设置控件保留现有固定窗口布局和 flex wrap；control label 适度加宽以容纳完整句式。
- 官网按钮定位和移动端规则完全不变。

## Accessibility (a11y)
- keyboard navigation: preset button、custom input、原生 anchor 均可键盘操作；Enter/blur 提交、Escape 恢复。
- focus order: 不新增中转层，顺序保持 DOM 结构。
- aria labels: custom input 使用本地化 `ariaLabel: customLabel`；tray 无新增可交互控件；官网保留下载 aria-label。
- contrast: active preset、placeholder、正文/详情继续使用既有 foreground/muted tokens。
- reduced motion: 本次设置/tray/site 变更不依赖动画，原有 reduced-motion 不变。

## Design Tokens / Tailwind Mapping
- typography: `settings-page-title / section-title / group-title / row-title / link-title / summary / detail / caption / control-label` 明确字号、字重和行高。
- spacing: SettingsRow/LinkRow 最小高度略增，节奏 label 宽度增加，完整休息 cadence hint 对齐输入区域。
- color usage: 继续使用 `text-foreground`、`text-muted-foreground`、现有 border/background tokens。
- key classes: 所有设置文字继续继承全局 `--app-font-sans: "LXGW WenKai Screen"`；break surface 的 `.type-break` 不变。

## Micro-animations (optional)
- 不新增动画。

## Edge Cases
- long text: 三语 cadence hint 可换行；设置 row 高度增加避免拥挤。
- slow network: site 请求完成前 pinned 始终可点击。
- empty datasets: site empty assets 走 pinned；locale generator 保证主 key 存在。
- permission denied: GitHub API 限流/拒绝走 pinned；desktop 无新权限。
- offline: 已加载页面仍显示 pinned URL，但不伪装离线下载；desktop tray 使用本地 runtime。

## Acceptance Criteria (UI)
- preset 命中时显示“自定义”入口而不是重复数字；进入/退出编辑不意外改值。
- 完整休息说明明确最后一个提醒周期为完整休息、其余为微休息，并展示折算分钟。
- 文字层级在源码和 production build 中存在，字体家族不分裂。
- waiting tray 只展示 backend 事实；官网按钮视觉不变且成功/失败 href 都可用。

## Open Questions
- 本轮不要求嵌套 site repo 清洁，也不新增真实机 UI 设计改动；视觉截图若环境不可用将作为显性证据缺口保留。
