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

当前没有待发布条目。

## 0.1.4 - 2026-07-23

### 调整

- 将根 `package.json`、`package-lock.json`、`apps/desktop/package.json`、`apps/desktop/package-lock.json`、`Cargo.toml` 与 `tauri.conf.json` 的版本真源统一提升到 `0.1.4`
- 将当前 desktop reminder v2 与相关 docs 工作树收口到 `0.1.4`，并以 macOS arm64 安装包作为公开发布资产
- 将 `apps/desktop` 智能提醒重新收敛为固定阈值 + 最长等待模型：移除 recovery hold / recovery credit 与递减阈值曲线，改为微休息 `8s / 90s`、长休息 `12s / 180s`；`smart` 仅在到点后等待明确空档，超过最长等待仍会开始，`forced` 继续保持到点直接打断
- 将 `apps/desktop` 的 break ideas 从 `locales/messages/*.json` 迁移到独立的 `locales/break-ideas/messages/*.json`；新增 `break-ideas/registry.json` 与 `registry.generated.json`，用数据标签标记 `official / legacy` 语言，并让运行时按当前语言优先、fallback 补位读取 ideas
- 扩充英文、简体中文与繁体中文 break ideas，并重新生成运行时 registry
- 整理 `apps/desktop` 的 reminder / break ideas 结构残留：前台设置模型不再继续携带 `idleOpportunitySeconds`，host 仅在读取旧配置时兼容该字段；`tick()` 中到点通知与到点开休息分支也收口到独立 helper，降低 `state.rs` 的主流程噪音
- 继续整理 `apps/desktop` Rust host 的结构边界：`state.rs` 中的 settings schema / sanitize、`settings.json` 的 load-save-migration 与 Rust 单测已分别拆到 `state/settings.rs`、`state/persistence.rs`、`state/tests.rs`，runtime 调度继续留在主文件，先把模块职责拉直而不改行为
- 将官网固定下载地址与本地版本说明同步到 GitHub `v0.1.4` release

### 修复

- 修复 `apps/desktop` 智能提醒在 pause/focus/DND/app exclusion 前后容易漂移的问题：这些 passive blocker 现在只冻结投递并在解除后平移 due / waiting timer，不再整轮 reset；同时 smart 模式新增 recovery hold / recovery credit，用户短暂离开时会在返回后自动抵扣 microbreak、顺延 long break，达到自然休息阈值才 full reset
- 修复 `apps/desktop` smart reminder v2 中 recovery credit 的 due gate 漂移：用户从 `45s+` idle 返回时，只有当前 pending break 已到点才会执行 microbreak 自动抵扣或 long break defer；未到点的微休息不再被提前吃掉，因此不会再出现“几乎只看到长休息”的异常节奏

## 0.1.3 - 2026-04-20

### 新增

- 新增 `apps/site` release workflow mirror：补齐 `.githooks/`、`docs/**`、`scripts/**` 与 `ReleasePlaybook`，让站点下载链接更新与 GitHub release 发布有可复用的一键流程
- 新增一轮多语言 break prompt 文案扩充：桌面端 50 个 locale 的 `miniBreakIdeas` / `longBreakIdeas` 同步增加大量新条目，并重新生成共享 locale registry
- 新增 `TID-20260413-fullscreen-break-current-space-fix` 任务文档包，把本轮 macOS fullscreen Space 回归修复的根因、验证与保留规则正式沉淀
- 新增 `0.1.3` 本地发布整理：当前未提交的 desktop / site / docs 工作树统一收口到同一版 release 边界

### 调整

