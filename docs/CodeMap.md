# CodeMap

> 维护规则：该文件描述当前仓库文件树与职责，确保与实际目录一致。

## Top Level
- `.github/`：CI 工作流、GitHub 模板与仓库级 Copilot/agent 辅助文档。
- `.githooks/`：Codex 工作流校验与提交审计 hooks。
- `README.md`：根目录快速入口，汇总当前有效的运行、测试与打包命令。
- `app/`：归档中的旧 Electron 壳；当前默认运行、构建、测试链路都不再依赖该目录。
- `apps/desktop/`：当前唯一有效的桌面端主实现，包含前台、Rust host 与 locale 真源。
- `apps/site/`：Pauza 单页官网与下载跳转目录；纯静态实现，后续适合作为 Vercel 部署根目录。
- `build/`：打包所需图标与安装器资源。
- `docs/`：变更日志、项目理解文档、任务 specs、plans 与 logs。
- `graphics/`：应用图标源文件与生成脚本；其中 `app-icon.svg` 面向 Dock / bundle，`tray-icon.svg` 面向 macOS tray，小尺寸几何单独维护，`generate_icon_assets.py` 负责把两者展开为打包资源。
- `scripts/`：工作流脚本、校验器与任务脚手架。
- `test/`：Vitest 测试用例与测试辅助资源。
- `package.json`：根级 npm scripts；当前只保留 `dev/build/typecheck` 短入口，以及 `desktop:*`、`site:*`、`test:*` 命名空间命令。
- `vitest.config.ts`：Vitest 运行与覆盖率配置。

## app/（archived）
- 旧 Electron 主进程、旧窗口页面、旧 preload/renderer、历史资源与旧 i18n 资产。
- 该目录当前只保留为历史参考，不应再作为默认运行、构建、测试或 locale 生成的输入。

## apps/desktop/
- `apps/desktop/package.json`：Tauri desktop 子应用的依赖与开发命令。
- `apps/desktop/components.json`：shadcn 风格组件基座声明，绑定 `src/styles.css`、`src/components/ui/*` 与别名约定。
- `apps/desktop/src/App.tsx`：同构的 Tauri 主设置页与 break prompt；根据 query 参数切换偏好页或独立 break prompt，浏览器 preview 也支持 `?preview=paused|focus` 取证。
- `apps/desktop/src/assets/audio/`：Tauri 前台 break start / end sound 资源，当前作为 `apps/desktop` 自带静态资源维护。
- `apps/desktop/src/components/ui/`：Button/Card/Switch/Input/Textarea/Accordion/Select/Badge/SegmentedControl 等前台 primitives。
- `apps/desktop/src/i18n.ts`：React 前端的轻量 i18n lookup / 插值层，消费 `registry.generated.json` 并负责语言 fallback 与时长格式化。
- `apps/desktop/src/lib/settings-controls.ts`：节奏页 preset 常量与数字草稿提交 helper，负责“5 个候选项 + blur/Enter 提交”的输入交互。
- `apps/desktop/src/lib/break-ideas.ts`：break prompt 的真源选择 helper，直接从 `miniBreakIdeas` / `longBreakIdeas` 读取交互语，并用稳定混洗索引避免固定周期总是命中同一条。
- `apps/desktop/src/lib/break-prompt.ts`：break prompt 的预设背景、提示音映射和自定义壁纸压缩 helper。
- `apps/desktop/src/lib/utils.ts`：`cn()` 合并工具，供 shadcn/Tailwind primitives 复用。
- `apps/desktop/src/locales/messages/`：桌面端自维护的每语言基础消息文件；break prompt 专属提示语也统一收口在这里的 `ui.breakCopy.*`。
- `apps/desktop/src/locales/config/`：桌面端自维护的每语言配置文件（code / label / fallback / desktopReady）。
- `apps/desktop/src/locales/registry.generated.json`：由 `scripts/sync_desktop_locales.py` 基于桌面端 locale 目录生成的前后端共享 locale registry。
- `apps/desktop/src/styles.css`：Tailwind CSS v4 token、base layer 与 Tauri 设置页 / break prompt 的玻璃化背景和动效。
- `apps/desktop/public/pauza.svg`：前端 branding 资源。

