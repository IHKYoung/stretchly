# UI

## 窗口与入口
- `welcome.html`：首次启动引导，现已采用更现代的双栏卡片布局，承载语言选择与主要 CTA。
- `preferences.html`：主设置界面，现以顶部卡片导航 + 内容面板组织休息时长、通知、干预风格、主题与关于信息。
- `microbreak.html`：短休息窗口，现为紧凑的现代卡片，Gentle 模式下默认以更轻的角落卡片显示。
- `break.html`：长休息窗口，现为居中的现代卡片，支持标题、正文、倒计时与柔和动作按钮。
- `process.html`：隐藏后台页，用于播放声音、版本检查和系统通知代理。
- `contributor-preferences.html` 与远程 auth/sync 窗口：Contributor 扩展功能。
- `apps/desktop/src/App.tsx`：新的 Tauri 前台，已成为默认入口；当前设置页已收敛为更紧凑的工具窗口，主窗口下是窄 sidebar categories + 单一主面板，`?window=break` 下是独立 break prompt。

## 前端实现模式
- 每个窗口基本都采用 `HTML + preload + renderer` 三件套。
- preload 只负责暴露受控能力，真正的窗口交互和 DOM 操作都在 renderer 内完成。
- 没有组件框架和全局前端状态管理；状态变更以 IPC、DOM 查询和当前设置快照为主。

## 主要用户路径
1. 首次启动进入 Welcome，看到现代化欢迎页，设置语言或直接进入 Preferences。
2. 常规运行时主要通过托盘菜单操作暂停、恢复、进入 Focus session、跳转到下一个休息和打开 Preferences。
3. 休息触发后显示 Mini/Long Break 卡片；用户可根据 `breakPromptStyle` 的强度、严格模式、推迟时机和快捷键决定推迟、跳过或完成。
4. Contributor 用户额外看到 Contributor Preferences 与 Sync Preferences 入口。
5. 新 Tauri 壳中，用户可通过偏好页、tray 和全局快捷键直接管理节奏、pause/focus、skip/reset 和 launch on login；休息开始时会弹出独立 break prompt，可执行 Done / Postpone / Skip，并在 manual finish 开启后进入 `manualAwaiting` 状态；tray 在 macOS 下已改为用原生 `Submenu` 作为右键菜单根节点，并移除后台秒级无条件菜单重建，以避免菜单刚展开就一闪即逝，前后端文案统一走共享 locale 资源。

## Tauri 设置页结构
- 主设置页现为固定双栏工具窗口：左侧导航区约 `168px`，右侧为单一主面板。
- 左侧导航只保留 4 个分类：`节奏`、`提醒与打断`、`智能暂停`、`通用`。
- `节奏`：microbreak、long break 的开关，以及顶部四个核心时间字段的 preset 芯片选择。
- `提醒与打断`：pre-break notification、postpone、strict mode、break prompt style。
- `提醒与打断` 现重新包含 `显示方式：窗口 / 全屏`，用于决定休息提示是以窗口还是铺满目标显示器出现。
- `智能暂停`：natural breaks、DND、app exclusions。
- `通用`：开机自启动、语言。
- overview、quick actions、快捷键编辑、运行时动作与额外第三列 save rail 已从设置页主结构移除。
- 页面视觉已继续收紧为偏桌面工具的平面风格：暖灰背景、纯白主面板、浅边框、小圆角、几乎无装饰背景，并移除侧栏状态区与重复标题。
- 基础控件（button / select / segmented control / number input）统一压平为更小的圆角和更紧的高度，不再强调悬浮卡片感。
- 设置页现已改为自动保存，顶部只在保存中或刚发生变更时显示轻量状态，不再保留显式保存按钮。
- `节奏` 分类已进一步压成两张紧凑 preset 卡：微休息与长休息顶部直接用芯片选择间隔/频率与时长，下方“提前提醒 / 延后”继续保留 stepper 作为低频细调。
- Tauri 主窗口默认尺寸调整为 `1040x585`，最小尺寸为 `960x540`，让设置页更接近“一个很小的偏好窗口”。

## Tauri Break Prompt
- break 页面现在直接填满宿主窗口本身，不再在 break window 里再套一张固定宽度的小卡片。
- 中心为环形倒计时，顶部展示当前时间和 break kind，底部保持清晰的主副 CTA。
- 主 CTA 在普通态显示 `Done`，在 `manualAwaiting` 态显示 `Resume work`。
- 次操作按钮按能力显示：允许 postpone 才显示 `Later`，允许 skip 才显示 `Skip`。
- `currentTimeInBreaks` 打开时，在顶部 eyebrow 显示当前时间。

## UI 个性化开发建议
- 改欢迎页或偏好页文案/布局时，优先从 `app/preferences-renderer.js`、`app/welcome-renderer.js` 和对应 HTML/CSS 入手。
- 改休息窗口视觉表现时，优先检查 `app/break-renderer.js`、`app/microbreak-renderer.js` 以及 `app/css/break.css`。
- 调整 break 强度时，同时审查 `app/utils/defaultSettings.js`、`app/utils/breakWindowProfile.js` 与 `app/main.js` 的窗口创建策略。
- 改系统图标、托盘提示或菜单结构时，入口仍在主进程 `app/main.js`，不是 renderer。
- 改 preload 接口时，同时审查 `app/utils/context-bridge-exposers.js` 与对应窗口脚本，避免 IPC 名称漂移。
- 设计新产品壳时，优先在 `apps/desktop/src/App.tsx`、`apps/desktop/src/styles.css`、`apps/desktop/src/components/ui/*`、`apps/desktop/src/locales/*.json` 与 `apps/desktop/src-tauri/src/{state,shell,commands}.rs` 上演进，不要继续把 Electron settings 页面或迁移展示页当作最终产品主工作台。
