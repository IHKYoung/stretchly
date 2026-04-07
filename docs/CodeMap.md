# CodeMap

> 维护规则：该文件描述当前仓库文件树与职责，确保与实际目录一致。

## Top Level
- `.github/`：CI 工作流、GitHub 模板与仓库级 Copilot/agent 辅助文档。
- `.githooks/`：Codex 工作流校验与提交审计 hooks。
- `app/`：旧 Electron 应用源码，现已降为 legacy 入口与 Tauri 迁移的领域逻辑参考。
- `apps/desktop/`：新的 Tauri 2 桌面主体，包含极简设置页 / break prompt 前台与 Rust host。
- `build/`：打包所需图标与安装器资源。
- `docs/`：变更日志、项目理解文档、任务 specs、plans 与 logs。
- `examples/`：平台服务集成示例。
- `graphics/`：应用图标源文件与生成脚本。
- `scripts/`：工作流脚本、校验器与任务脚手架。
- `test/`：Vitest 测试用例与测试辅助资源。
- `package.json`：根级 npm scripts；默认入口现已切换到 Tauri，同时仍保留 Electron legacy scripts 与 electron-builder 配置。
- `vitest.config.ts`：Vitest 运行与覆盖率配置。

## app/
- `app/main.js`：主进程中枢，负责启动、设置迁移、托盘、窗口、IPC 和休息生命周期。
- `app/breaksPlanner.js`：休息计划器，整合通知、自然休息、DND 与应用排除规则。
- `app/break.html` / `app/microbreak.html`：长休息与短休息窗口页面。
- `app/preferences.html` / `app/welcome.html` / `app/process.html`：偏好设置、欢迎页与后台处理窗口。
- `app/*-renderer.js`：对应窗口的前端 DOM 行为。
- `app/*-preload.mjs`：窗口 preload，向渲染层暴露受控 API。
- `app/electron-bridge.mjs`：给远端 Contributor/Auth/Sync 页面暴露有限 Electron 能力。
- `app/css/`：窗口样式资源。
- `app/audio/`：提示音资源。
- `app/images/`：应用图标与窗口图片。
- `app/locales/`：多语言文案。

## apps/desktop/
- `apps/desktop/package.json`：Tauri desktop 子应用的依赖与开发命令。
- `apps/desktop/components.json`：shadcn 风格组件基座声明，绑定 `src/styles.css`、`src/components/ui/*` 与别名约定。
- `apps/desktop/src/App.tsx`：同构的 Tauri 主设置页与 break prompt；根据 query 参数切换 Apple HIG 风格的主偏好页或独立 break prompt。
- `apps/desktop/src/components/ui/`：Button/Card/Switch/Input/Textarea/Accordion/Select/Badge/SegmentedControl 等前台 primitives。
- `apps/desktop/src/i18n.ts`：React 前端的轻量 i18n lookup 与插值层，读取 locale JSON 并负责格式化时长。
- `apps/desktop/src/lib/utils.ts`：`cn()` 合并工具，供 shadcn/Tailwind primitives 复用。
- `apps/desktop/src/locales/`：Tauri 前端使用的 `zh-CN / en` locale 资源，默认中文。
- `apps/desktop/src/styles.css`：Tailwind CSS v4 token、base layer 与 Tauri 设置页 / break prompt 的玻璃化背景和动效。
- `apps/desktop/public/pauza.svg`：前端 branding 资源。

## apps/desktop/src-tauri/
- `apps/desktop/src-tauri/tauri.conf.json`：Tauri 2 应用配置、窗口尺寸与 bundle icon 列表。
- `apps/desktop/src-tauri/src/i18n.rs`：Rust host 侧共享 locale lookup 与插值层，消费前端同源 JSON 资源。
- `apps/desktop/src-tauri/src/lib.rs`：插件装配、状态初始化、engine 启动与命令注册入口。
- `apps/desktop/src-tauri/src/commands.rs`：设置、pause/focus、break actions、autostart 与主窗口命令边界。
- `apps/desktop/src-tauri/src/engine.rs`：后台 tick 循环，驱动调度状态机、通知与 break window。
- `apps/desktop/src-tauri/src/platform.rs`：idle / DND / app exclusion 的跨平台轻量探测层。
- `apps/desktop/src-tauri/src/shell.rs`：tray 菜单、托盘点击、全局快捷键与 break/main window 生命周期。
- `apps/desktop/src-tauri/src/state.rs`：`PauzaSettings`、`RuntimeState`、`DesktopSnapshot` 的运行时真源，现已覆盖通知、postpone、strict/manual finish、break surface 与 shortcut 配置，并通过 i18n 层输出本地化状态文案。

## app/utils/
- `defaultSettings.js`：偏好设置默认值与配置面。
- `breakWindowProfile.js`：根据打断风格、屏幕尺寸和旧窗口配置计算 break 窗口展示策略。
- `context-bridge-exposers.js`：preload 对外暴露的所有桥接接口。
- `displayManager.js`：多显示器尺寸与窗口位置选择逻辑。
- `dndManager.js`：跨平台免打扰检测。
- `naturalBreaksManager.js`：空闲时间检测与自然休息暂停逻辑。
- `appExclusionsManager.js`：按进程名或命令行片段控制 pause/resume。
- `breakShortcuts.js`：全局快捷键注册和行为分发。
- `commands.js`：CLI 参数解析与单实例转发命令。
- `scheduler.js`：低层计时器封装。
- `ideasLoader.js` / `sanitizeIdea.js`：休息内容加载与 HTML 安全处理。
- `statusMessages.js`：托盘状态文案拼装。

## test/
- `test/breakWindowProfile.js`：break 窗口策略的纯逻辑测试。
- `test/commands.js`、`test/breakShortcuts.js`：命令行与快捷键行为测试。
- `test/dndManager.js`、`test/naturalBreaksManager.js`、`test/appExclusionsManager.js`：系统状态与暂停逻辑测试。
- `test/displayManager.js`、`test/appIcon.js`：窗口显示与图标计算测试。
- `test/translations.js`：多语言资源一致性测试。
- 其余文件按 `app/utils/*` 对应功能拆分。

## docs/
- `docs/RepositoryGuidelines.md`：仓库结构、命令与开发约束索引。
- `docs/CodeMap.md`：当前文件树职责地图。
- `docs/Architecture.md`：运行时架构摘要。
- `docs/UI.md`：窗口与交互结构摘要。
- `docs/SettingsInventory.md`：当前设置页、Tauri 真源隐藏设置、旧版候选设置与运行时动作的边界清单。
- `docs/CHANGELOG.md`：当前中文变更日志入口。
- `docs/plans/`、`docs/logs/`、`docs/specs/`：Codex 任务闭环文档。