- 整理根 `package.json` 的 repo 级脚本入口：删除 `start/pack/dist/test-single` 等重复或低价值脚本，新增 `typecheck` 与 `test:*` 命名，保留一套短入口（`dev/build/typecheck`）和一套显式命名空间（`desktop:*`、`site:*`、`test:*`）
- 继续收敛 `apps/site` 官网首页：打字机单条文案现会完整停留约 `10s`，主舞台改为明显居中的 `80%` 视口宽度，并补上 pointer aura、滚动 sweep、点击 burst 与调侃提醒气泡等轻交互层
- 继续收敛 `apps/site` 首页结构：中央舞台已去掉卡片壳，`Pauza>` 改为左上角终端提示头，首页更接近纸面上的终端输出感
- 继续收敛 `apps/site` 终端感细节：`Pauza>` 提示头进一步加重，主输出文案宽度现按浏览器宽度 `80%` 收敛，粒子反馈也从光点改为更偏 `0 / 1 / #` 的符号字元
- 继续收敛 `apps/site` 的字体与粒子语义：全站现统一使用 `LXGW WenKai Screen`，`Pauza>`、下载按钮、互动提示和粒子都不再混用另一套字体；粒子数量、字号和 `0 / 1 / # / @ / ！ / ¥ / $` 符号密度也同步提高
- 将 `README.md`、`docs/RepositoryGuidelines.md`、`docs/CodeMap.md`、`docs/Architecture.md` 与 `docs/UI.md` 的当前真源口径统一为“仓库现仅维护 `apps/desktop` 与 `apps/site`”，不再把已删除的旧 `app/` 目录写成现存模块
- 将 `apps/desktop` 的 break prompt 升级为居中的终端打字风格：顶部保留 `Pauza>` prompt，正文重新与时间、倒计时、CTA 共用中轴；微休息每次只显示一条 prompt，长休息在整句打完后停留 `30s` 再轮播
- 将 `apps/site` 官网下载入口收口回首页单按钮：点击时前端先解析 GitHub `latest release` 的 Apple Silicon DMG，失败时回退到 pinned 稳定链接；站点 release 脚本会同步更新 pinned URL 到 `0.1.3`
- 将根 `package.json`、`apps/desktop/package.json`、`apps/desktop/package-lock.json`、`Cargo.toml` 与 `tauri.conf.json` 的版本真源统一提升到 `0.1.3`

### 修复

- 修复 `apps/desktop` break prompt 文案容易被从中间截断的问题：当前改为“整句优先，超长句按分句换行”，中文会在逗号/句号等自然停顿处独立成行，不再在任意字位折断
- 修复 `apps/desktop` 顶部菜单点击 `跳到下一次休息` 后可能直接卡死的问题：tray 菜单事件不再在点击路径里立刻整棵 `set_menu()` 重建，而是等 native 菜单关闭后再延后按需刷新
- 修复 `apps/desktop` 在 macOS 全屏工作区里到点后 break prompt 可能不浮出，以及设置页数字输入每击键回弹/卡住的问题：break 窗口现会显式激活 app，数字 stepper 现改为“本地草稿 -> blur/Enter/按钮提交”模式
- 修复 `apps/desktop` 设置页点击与改时间仍可能卡死的问题：前端 autosave 现改为串行/合并保存，Rust `update_settings` 也不再每次都无条件重绑快捷键和整棵 tray rebuild；普通设置变化只走按需轻量刷新
- 修复 `apps/desktop` 设置页保存节奏配置时仍可能出现 macOS 彩球卡死的问题：真正根因是后台 `sync_tray_menu_text()` 持有 `LAST_TRAY_MENU_TEXT_UPDATER` 锁时调用 `MenuItem::set_text()`，而设置保存触发的 tray rebuild 会在主线程 `register_tray_menu_text_updater()` 路径争用同一把锁；当前改为锁内只 clone updater、锁外再执行 `set_text()`，消除后台 tick 与主线程 tray rebuild 的锁反转死锁
- 修复 `apps/desktop` break preview 中 prompt 顺序会因模拟 `startedAtMs` 每轮变动而重新抽签的问题：预览态现使用稳定起始时间，方便真实观察打字与轮播节奏
- 修复 `apps/desktop` 在 macOS 全屏工作区里到点后 break 只会在别的屏幕或后台 Space 自己开始的问题：break 宿主窗口重新保留 `CanJoinAllSpaces | MoveToActiveSpace | FullScreenAuxiliary` 组合，不再把后两者从 native `collectionBehavior` 中移除
- 修复 `apps/desktop` 在 bundle 环境下系统通知可能没有显式权限授权的问题：app setup 现会在确认 bundle identifier 可用后主动请求 macOS 通知权限，避免仅靠旧通知中心路径导致提醒被静默降级

