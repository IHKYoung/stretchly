# Architecture

## 总览
Pauza 当前仍处于双壳阶段，但默认入口已经从 Electron 切到 `apps/desktop`：根 `package.json` 的 `start/dev/build/pack/dist/postinstall` 默认走 Tauri 2 runtime，旧 `app/**` 则退为 `legacy:*` 入口与 source basis。当前产品结构可以理解为“Electron 参考实现 + Tauri 主实现”的过渡态，后续方向是在 Rust host 内继续补统计、自适应节奏和生态集成，再进一步清理 Electron 外围资产。

## 运行时分层
- 主进程：`app/main.js` 维护全部窗口引用、休息状态机、设置持久化、i18n、托盘与系统事件监听。
- 调度域：`app/breaksPlanner.js` 组合 `Scheduler`、`NaturalBreaksManager`、`DndManager`、`AppExclusionsManager`，决定何时通知、开始、暂停、恢复或结束休息。
- 渲染层：`app/break.html`、`app/microbreak.html`、`app/preferences.html`、`app/welcome.html`、`app/process.html` 及其 renderer/preload 文件负责界面交互。
- 窗口策略层：`app/utils/breakWindowProfile.js` 根据 `breakPromptStyle`、屏幕尺寸和既有窗口配置，计算 gentle / balanced / immersive 三种 break 展示策略。
- 桥接层：`app/utils/context-bridge-exposers.js` 把 IPC 能力切成 `settings`、`pauza`、`breaks`、`utils`、`runtime` 等稳定接口。
- 持久化与资源：`electron-store` 保存偏好设置，`app/locales/*.json` 保存文案，`app/audio/`、`app/images/`、`graphics/` 提供运行时与打包资源。
- 新桌面壳：`apps/desktop/src-tauri/src/lib.rs`、`commands.rs`、`shell.rs`、`state.rs`、`engine.rs`、`platform.rs` 组成 Tauri host，负责设置持久化、调度、自然休息、DND、应用排除、tray、快捷键、通知、manual finish、postpone limit、strict break 与 break/main window 生命周期。
- 新工作台：`apps/desktop/src/App.tsx`、`src/components/ui/*` 与 `src/styles.css` 现在组成 Tailwind CSS v4 + shadcn 风格的前台层，承接 Apple HIG 风格的设置页和 break prompt；前台只消费 `DesktopSnapshot`，不自行维护并行运行时真源。

## 核心时序
1. `app.on('ready', initialize)` 启动后加载 `electron-store` 默认设置、执行迁移、初始化自动启动与多语言。
2. `BreaksPlanner.nextBreak()` 依据 `microbreakInterval`、`breakInterval`、通知开关和暂停状态规划下一次事件。
3. 到达通知或休息触发点后，主进程先用 `breakWindowProfile` 计算窗口尺寸、位置、聚焦与置顶策略，再创建对应 `BrowserWindow`，并通过 IPC 将持续时间、严格模式、跳过/推迟能力、视觉参数与当前干预风格发送给渲染层。
4. 休息结束、手动完成、跳过或推迟后，主进程更新 danger 值、清理窗口、重建下一次调度并刷新托盘状态。
5. 设置变更通过 `save-setting` IPC 即时写回 `electron-store`，必要时同步更新主题、托盘、DND 监听、自动启动或新的 break 展示风格。

## Tauri 核心时序
1. `lib.rs` 在 setup 时初始化 `PauzaState`，从 app config 目录加载或创建 `settings.json`。
2. `engine.rs` 启动后台 1s tick，周期性调用 `platform.rs` 获取 idle / DND / app exclusion 信号。
3. `state.rs::tick()` 根据当前阻塞态、休息计划、pre-break notification、active break 与 manual-awaiting 状态计算下一步动作，并返回 `EngineActions`。
4. `shell.rs` 根据动作显示/隐藏 break prompt、维持 tray/shortcut 与主窗口行为，并为 gentle / balanced / immersive 计算不同 break window profile；其中 macOS tray context menu 现在显式使用 `Submenu` 作为根菜单以匹配 `muda` 的平台约束。
5. `engine.rs` 的后台 tick 不再每秒无条件重建 tray menu，而是只在 tray 菜单内容有效变化时刷新，避免 macOS 原生菜单刚展开就被替换。
5. 前台通过 `commands.rs` 读写 `DesktopSnapshot`；设置页和 break prompt 始终消费同一份运行时状态，pause/focus/skip/reset/autostart 都经同一命令面闭环。

## 关键状态
- `settings.store`：所有用户偏好与 Contributor 配置的真源。
- `settings.store.breakPromptStyle`：break 展示强度的真源，控制 gentle / balanced / immersive 三种体验。
- `settings.store.fullscreen` / `PauzaState.settings.fullscreen`：break surface mode 的真源，控制休息提示以窗口还是全屏出现。
- `breakPlanner.scheduler`：当前倒计时和引用事件名，是休息状态机的核心运行状态。
- `danger`：Break Health Mode 的视觉强度累积值，跳过/推迟会升高，正常完成会降低。
- `global.isNewVersion`、`global.isContributor`：控制升级提示与 Contributor 功能入口。
- `microbreakWins`、`breakWins`、`preferencesWin` 等窗口引用：决定当前 UI 生命周期与 IPC handler 注册时机。
- `PauzaState.settings`：Tauri 端的本地配置真源，已覆盖 notification、postpone、strict/manual finish、surface、fullscreen 和 shortcut 等 parity 配置。
- `PauzaState.current_break`：Tauri 端当前 break 生命周期真源，决定 `manualAwaiting`、`canPostpone`、`canSkip` 与窗口关闭策略。
- `DesktopSnapshot`：Tauri 前台唯一可读模型，避免前台自行拼装运行时状态。

## 平台与外部集成
- CLI：通过 `Command` 和单实例锁实现 `pauza pause|resume|mini|long|preferences` 等命令转发。
- 系统状态：`powerMonitor` 处理 suspend/lock，`DndManager` 处理免打扰，`NaturalBreaksManager` 处理空闲时间。
- 平台兼容：包含 Windows Store、Portable、Flatpak、Snap 分支处理，以及自动启动与 Portal 兼容逻辑。
- 云端入口：Contributor/Auth/Sync 页面通过独立 BrowserWindow 加载远端页面，但核心休息逻辑仍全部留在本地主进程。
- Tauri 宿主能力：当前已覆盖设置持久化、调度、tray、global shortcut、notification、autostart、主窗口生命周期、break prompt、skip/reset/pause/focus actions，以及基于 `sysinfo + 系统命令` 的 idle / DND / app exclusion 轻量迁移。

## 个性化开发优先入口
- 产品行为：`app/utils/defaultSettings.js`、`app/utils/breakWindowProfile.js`、`app/breaksPlanner.js`、`app/main.js`
- UI 外观：`app/css/*.css`、`app/*-renderer.js`、`app/*-preload.mjs`
- 多显示器与系统联动：`app/utils/displayManager.js`、`app/utils/dndManager.js`、`app/utils/naturalBreaksManager.js`
- 快捷键与命令面板：`app/utils/breakShortcuts.js`、`app/utils/commands.js`
- 新跨平台壳：`apps/desktop/src-tauri/src/*.rs`、`apps/desktop/src/App.tsx`、`apps/desktop/src/components/ui/*`、`apps/desktop/src/styles.css`
