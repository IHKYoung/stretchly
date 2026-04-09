# CodeMap

> 维护规则：该文件描述当前仓库文件树与职责，确保与实际目录一致。

## Top Level
- `.github/`：CI 工作流、GitHub 模板与仓库级 Copilot/agent 辅助文档。
- `.githooks/`：Codex 工作流校验与提交审计 hooks。
- `app/`：归档中的旧 Electron 壳；当前默认运行、构建、测试链路都不再依赖该目录。
- `apps/desktop/`：当前唯一有效的桌面端主实现，包含前台、Rust host、locale 真源与迁移中的 JS 领域模块。
- `build/`：打包所需图标与安装器资源。
- `docs/`：变更日志、项目理解文档、任务 specs、plans 与 logs。
- `examples/`：平台服务集成示例。
- `graphics/`：应用图标源文件与生成脚本；其中 `app-icon.svg` 面向 Dock / bundle，`tray-icon.svg` 面向 macOS tray，小尺寸几何单独维护，`generate_icon_assets.py` 负责把两者展开为打包资源。
- `scripts/`：工作流脚本、校验器与任务脚手架。
- `test/`：Vitest 测试用例与测试辅助资源。
- `package.json`：根级 npm scripts；`start/dev/build/pack/dist` 全部转发到 `apps/desktop`。
- `vitest.config.ts`：Vitest 运行与覆盖率配置。

## app/（archived）
- 旧 Electron 主进程、旧窗口页面、旧 preload/renderer、历史资源与旧 i18n 资产。
- 该目录当前只保留为历史参考，不应再作为默认运行、构建、测试或 locale 生成的输入。

## apps/desktop/
- `apps/desktop/package.json`：Tauri desktop 子应用的依赖与开发命令。
- `apps/desktop/components.json`：shadcn 风格组件基座声明，绑定 `src/styles.css`、`src/components/ui/*` 与别名约定。
- `apps/desktop/src/App.tsx`：同构的 Tauri 主设置页与 break prompt；根据 query 参数切换偏好页或独立 break prompt，浏览器 preview 也支持 `?preview=paused|focus` 取证。
- `apps/desktop/src/assets/audio/`：Tauri 前台 break start sound 资源，当前复用 legacy `app/audio/*.wav`。
- `apps/desktop/src/components/ui/`：Button/Card/Switch/Input/Textarea/Accordion/Select/Badge/SegmentedControl 等前台 primitives。
- `apps/desktop/src/i18n.ts`：React 前端的轻量 i18n lookup / 插值层，消费 `registry.generated.json` 并负责语言 fallback 与时长格式化。
- `apps/desktop/src/lib/break-prompt.ts`：break prompt 的预设背景、提示音映射和自定义壁纸压缩 helper。
- `apps/desktop/src/lib/utils.ts`：`cn()` 合并工具，供 shadcn/Tailwind primitives 复用。
- `apps/desktop/src/locales/break-message-copy.json`：break 消息页专属文案的可编辑真源。
- `apps/desktop/src/locales/break-message-copy.ts`：读取 break 消息页专属文案 JSON 的 helper。
- `apps/desktop/src/locales/messages/`：桌面端自维护的每语言基础消息文件。
- `apps/desktop/src/locales/config/`：桌面端自维护的每语言配置文件（code / label / fallback / desktopReady）。
- `apps/desktop/src/locales/registry.generated.json`：由 `scripts/sync_desktop_locales.py` 基于桌面端 locale 目录生成的前后端共享 locale registry。
- `apps/desktop/src/styles.css`：Tailwind CSS v4 token、base layer 与 Tauri 设置页 / break prompt 的玻璃化背景和动效。
- `apps/desktop/public/pauza.svg`：前端 branding 资源。
- `apps/desktop/legacy-utils/`：从旧壳迁入 `apps` 体系的 JS 领域/兼容辅助模块，当前主要供根级测试与迁移期 source basis 使用。

## apps/desktop/src-tauri/
- `apps/desktop/src-tauri/tauri.conf.json`：Tauri 2 应用配置、窗口尺寸与 bundle icon 列表。
- `apps/desktop/src-tauri/src/i18n.rs`：Rust host 侧共享 locale lookup 与插值层，消费 `registry.generated.json` 并与前端使用同一语言 fallback 规则。
- `apps/desktop/src-tauri/src/lib.rs`：插件装配、状态初始化、engine 启动与命令注册入口。
- `apps/desktop/src-tauri/src/commands.rs`：设置、pause/focus、break actions、autostart 与主窗口命令边界。
- `apps/desktop/src-tauri/src/engine.rs`：后台 tick 循环，驱动调度状态机、通知与 break window。
- `apps/desktop/src-tauri/src/platform.rs`：idle / DND / app exclusion 的跨平台轻量探测层；`idle_ms` 已与 `natural_breaks` 开关解耦，始终可供智能提醒使用。
- `apps/desktop/src-tauri/src/shell.rs`：tray 菜单、托盘点击、全局快捷键与 break/main window 生命周期；macOS 下还负责定向 patch Pauza 自己的 `NSStatusItem` tray image，并在开发态兜底 Dock icon。
- `apps/desktop/src-tauri/src/state.rs`：`PauzaSettings`、`RuntimeState`、`DesktopSnapshot` 的运行时真源，现已覆盖通知、postpone、`reminder_mode`、manual finish、break surface、自定义壁纸 / cue / start/end sound、shortcut 配置，并通过 i18n 层输出本地化状态文案。

## apps/desktop/legacy-utils/
- `defaultSettings.js`：迁移期保留的设置默认值基线。
- `breakWindowProfile.js`：旧 break 展示策略计算逻辑的迁入副本。
- `displayManager.js`、`dndManager.js`、`naturalBreaksManager.js`、`appExclusionsManager.js`：系统状态与暂停逻辑辅助模块。
- `breakShortcuts.js`、`commands.js`、`scheduler.js`：快捷键、命令与定时调度辅助逻辑。
- `ideasLoader.js`、`sanitizeIdea.js`、`statusMessages.js` 等：仍由测试或迁移审查使用的领域工具。

## test/
- `test/breakWindowProfile.js`：break 窗口策略的纯逻辑测试。
- `test/commands.js`、`test/breakShortcuts.js`：命令行与快捷键行为测试。
- `test/dndManager.js`、`test/naturalBreaksManager.js`、`test/appExclusionsManager.js`：系统状态与暂停逻辑测试。
- `test/displayManager.js`、`test/appIcon.js`：窗口显示与图标计算测试。
- `test/translations.js`：桌面端 `apps/desktop/src/locales/messages/*.json` 的多语言资源一致性测试。
- 其余文件主要覆盖 `apps/desktop/legacy-utils/*` 与当前桌面端的共享逻辑。

## docs/
- `docs/RepositoryGuidelines.md`：仓库结构、命令与开发约束索引。
- `docs/CodeMap.md`：当前文件树职责地图。
- `docs/Architecture.md`：运行时架构摘要。
- `docs/ReminderScheduling.md`：下一轮提醒状态机与调度逻辑设计文档，定义智能提醒、强制提醒、严格模式与自然休息的职责边界。
- `docs/UI.md`：窗口与交互结构摘要。
- `docs/SettingsInventory.md`：当前设置页、Tauri 真源隐藏设置、旧版候选设置与运行时动作的边界清单。
- `docs/CHANGELOG.md`：当前中文变更日志入口。
- `docs/plans/`、`docs/logs/`、`docs/specs/`：Codex 任务闭环文档。