### 移除

- 移除旧 Electron `app/**` 目录及其页面、preload/renderer、音频、图片与旧 locale 资产；当前工作树不再保留双壳并行结构
- 移除 `apps/site/download/**` 旧中转页面与样式文件；官网不再通过单独下载页面做跳转

## 0.1.1 - 2026-04-10

### 调整

- 将 `apps/desktop` 的“提前提醒”收敛为真正可见的 `heads-up cue`：due 前会先在设置页运行态和 tray 文本中显示“即将开始 / Up next”，系统通知退居辅助；通知失败也不再静默
- 将产品名称统一调整为 `Pauza`
- 将后续变更日志迁移到 `docs/CHANGELOG.md`，统一改为中文维护
- 将原根目录英文 `CHANGELOG.md` 归档到 `docs/archive/CHANGELOG.upstream.en.md`
- 新增 `Interruption style`（gentle / balanced / immersive），默认采用更温和的 break 展示策略
- 托盘新增 `Focus session` 入口，允许用户主动保护 25 / 45 / 60 分钟心流
- 重做 welcome / preferences / microbreak / break 的视觉样式，统一为更克制的暖白桌面工具风格，减少发光感和过度装饰
- 删除仓库中与产品主体无关的第一批外围资产，包括 Docker/Snap 辅助壳文件与调试页，降低二次开发噪音
- 新增 `apps/desktop` Tauri 2 桌面基础壳，用 Rust host 接管 tray / shortcut / notification / autostart，并提供可浏览器预览的新 dashboard
- 将 `apps/desktop` 从迁移展示页推进为可用核心闭环：Tauri host 现已接管设置持久化、调度、pause/focus、自然休息、DND、应用排除与 break prompt，前台改为极简设置页
- 将 `apps/desktop` 的多语言管理重构为共享 locale 资源层，恢复语言切换并默认使用中文；tray、break prompt 与运行时状态文案不再以内联 if/else 维护
- 将根目录默认 `start/dev/build/pack/dist/postinstall` 脚本切换到 `apps/desktop` Tauri 2 runtime；同时补齐 Tauri host 的 pre-break notification、strict/manual finish、skip/reset、tray submenu 与快捷键能力
- 根据用户反馈，将 Tauri 设置页从控制台/展示页语言继续收敛为更像现代 macOS 小工具的偏好页：默认可见内容仅保留节奏、信号、通用三组，高级项折叠；视觉语言改为安静的 grouped settings、segmented control、switch、setting-card 与更简洁的 break prompt
- 将 `apps/desktop` 前台进一步重构为 Tailwind CSS v4 + shadcn 风格组件基座，并按 Apple HIG 收敛为 overview hero、grouped settings、sticky save rail 与环形倒计时 break prompt
- 将 `apps/desktop` 设置页进一步改为侧边栏分类布局：左侧切换 `概览 / 节奏 / 提醒与延后 / 打断与显示 / 智能暂停 / 通用与快捷键`，中间查看当前分类详情，右侧固定保存与状态栏；现有 Tauri 设置项全部保留，不再折叠进单一高级 accordion
- 根据最新反馈，进一步把 `apps/desktop` 设置页从全局三栏修正为稳定的双栏 split view：左侧 sidebar 约占 `1/4`，右侧主体约占 `3/4`，保存/状态区并回主体顶部；同时将主窗口默认与最小尺寸提高到 `1440x810` / `1280x720`
- 同步收紧 `apps/desktop` 前台组件的圆角并裁掉设置页里偏介绍性的冗长文案，让整体更接近克制的桌面工具
- 将 `apps/desktop` 设置页继续极简化：主结构只保留 `节奏 / 提醒与打断 / 智能暂停 / 通用` 四个分类与单一主面板，设置内容只暴露核心真设置；overview、快捷动作、快捷键编辑和多余卡片感已从页面主结构移除
- 将 `apps/desktop` 节奏页顶部四个核心时间输入改为 preset 芯片，保留“提前提醒 / 延后”两组 stepper，降低高频时间设置的输入负担
- 恢复 `apps/desktop` 休息提示的 `窗口 / 全屏` 设置：Tauri `PauzaSettings` 重新持久化 `fullscreen`，设置页在“提醒与打断”中重新暴露该入口，window 模式下 immersive break 不再被强制全屏
- 将当前用户界面术语统一为 `微休息 / 休息`（英文：`Microbreak / Break`），内部兼容保留 `miniBreak*` / `longBreak*` 等既有标识
- 修复 `apps/desktop` 休息窗口过小和双层卡片的问题：break 页面现在直接使用宿主窗口空间，window 模式下各档默认尺寸整体拉大，不再出现“窗口里再塞一个小窗”的观感
- 根据最新反馈，将 `apps/desktop` 主设置窗口继续压缩为更像桌面工具的紧凑偏好页：默认主窗口调整为 `1040x585`、最小 `960x540`，侧栏收窄为约 `168px`，按钮/选择器/分段控件/开关/多行输入统一改为更平直的小圆角样式，页面移除大卡片壳、重复标题、侧栏状态区与展示型背景，并将设置改为自动保存
- 将 `apps/desktop` 的默认提醒投递策略改为低打断状态机：break 到点后若检测到用户仍在持续输入/操作，会先进入“等待空档”，在检测到短暂停顿后再开始；若等待过久，只发送一次温和提醒，不再固定时间硬插入当前工作流
- 进一步收敛 `apps/desktop` 的提醒模型：设置页现只保留 `智能提醒 / 强制提醒 / 自然休息`；`强制提醒` 直接吸收原 strict mode 语义，host 侧 soft nudge 已移除，`idle_ms` 也不再依赖 `natural_breaks` 开关
- 继续做减法：`apps/desktop` 主实现已移除 `breakPromptStyle / break_prompt_style` 设置真源，window 模式下改用单一默认 break window profile，只保留 `窗口 / 全屏` 这根显示轴
- 将 `apps/desktop` 主设置窗口的宿主边界收敛为默认 `960x640`、最小 `800x600`，避免当前 Tauri 配置继续把窗口压到不可用尺寸
- 将 `apps/desktop` break prompt 向原版体验补齐：偏好页已接回背景主题、自定义壁纸、随机交互语、微休息/休息开始音与音量；break 窗口也恢复 cue card、线性倒计时和更完整的氛围背景
- 根据最新反馈，将 `apps/desktop` break prompt 从“展示型双栏 + cue card + 环形倒计时”重新收敛为纯净单列界面：主视觉只保留一条交互语、数字倒计时和细条形进度，文案统一改由 locale JSON 管理
- 继续收敛 `apps/desktop` 的休息体验：break 卡片进一步改成更通透的玻璃材质，window 模式比例收敛到 16:9，自定义壁纸在设置页改为完整预览，并补上微休息 / 休息的结束提示音
- 将 `apps/desktop` 的多语言结构升级为“每语言一份消息文件 + 每语言一份配置文件 + 自动生成共享 registry”：legacy `app/locales/*.json` 现会同步到 `apps/desktop/src/locales/messages/`，前端和 Rust host 共用 `registry.generated.json`，不再手写 `zh-CN/en` 双分支
- 将 `apps/desktop` 的 locale 真源进一步切到桌面端目录本身：`scripts/sync_desktop_locales.py` 不再从 legacy `app/locales` 或 `app/preferences.html` 回灌，桌面端改为直接维护 `apps/desktop/src/locales/{messages,config}` 并生成共享 registry
- 将 `apps/desktop` break 消息页提示语继续收口为单一真源，但不再保留 `break-message-copy.*` 过渡文件；当前统一维护在 `apps/desktop/src/locales/messages/*.json` 的 `ui.breakCopy.*`
- 将根级测试和迁移期 JS 领域辅助模块统一搬到 `apps/desktop/legacy-utils`，并让默认运行、构建、测试链路彻底停止依赖 `app/`
- 继续做减法：移除 `apps/desktop/legacy-utils` 与依赖它的根级旧测试，默认测试链路只保留当前 desktop locale / break 文案真源校验；`app/` 继续仅作为原版参考目录存在
- 继续清理非主体开发资产：删除 `coverage/`、`examples/`、`output/` 与根目录旧 `README` 展示素材
- 继续清理根级说明/发布元数据：将 `README.md`、`CONTRIBUTING.md`、`CODE_OF_CONDUCT.md`、`LICENSE`、`net.hovancik.Pauza.desktop`、`net.hovancik.Pauza.metainfo.xml` 的有用信息提炼到 `docs/RootMetadataArchive.md`，根目录只保留主体开发相关文件
- 整理 `apps/desktop` 设置页中的运行时动作边界：顶部 `恢复提醒` 现仅在暂停态出现，focus 态改为独立的 `结束专注`，`重置节奏` 单独收进 `节奏控制` 区并明确不会自动解除暂停
- 统一当前开发者显示名：desktop locale、归档 locale、应用元数据与仓库 README 中的 `Jan Hovancik` 现统一替换为 `Clarke Young`；`LICENSE` 历史版权归属保持不变
- 将当前版本统一提升为 `0.1.1`，并在根 `README.md` 补充本版概览、产品主张以及“官网先行，Vercel 部署 + 下载入口”的下一阶段方向
- 先将根 `package.json` / `package-lock.json` 的遗留版本出口收口到与 `apps/desktop`、Tauri 配置和 Cargo manifest 一致的单一版本线，为本轮统一提升到 `0.1.1` 打下基础
- 修复 `apps/desktop` 设置页的语言切换链路：语言选择现改为下拉，完整 locale 列表可见，前后端运行时不再把现有语言强制回退到 `zh-CN/en`
- 调整 `apps/desktop` 节奏页的 preset 和手动输入交互：微休息/休息四组参数现统一保留 5 个候选项，自定义输入允许先清空再录入，并在提交时再 clamp
- 调整 `apps/desktop` 的智能提醒默认节奏：smart 模式的空档阈值默认收紧到 `6s`，设置页“智能暂停”分组也重新暴露 `等待空档` 秒数，让用户可直接微调
- 继续做减法：`apps/desktop` 设置页已收回 smart reminder 的 `等待空档` 调节项，前台只保留 `智能提醒 / 强制提醒` 选择，阈值和等待上限重新回到内部策略
- 将 `apps/desktop` 的 smart reminder 内部策略继续从“单一短空档 + deadline”收敛为递减阈值曲线：微休息改为 `6s -> 3s -> 1s -> 45s deadline`，休息改为 `8s -> 4s -> 1s -> 90s deadline`

