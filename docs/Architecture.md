# Architecture

## 总览
Pauza 当前的桌面端单一真源是 `apps/desktop`。根 `package.json` 的 `start/dev/build/pack/dist/postinstall` 全部转发到 Tauri 2 桌面壳；`app/**` 只作为归档参考保留，不再参与默认运行、构建、测试或 locale 生成链路。当前架构目标不是继续维护“双壳并行”，而是把所有有效桌面端资产持续收口到 `apps/desktop`。

## 运行时分层
- 前台层：`apps/desktop/src/App.tsx`、`src/components/ui/*`、`src/styles.css` 组成设置页与 break prompt 的 React 前台；浏览器 preview 与原生 runtime 共用同一 UI 代码。
- 文案层：`apps/desktop/src/locales/messages/*.json`、`config/*.json` 与 `break-message-copy.json` 是桌面端唯一有效的文案真源；`scripts/sync_desktop_locales.py` 生成 `registry.generated.json`，供前端 `i18n.ts` 与 Rust host `i18n.rs` 共享。
- 前台 helper：`apps/desktop/src/lib/break-prompt.ts` 负责 break 主题、提示音映射、自定义壁纸压缩与 break prompt 辅助逻辑。
- Rust host：`apps/desktop/src-tauri/src/lib.rs`、`commands.rs`、`shell.rs`、`state.rs`、`engine.rs`、`platform.rs` 组成宿主层，负责设置持久化、调度、tray、shortcut、notification、窗口生命周期与系统状态采集。
- 迁移兼容层：`apps/desktop/legacy-utils/*.js` 保存仍由根级 Vitest 或迁移审查使用的 JS 领域模块；这些文件已经迁入 `apps/desktop` 体系，不再要求 `app/utils/*` 参与当前链路。

## Tauri 核心时序
1. `lib.rs` 在 setup 时初始化 `PauzaState`，从 app config 目录加载或创建 `settings.json`。
2. `engine.rs` 启动后台 1s tick，周期性调用 `platform.rs` 获取 idle / DND / app exclusion 信号。
3. `state.rs::tick()` 根据当前阻塞态、休息计划、pre-break notification、due-but-protected、active break 与 manual-awaiting 状态计算下一步动作，并返回 `EngineActions`。
4. `shell.rs` 根据动作显示/隐藏 break prompt、维持 tray/shortcut 与主窗口行为；当前 break window 只保留单一默认 window profile，并由 `fullscreen` 决定是否改为全屏呈现。其中 macOS tray context menu 现在显式使用 `Submenu` 作为根菜单以匹配 `muda` 的平台约束。
5. `engine.rs` 的后台 tick 不再每秒无条件重建 tray menu，而是只在 tray 菜单内容有效变化时刷新，避免 macOS 原生菜单刚展开就被替换。
6. 前台通过 `commands.rs` 读写 `DesktopSnapshot`；设置页和 break prompt 始终消费同一份运行时状态，pause/focus/skip/reset/autostart 都经同一命令面闭环。

## 关键状态
- `PauzaState.settings`：Tauri 端的本地配置真源，已覆盖 notification、postpone、manual finish、`reminder_mode`、surface、fullscreen、break backdrop / custom wallpaper、cue 开关、start/end sound、shortcut，以及隐藏的 `idle_opportunity_seconds` 等 parity 配置。
- `registry.generated.json`：前后端共享 locale registry；上游真源只有桌面端自己的 `messages/*.json` 与 `config/*.json`。
- `break-message-copy.json`：break 消息页专属提示语的用户可编辑真源，独立于通用 locale registry。
- `PauzaState.current_break`：Tauri 端当前 break 生命周期真源，决定 `manualAwaiting`、`canPostpone`、`canSkip` 与窗口关闭策略。
- `RuntimeState.next_break_wait_started_ms`：Tauri 端低打断投递状态机的关键运行时字段，用来标记“已到点但先等空档”。
- `DesktopSnapshot`：Tauri 前台唯一可读模型，避免前台自行拼装运行时状态。
- `breakCustomBackdropDataUrl`：当前自定义壁纸的持久化形态；它不是原始文件路径，而是前端压缩后的 data URL，用来避开当前未配置 `assetProtocol` 时的本地路径复用问题。

## 平台与外部集成
- CLI / shortcut / tray 行为现在统一由 Tauri host 与当前桌面端命令面承接。
- 系统状态采集由 `platform.rs` 管理，覆盖 idle / DND / app exclusion 等轻量探测。
- 平台兼容仍覆盖 macOS、Windows、Linux 桌面环境差异，以及自动启动与 Portal 兼容逻辑。
- Tauri 宿主能力：当前已覆盖设置持久化、调度、tray、global shortcut、notification、autostart、主窗口生命周期、break prompt、skip/reset/pause/focus actions，以及基于 `sysinfo + 系统命令` 的 idle / DND / app exclusion 轻量迁移。
- 当前默认 break delivery 已不再是“固定时间一定打断”，而是以 `reminder_mode` 决定：`smart` 下当用户仍处于连续输入/操作中时，Pauza 会先把 break 标记为 due，等待短暂 idle gap 后再投递；`forced` 下则到点直接严格开始 break。

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
- 文案与 locale：`apps/desktop/src/locales/{messages,config}/`、`apps/desktop/src/locales/break-message-copy.json`
- 迁移中的 JS 领域逻辑：`apps/desktop/legacy-utils/*`
- `app/**` 仅在需要追溯历史行为时作为只读参考，不再是默认开发入口。
