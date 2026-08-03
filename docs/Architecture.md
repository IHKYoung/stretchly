# Architecture

## 总览
Pauza 当前的可运行资产分为两层：`apps/desktop` 负责桌面端产品本体，`apps/site` 负责单页官网与下载稳定路由。根 `package.json` 当前只保留 `dev/build/typecheck` 短入口，以及 `desktop:*`、`site:*`、`test:*` 命名空间脚本；旧 Electron 壳已从当前工作树移除，不再保留“双壳并行”目录。当前架构目标是把桌面端与官网分别收口到明确目录与职责边界，并通过 docs 与 git 历史承载历史追溯。

## 运行时分层
- 前台层：`apps/desktop/src/App.tsx`、`src/components/ui/*`、`src/styles.css` 组成设置页与 break prompt 的 React 前台；浏览器 preview 与原生 runtime 共用同一 UI 代码。
- 设置页前台对语言采用“双层边界”：`form.language` 只表示用户草稿选择，真正驱动整页 labels 与 `document lang/dir` 的则是 `DesktopSnapshot.settings.language`；只有保存回包成功后才切换可见语言。
- 设置页 `CompactNumber` 也采用“双层边界”：输入框内部 `draft` 允许临时空值和多位数，只有 `blur / Enter / step button` 才提交到 `form` 与 autosave，避免每击键直写 `update_settings`。
- 设置页 autosave 现已改为串行/合并保存：同一时刻最多只有一轮 `update_settings` 在飞；保存进行中若继续修改，只会把新的 `formRevision` 合并进下一轮，而不是并发叠加多个宿主设置更新。
- 文案层：`apps/desktop/src/locales/messages/*.json` 与 `config/*.json` 是桌面端界面文案真源；break prompt 默认提示语通过 `messages/*.json` 中的 `ui.breakCopy.*` 维护。`apps/desktop/src/locales/break-ideas/messages/*.json` 与 `break-ideas/registry.json` 是 break ideas 真源，并单独生成 `break-ideas/registry.generated.json` 供前台读取。
- 前台 helper：`apps/desktop/src/lib/break-prompt.ts` 负责 break 主题、提示音映射、自定义壁纸压缩与 break prompt 辅助逻辑；`lib/break-ideas.ts` 负责读取独立 break ideas registry、按 fallback 解析 bundle 并做稳定轮换。
- Rust host：`apps/desktop/src-tauri/src/lib.rs`、`commands.rs`、`shell.rs`、`state.rs`、`engine.rs`、`platform.rs` 组成宿主层，负责设置持久化、调度、tray、shortcut、notification、窗口生命周期与系统状态采集；其中 `state` 现已拆成 `state.rs`（runtime）、`state/settings.rs`（settings schema）和 `state/persistence.rs`（load/save/migration）三层边界。
- `shell.rs` 的 tray 文本热更新现在有一条明确并发不变量：不得在持有 `LAST_TRAY_MENU_TEXT_UPDATER` 锁时调用 `MenuItem::set_text()`。后台 tick 只能在锁内 clone 当前 updater，再在锁外执行主线程派发；否则会形成“后台线程持锁等待主线程 `set_text()` 完成，主线程在 `create_tray_menu() -> register_tray_menu_text_updater()` 中等待同一把锁”的 lock inversion deadlock，表现为设置保存时的 macOS 彩球。

## 官网分层
- `apps/site/index.html` + `styles.css` + `copy.js` + `script.js` 组成单页官网本体，不引入额外前端依赖或构建链。
- `apps/site/fonts/LXGWWenKaiScreen.ttf` 是官网自带字体资源，避免官网依赖外部字体 CDN。
- `apps/site/download/targets.js` 是官网侧唯一的下载目标地址真源；首页 CTA 不直接持有外部下载 URL。
- `apps/site/download/*/index.html` 与 `redirect.js` 共同组成稳定下载路由层：
  - 当目标地址已配置时，页面做短暂倒计时后自动跳转，并保留手动打开入口。
  - 当目标地址留空时，页面停留在 fallback 状态并输出 `console.warn`，避免用户遇到死链或空白页。
- 官网首页当前只承担“品牌气质表达 + 下载入口”职责：中央打字机舞台循环展示精选提醒文案，站点不与桌面端运行时共享状态，也不依赖桌面端构建流程。