## apps/desktop/src-tauri/
- `apps/desktop/src-tauri/tauri.conf.json`：Tauri 2 应用配置、窗口尺寸与 bundle icon 列表。
- `apps/desktop/src-tauri/icons/`：当前正式 bundle / tray 图标输出目录；其中 `icon-alt-nw45.{svg,png}` 是从网页 favicon 演化出的透明底候选图标素材，不参与当前默认打包链路。
- `apps/desktop/src-tauri/src/i18n.rs`：Rust host 侧共享 locale lookup 与插值层，消费 `registry.generated.json` 并与前端使用同一语言 fallback 规则。
- `apps/desktop/src-tauri/src/lib.rs`：插件装配、状态初始化、engine 启动与命令注册入口。
- `apps/desktop/src-tauri/src/commands.rs`：设置、pause/focus、break actions、autostart 与主窗口命令边界。
- `apps/desktop/src-tauri/src/engine.rs`：后台 tick 循环，驱动调度状态机、通知与 break window。
- `apps/desktop/src-tauri/src/platform.rs`：idle / DND / app exclusion 的跨平台轻量探测层；`idle_ms` 已与 `natural_breaks` 开关解耦，始终可供智能提醒使用。
- `apps/desktop/src-tauri/src/shell.rs`：tray 菜单、托盘点击、全局快捷键与 break/main window 生命周期；macOS 下还负责定向 patch Pauza 自己的 `NSStatusItem` tray image，并在开发态兜底 Dock icon。
- `apps/desktop/src-tauri/src/state.rs`：`PauzaSettings`、`RuntimeState`、`DesktopSnapshot` 的运行时真源，现已覆盖通知、postpone、`reminder_mode`、manual finish、break surface、自定义壁纸 / cue / start/end sound、shortcut 配置，并通过 i18n 层输出本地化状态文案。

## apps/site/
- `apps/site/index.html`：官网单页首页；当前采用纯白纸面背景与极简打字机舞台，只保留一个下载按钮。
- `apps/site/favicon.svg`：官网与下载页共用的站点图标；当前已改为透明底，开口朝向左上，作为桌面端候选图标的几何来源。
- `apps/site/styles.css`：官网视觉层，负责纸面/网格质感、中央舞台与下载按钮样式。
- `apps/site/script.js`：首页打字机脚本，负责逐字输入、停留、退格和下一条文案切换。
- `apps/site/copy.js`：官网循环展示的提醒文案真源，当前精选自桌面端内嵌提醒文案。
- `apps/site/fonts/LXGWWenKaiScreen.ttf`：官网自带的 `LXGW WenKai Screen` 字体资源，用于实现“落霞孤鹜 / 霞鹜”风格。
- `apps/site/download/targets.js`：网站下载目标地址真源；首页 CTA 只连到站内稳定路由，真实 URL 统一在这里维护。
- `apps/site/download/redirect.js`：下载跳转页的统一逻辑，负责 loading、fallback 和自动跳转。
- `apps/site/download/styles.css`：下载跳转页的共享视觉样式。
- `apps/site/download/*/index.html`：按平台拆分的稳定下载路由页面。

## test/
- `test/translations.js`：桌面端 `apps/desktop/src/locales/messages/*.json` 的多语言资源一致性测试。
- `test/desktopBreakCopySource.js`：校验 break prompt 文案只从 desktop `messages/*.json` 读取。

## docs/
- `docs/RepositoryGuidelines.md`：仓库结构、命令与开发约束索引。
- `docs/CodeMap.md`：当前文件树职责地图。
- `docs/Architecture.md`：运行时架构摘要。
- `docs/RootMetadataArchive.md`：原根目录 README / LICENSE / 社区治理文件 / Linux 发布元数据的提炼归档。
- `docs/ReminderScheduling.md`：下一轮提醒状态机与调度逻辑设计文档，定义智能提醒、强制提醒、严格模式与自然休息的职责边界。
- `docs/UI.md`：窗口与交互结构摘要。
- `docs/SettingsInventory.md`：当前设置页、Tauri 真源隐藏设置、旧版候选设置与运行时动作的边界清单。
- `docs/CHANGELOG.md`：当前中文变更日志入口。
- `docs/plans/`、`docs/logs/`、`docs/specs/`：Codex 任务闭环文档。
