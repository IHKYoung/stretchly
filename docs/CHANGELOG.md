# 变更日志

本文件是 `Pauza` 后续维护的正式变更日志，统一使用简体中文记录。

说明：

- 自 2026-04-02 起，所有新变更优先记录在本文件
- 根目录 `CHANGELOG.md` 仅保留迁移说明与入口导航
- 历史英文原始记录见 `docs/archive/CHANGELOG.upstream.en.md`
- 历史中文整理见 `docs/历史版本整理.md`

记录规则：

- 按 `未发布` 或版本号维护
- 分类优先使用：`新增`、`调整`、`修复`、`移除`
- 优先记录用户可感知变化、兼容性影响、迁移事项与关键技术变更

## 未发布

### 调整

- 将产品名称统一调整为 `Pauza`
- 将后续变更日志迁移到 `docs/CHANGELOG.md`，统一改为中文维护
- 将原根目录英文 `CHANGELOG.md` 归档到 `docs/archive/CHANGELOG.upstream.en.md`
- 新增 `Interruption style`（gentle / balanced / immersive），默认采用更温和的 break 展示策略
- 托盘新增 `Focus session` 入口，允许用户主动保护 25 / 45 / 60 分钟心流
- 重做 welcome / preferences / mini break / long break 的视觉样式，统一为更克制的暖白桌面工具风格，减少发光感和过度装饰
- 删除仓库中与产品主体无关的第一批外围资产，包括 Docker/Snap 辅助壳文件与调试页，降低二次开发噪音
- 新增 `apps/desktop` Tauri 2 桌面基础壳，用 Rust host 接管 tray / shortcut / notification / autostart，并提供可浏览器预览的新 dashboard
- 将 `apps/desktop` 从迁移展示页推进为可用核心闭环：Tauri host 现已接管设置持久化、调度、pause/focus、自然休息、DND、应用排除与 break prompt，前台改为极简设置页
- 将 `apps/desktop` 的多语言管理重构为共享 locale 资源层，恢复语言切换并默认使用中文；tray、break prompt 与运行时状态文案不再以内联 if/else 维护
- 将根目录默认 `start/dev/build/pack/dist/postinstall` 脚本切换到 `apps/desktop` Tauri 2 runtime，Electron 入口降级为 `legacy:*` 保留；同时补齐 Tauri host 的 pre-break notification、strict/manual finish、skip/reset、tray submenu 与快捷键能力
- 根据用户反馈，将 Tauri 设置页从控制台/展示页语言继续收敛为更像现代 macOS 小工具的偏好页：默认可见内容仅保留节奏、信号、通用三组，高级项折叠；视觉语言改为安静的 grouped settings、segmented control、switch、setting-card 与更简洁的 break prompt
- 将 `apps/desktop` 前台进一步重构为 Tailwind CSS v4 + shadcn 风格组件基座，并按 Apple HIG 收敛为 overview hero、grouped settings、sticky save rail 与环形倒计时 break prompt
- 将 `apps/desktop` 设置页进一步改为侧边栏分类布局：左侧切换 `概览 / 节奏 / 提醒与延后 / 打断与显示 / 智能暂停 / 通用与快捷键`，中间查看当前分类详情，右侧固定保存与状态栏；现有 Tauri 设置项全部保留，不再折叠进单一高级 accordion
- 根据最新反馈，进一步把 `apps/desktop` 设置页从全局三栏修正为稳定的双栏 split view：左侧 sidebar 约占 `1/4`，右侧主体约占 `3/4`，保存/状态区并回主体顶部；同时将主窗口默认与最小尺寸提高到 `1440x810` / `1280x720`
- 同步收紧 `apps/desktop` 前台组件的圆角并裁掉设置页里偏介绍性的冗长文案，让整体更接近克制的桌面工具
- 将 `apps/desktop` 设置页继续极简化：主结构只保留 `节奏 / 提醒与打断 / 智能暂停 / 通用` 四个分类与单一主面板，设置内容只暴露核心真设置；overview、快捷动作、快捷键编辑和多余卡片感已从页面主结构移除
- 将 `apps/desktop` 节奏页顶部四个核心时间输入改为 preset 芯片，保留“提前提醒 / 延后”两组 stepper，降低高频时间设置的输入负担
- 恢复 `apps/desktop` 休息提示的 `窗口 / 全屏` 设置：Tauri `PauzaSettings` 重新持久化 `fullscreen`，设置页在“提醒与打断”中重新暴露该入口，window 模式下 immersive long break 不再被强制全屏
- 修复 `apps/desktop` 休息窗口过小和双层卡片的问题：break 页面现在直接使用宿主窗口空间，window 模式下各档默认尺寸整体拉大，不再出现“窗口里再塞一个小窗”的观感
- 根据最新反馈，将 `apps/desktop` 主设置窗口继续压缩为更像桌面工具的紧凑偏好页：默认主窗口调整为 `1040x585`、最小 `960x540`，侧栏收窄为约 `168px`，按钮/选择器/分段控件/开关/多行输入统一改为更平直的小圆角样式，页面移除大卡片壳、重复标题、侧栏状态区与展示型背景，并将设置改为自动保存

### 修复

- 修复 `apps/desktop` 在 macOS 上顶部菜单栏 tray icon 右键时的原生菜单闪退；tray 根 context menu 现按平台分支为 macOS 使用 `Submenu`，避免在 root `Menu` 上直接挂普通 `MenuItem`
- 修复 `apps/desktop` 的 tray 菜单会在后台 1s tick 中被反复重建的问题；tray 现在只在菜单内容有效变化时刷新，不再让已展开的原生菜单一闪即逝
- 修复 `apps/desktop` fullscreen 休息窗口在点击 `跳过` 后可能残留黑屏的问题：break close path 现在会先退出 fullscreen，再销毁窗口实例，不再只隐藏 fullscreen break
- 修复 `apps/desktop` break prompt 点击 `完成休息 / 稍后 / 跳过` 时可能直接退出应用的问题：break CTA 现先返回 `DesktopSnapshot`，再异步销毁当前 break webview，避免在 `invoke` 回包过程中同步 teardown 当前窗口
- 修复 `apps/desktop` 在 macOS 上顶部 tray icon 发糊且偏大的问题：tray 现直接 patch Pauza 自己的 `NSStatusItem`，并使用 1x/2x template image；同时重画了小尺寸 tray SVG，避免 glyph 贴满菜单栏槽位
- 修复 `apps/desktop` 在 macOS 上 Dock icon 偏大的问题：应用图标现为更保守的安全边距构图，release `.app` 也不再执行额外的 Dock runtime patch，优先使用系统 bundle icon
- 修复图标生成链中的两个真实故障：`graphics/generate_icon_assets.py` 不再对同一路径做 ffmpeg 原地覆盖，`ensure_srgb` 也恢复兼容输出路径参数，图标资源可重新稳定生成

### 文档

- 新增 `docs/历史版本整理.md`，按阶段整理历史版本演进