## Tauri 核心时序
1. `lib.rs` 在 setup 时初始化 `PauzaState`，从 app config 目录加载或创建 `settings.json`。
2. `engine.rs` 启动后台 1s tick，周期性调用 `platform.rs` 获取 idle / DND / app exclusion 信号。
3. `state.rs::tick()` 根据 active break 生命周期、delivery blocker、休息计划、pre-break notification 与 manual-awaiting 状态计算下一步动作，并返回 `EngineActions`。当前顺序明确保证“已开始的 break 生命周期”优先于 passive blocker，pause/focus/DND/app exclusion 只冻结投递，不再重置节奏。
4. `state/settings.rs` 负责 `PauzaSettings`、enum、shortcut 和 sanitize 规则；`state/persistence.rs` 负责 `settings.json` 的加载、保存与 legacy migration，从而把配置 schema / 持久化与 runtime 调度从单文件中拆开。
5. `shell.rs` 根据动作显示/隐藏 break prompt、维持 tray/shortcut 与主窗口行为；当前 break window 只保留单一默认 window profile，并由 `fullscreen` 决定是否改为全屏呈现。其中 macOS tray context menu 现在显式使用 `Submenu` 作为根菜单以匹配 `muda` 的平台约束；break 窗口在 macOS 上还会额外 patch 原生 `NSWindow` 的 `CanJoinAllSpaces | MoveToActiveSpace | FullScreenAuxiliary` 与最高 native level，并在 non-focusable/windowed 路径显示后显式激活 `NSApplication`，确保在全屏 Space 中也能覆盖当前工作屏幕。对 Pauza 来说，这三个 collection behavior bits 需要一起保留；单独移除 `MoveToActiveSpace` / `FullScreenAuxiliary` 会回退成“break 在别的屏幕或后台 Space 自己开始”的无感状态。app setup 也会在 bundle identifier 可用时请求 macOS 通知权限，避免 bundle 环境下提醒仅走旧通知中心路径而被系统静默降级。
6. `engine.rs` 的后台 tick 不再每秒无条件重建 tray menu，而是只在 tray 菜单内容有效变化时刷新，避免 macOS 原生菜单刚展开就被替换。
7. 前台通过 `commands.rs` 读写 `DesktopSnapshot`；设置页和 break prompt 始终消费同一份运行时状态，pause/focus/skip/reset/autostart 都经同一命令面闭环。
8. `commands.rs::update_settings()` 现采用差异驱动的 host refresh：`PauzaSettings` 未变化时直接短路；shortcut 绑定未变化时不重绑；语言未变化时不做整棵 tray menu rebuild，其余设置最多走 `refresh_tray_if_needed()`。
9. 与上条并行成立的一条更底层约束是：之前“autosave 并发 + host 全量刷新”只解释了为何设置保存更容易撞上宿主刷新，并不是最终根因；真正导致 beachball 的是 `sync_tray_menu_text()` 与 `register_tray_menu_text_updater()` 围绕 `LAST_TRAY_MENU_TEXT_UPDATER` 的锁反转。当前实现通过 `Arc` 化 updater 并在锁外执行 `set_text()` 来打破互锁。

## 关键状态
- `PauzaState.settings`：Tauri 端的本地配置真源，已覆盖 notification、postpone、manual finish、`reminder_mode`、surface、fullscreen、break backdrop / custom wallpaper、cue 开关、start/end sound、shortcut；旧的 `idle_opportunity_seconds` 仅在读取历史配置时兼容保留，不再继续序列化到新的 settings/snapshot。
- `registry.generated.json`：前后端共享 locale registry；上游真源只有桌面端自己的 `messages/*.json` 与 `config/*.json`。
- `break-ideas/registry.generated.json`：前台 break prompt 使用的独立 ideas registry；上游真源是 `break-ideas/messages/*.json` 与 `break-ideas/registry.json`，其中 `official / legacy` 只是数据标签，不参与代码硬编码。
- `PauzaState.current_break`：Tauri 端当前 break 生命周期真源，决定 `manualAwaiting`、`canPostpone`、`canSkip` 与窗口关闭策略。
- `RuntimeState.next_break_wait_started_ms`：Tauri 端低打断投递状态机的关键运行时字段，用来标记“已到点但先等空档”；实际等待规则现为内置的 per-kind 固定阈值 + 最大等待。
- `DesktopSnapshot.next_break_wait_remaining_ms`：由 Tauri 运行时根据上述起点和 per-kind 最大等待派生；只在提醒已到点、正在智能等待且没有其他 blocker 时存在。tray / 前台不得从 `next_break_in_ms` 自行推算该等待进度。
- `RuntimeState.delivery_block_started_ms`：记录 pause/focus/DND/app exclusion 这类 delivery blocker 的进入时间；解除阻塞时会把 due / notification / waiting timer 一并平移，确保 blocker 只冻结投递、不重置节奏。
- `RuntimeState.heads_up_kind(now)`：根据 `next_break_due_ms` 与当前 break 的 lead time 派生出 due 前的 heads-up 阶段；它是设置页运行态和 tray 文本的主信号，不再把 pre-break 能见性完全绑定到一次性系统通知。
- `DesktopSnapshot`：Tauri 前台唯一可读模型，避免前台自行拼装运行时状态。
- `breakCustomBackdropDataUrl`：当前自定义壁纸的持久化形态；它不是原始文件路径，而是前端压缩后的 data URL，用来避开当前未配置 `assetProtocol` 时的本地路径复用问题。