### 修复

- 修复 `apps/desktop` 顶部菜单点击 `跳到` 类动作后可能直接卡死的问题：tray 菜单事件不再在点击路径里立刻整棵 `set_menu()` 重建，而是等 native 菜单关闭后再延后按需刷新，避开 macOS 宿主菜单生命周期冲突
- 修复 `apps/desktop` 在 macOS 上顶部菜单栏 tray icon 右键时的原生菜单闪退；tray 根 context menu 现按平台分支为 macOS 使用 `Submenu`，避免在 root `Menu` 上直接挂普通 `MenuItem`
- 修复 `apps/desktop` 的 tray 菜单会在后台 1s tick 中被反复重建的问题；tray 现在只在菜单内容有效变化时刷新，不再让已展开的原生菜单一闪即逝
- 修复 `apps/desktop` 顶部 tray 倒计时和右键 tray 菜单状态不同步的问题：tray 标题现按秒同步真实剩余时间，已展开菜单里的状态文本也会原地实时更新；同时新增设置项控制顶部倒计时是否显示，并补上该项的多语言兼容回退
- 修复 `apps/desktop` break 在 window 模式下仍缩成右下角小浮窗的问题：windowed 宿主 profile 现统一居中，并约占当前工作区 `80% x 80%`，不再保留 microbreak 角落卡片逻辑
- 修复 `apps/desktop` fullscreen 休息窗口在点击 `跳过` 后可能残留黑屏的问题：break close path 现在会先退出 fullscreen，再销毁窗口实例，不再只隐藏 fullscreen break
- 修复 `apps/desktop` break prompt 点击 `完成休息 / 稍后 / 跳过` 时可能直接退出应用的问题：break CTA 现先返回 `DesktopSnapshot`，再异步销毁当前 break webview，避免在 `invoke` 回包过程中同步 teardown 当前窗口
- 修复 `apps/desktop` fullscreen break 在 macOS 上可能出现顶部空白的问题：break 宿主 fullscreen 现统一经由 helper 处理，macOS 改走 simple fullscreen，close path 也会同步兜底退出 simple/native fullscreen
- 修复 `apps/desktop` 在 macOS 全屏工作区里 break 窗口无法覆盖当前工作屏幕的问题：break 宿主窗口现在会额外 patch 原生 `NSWindow` 的 `CanJoinAllSpaces | MoveToActiveSpace | FullScreenAuxiliary`，并直接抬到最高 native level；显示后还会主动 front 到当前 Space
- 修复 `apps/desktop` 在 macOS 上到点后 window 模式 break 仍可能停留在后台的问题：non-focusable break 路径现在除了 `present_break_window()` 外，还会显式激活 `NSApplication`，让提示更可靠地切到当前全屏 Space
- 修复 `apps/desktop` 在 macOS 上顶部 tray icon 发糊且偏大的问题：tray 现直接 patch Pauza 自己的 `NSStatusItem`，并使用 1x/2x template image；同时重画了小尺寸 tray SVG，避免 glyph 贴满菜单栏槽位
- 修复 `apps/desktop` 在 macOS 上 Dock icon 偏大的问题：应用图标现为更保守的安全边距构图，release `.app` 也不再执行额外的 Dock runtime patch，优先使用系统 bundle icon
- 修复图标生成链中的两个真实故障：`graphics/generate_icon_assets.py` 不再对同一路径做 ffmpeg 原地覆盖，`ensure_srgb` 也恢复兼容输出路径参数，图标资源可重新稳定生成
- 修复 `apps/desktop` 设置页里开始音下拉直接露出 `ui.microbreakStartSound` / `ui.longBreakStartSound` 字段名的问题：相关 locale key 已补齐
- 修复 `apps/desktop` 设置页里的中文文案混入 `break window` / `break prompt` 等英文术语的问题，相关提示改回用户向中文表达
- 修复 `apps/desktop` 保存微休息/休息设置时会打断当前 break 状态机的问题：设置更新现仅在没有 active break flow 且未阻塞时才重排 schedule，当前 break 和已排队 next break 会保持到下一轮结束/触发
- 修复 `apps/desktop` break 页交互语总是落在固定 prompt 的问题：当前实现已改回直接从 `miniBreakIdeas` / `longBreakIdeas` 读取 `.text`，并删除 `ui.breakCopy.prompts` 过渡字段；固定 10m/30m 节奏下也不再因为简单取模而一直命中同一条
- 修复 `apps/desktop` 智能提醒可能无限等待的问题：smart 模式当前会先等短空档，但如果你一直持续输入/操作，也会在 bounded wait cap 后自动开始，不再无限 defer
- 修复 `apps/desktop` 智能提醒过于机械的问题：smart 模式现在不再只看单一 idle 阈值，而是先等更明显的空档，再逐步放宽阈值，最后到达 deadline 时直接开始
- 修复 `apps/desktop` break CTA 过宽的问题：倒计时进行中不再允许提前完成，`Later` 也从“前 30% 时间可用”收紧为仅在倒计时开始后的前 10 秒可用
- 修复 `apps/desktop` 设置页切换语言时可能卡住的问题：整页文案与 `dir/lang` 现在会等 autosave 成功后再切换，snapshot 轮询也不再被语言草稿变化重建
- 修复 `apps/desktop` 设置页数字输入手动编辑时会卡住的问题：`CompactNumber` 现改为本地草稿编辑，只在 `blur / Enter / 加减按钮` 时提交 clamp 后的值，支持临时清空、多位数输入和 `Escape` 恢复

### 文档

- 新增 `docs/历史版本整理.md`，按阶段整理历史版本演进