## 平台与外部集成
- CLI / shortcut / tray 行为现在统一由 Tauri host 与当前桌面端命令面承接。
- 系统状态采集由 `platform.rs` 管理，覆盖 idle / DND / app exclusion 等轻量探测。
- 平台兼容仍覆盖 macOS、Windows、Linux 桌面环境差异，以及自动启动与 Portal 兼容逻辑。
- Tauri 宿主能力：当前已覆盖设置持久化、调度、tray、global shortcut、notification、autostart、主窗口生命周期、break prompt、skip/reset/pause/focus actions，以及基于 `sysinfo + 系统命令` 的 idle / DND / app exclusion 轻量迁移。
- 当前默认 break delivery 已不再是“固定时间一定打断”，而是以 `reminder_mode` 决定：`smart` 下当用户仍处于连续输入/操作中时，Pauza 会先把 break 标记为 due，并按 break kind 使用固定阈值寻找空档；若一直没有空档，则在最大等待后直接开始。当前内置策略是：
  - 微休息：到点后要求连续空闲 `8s`；若等待满 `90s` 仍没有空档，则直接开始。
  - 完整休息：到点后要求连续空闲 `12s`；若等待满 `180s` 仍没有空档，则直接开始。
  - `forced` 下则到点直接严格开始 break。
- due 前如果开启了对应 break 的提前提示，Pauza 还会进入一个短暂的 heads-up 阶段：设置页状态、tray 菜单与 tooltip 会先显示“即将开始 / Up next”；系统通知仍可作为辅助，但不再是唯一有效出口。通知投递失败时，`engine.rs` 会输出日志而不是静默吞掉。

## macOS 图标策略
- `graphics/app-icon.svg` 是 Dock / bundle icon 的单一视觉真源，`graphics/tray-icon.svg` 是 macOS 菜单栏 tray glyph 的单一视觉真源；二者不能共用同一套“直接缩放”的几何。
- `graphics/generate_icon_assets.py` 负责把两份 SVG 展开为 `apps/desktop/src-tauri/icons/*`、根目录 `Pauza.png`、`build/icon.icns` 等打包资源；改动图标源后必须先重跑脚本，再构建 Tauri 产物。
- macOS tray icon 的标准目标是 `NSStatusItem` 上的 template image，并提供至少 1x / 2x 两套位图表示；Apple 的高分辨率图像指南建议为 `NSImage` 提供对应分辨率表示，而不是只依赖单张位图缩放。
- 当前 Tauri 侧的特殊点在于底层 `tray-icon` crate 在 macOS 上把状态栏图标逻辑高度固定为 `18pt`。因此：
  - 真正决定“看起来大不大”的，不只是像素尺寸，还有 glyph 本身在画布里的安全边距。
  - 对于 Pauza 这种中空环形图标，如果直接从大图硬缩到小图，边缘会发糊，且视觉体积会偏大。
- 当前实现采用“半标准、半适配层”的策略：
  - 标准部分：tray 保持 template image + 1x/2x 表示；Dock 在 release `.app` 中优先使用 bundle `icon.icns`；app icon 保持 1024 画布并留出安全边距。
  - 适配层部分：由于 Tauri / `tray-icon` 没有直接暴露足够精细的 macOS tray 显示控制，`shell.rs` 通过 `with_inner_tray_icon()` 只 patch Pauza 自己的 `NSStatusItem`，避免扫描全部 status items 的脆弱 hack。
- 之前方案失败的根因有三条：
  - tray 侧只是在宿主层“换图”，但没有把小尺寸 glyph 按菜单栏网格单独设计，导致图标又糊又显大。
  - Dock 侧的 app icon 原始构图过满，主图形几乎填满 1024 画布，所以在 Dock 中视觉重量天然偏大。
  - Dock runtime patch 在 release `.app` 中仍然覆盖系统 bundle icon，等于把本来已经打包好的标准路径又人为改写了一次。
- 参考依据：
  - Apple [`High Resolution Guidelines for OS X`](https://developer.apple.com/library/archive/documentation/GraphicsAnimation/Conceptual/HighResolutionOSX/)：强调 `NSImage` 应提供对应分辨率表示，而不是只依赖单张位图缩放。
  - Apple Human Interface Guidelines [`App icons`](https://developer.apple.com/design/human-interface-guidelines/app-icons)：明确 macOS app icon 使用 `1024x1024` 画布，且不需要把内容填满整个 icon canvas。
  - Apple [`NSStatusBar`](https://developer.apple.com/documentation/appkit/nsstatusbar) / [`NSStatusItem`](https://developer.apple.com/documentation/appkit/nsstatusitem) 文档：菜单栏 status item 的可用空间有限，因此 tray glyph 必须优先为小尺寸场景单独设计。

## 个性化开发优先入口
- 产品行为：`apps/desktop/src-tauri/src/{state,engine,platform,shell,commands}.rs`
- UI 外观：`apps/desktop/src/App.tsx`、`apps/desktop/src/components/ui/*`、`apps/desktop/src/styles.css`
- 文案与 locale：`apps/desktop/src/locales/messages/`、`apps/desktop/src/locales/config/` 与 `apps/desktop/src/locales/break-ideas/`
- 历史行为追溯：优先查看 git 历史、`docs/历史版本整理.md` 与既有 task specs，而不是恢复旧 Electron 壳目录。
