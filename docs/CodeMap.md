# CodeMap

> 维护规则：该文件描述当前仓库文件树与职责，确保与实际目录一致。

## Top Level
- `.github/`：CI 工作流、GitHub 模板与仓库级 Copilot/agent 辅助文档。
- `.githooks/`：Codex 工作流校验与提交审计 hooks。
- `README.md`：根目录快速入口，汇总当前有效的运行、测试与打包命令。
- `apps/desktop/`：当前唯一有效的桌面端主实现，包含前台、Rust host 与 locale 真源。
- `apps/site/`：Pauza 单页官网与下载跳转目录；纯静态实现，现已补齐独立 workflow mirror、release playbook 与发布脚本。
- `build/`：打包所需图标与安装器资源。
- `docs/`：变更日志、项目理解文档、任务 specs、plans 与 logs。
- `graphics/`：应用图标源文件与生成脚本；其中 `app-icon.svg` 面向 Dock / bundle，`tray-icon.svg` 面向 macOS tray，小尺寸几何单独维护，`generate_icon_assets.py` 负责把两者展开为打包资源。
- `scripts/`：工作流脚本、校验器与任务脚手架。
- `test/`：Vitest 测试用例与测试辅助资源。
- `package.json`：根级 npm scripts；当前只保留 `dev/build/typecheck` 短入口，以及 `desktop:*`、`site:*`、`test:*` 命名空间命令。
- `vitest.config.ts`：Vitest 运行与覆盖率配置。

## apps/desktop/
- `apps/desktop/package.json`：Tauri desktop 子应用的依赖与开发命令。
- `apps/desktop/components.json`：shadcn 风格组件基座声明，绑定 `src/styles.css`、`src/components/ui/*` 与别名约定。
- `apps/desktop/src/App.tsx`：同构的 Tauri 主设置页与 break prompt；根据 query 参数切换偏好页或独立 break prompt，浏览器 preview 也支持 `?preview=paused|focus` 取证。
- `apps/desktop/src/assets/audio/`：Tauri 前台 break start / end sound 资源，当前作为 `apps/desktop` 自带静态资源维护。
- `apps/desktop/src/components/ui/`：Button/Card/Switch/Input/Textarea/Accordion/Select/Badge/SegmentedControl 等前台 primitives。
- `apps/desktop/src/i18n.ts`：React 前端的轻量 i18n lookup / 插值层，消费 `registry.generated.json` 并负责界面语言 fallback 与时长格式化。
- `apps/desktop/src/lib/break-copy-layout.ts`：break prompt 文案分行 helper，负责“整句优先、超长句按分句换行”的阅读节奏。
- `apps/desktop/src/lib/settings-controls.ts`：节奏页 preset 常量与数字草稿提交 helper，负责“5 个候选项 + blur/Enter 提交”的输入交互。
- `apps/desktop/src/lib/break-ideas.ts`：break prompt 的选择 helper，消费 `locales/break-ideas/registry.generated.json` 并用稳定混洗索引避免固定周期总是命中同一条。
- `apps/desktop/src/lib/break-prompt.ts`：break prompt 的预设背景、提示音映射和自定义壁纸压缩 helper。
- `apps/desktop/src/lib/utils.ts`：`cn()` 合并工具，供 shadcn/Tailwind primitives 复用。
- `apps/desktop/src/locales/messages/`：桌面端自维护的每语言基础消息文件；break prompt 默认提示语统一收口在这里的 `ui.breakCopy.*`。
- `apps/desktop/src/locales/break-ideas/messages/`：桌面端 break ideas 真源，每语言只保留 `miniBreakIdeas` / `longBreakIdeas` 两组内容资产。
- `apps/desktop/src/locales/break-ideas/registry.json`：break ideas 的源 metadata，当前用于声明默认语言与 `official / legacy` 语言边界。
- `apps/desktop/src/locales/break-ideas/registry.generated.json`：由 `scripts/sync_desktop_break_ideas.py` 生成的运行时 break ideas registry，包含 fallback/tier/available 元数据与 ideas bundles。
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
- `apps/desktop/src-tauri/src/shell.rs`：tray 菜单、托盘点击、全局快捷键与 break/main window 生命周期；macOS 下还负责 break window 的当前 Space/fullscreen overlay 策略、定向 patch Pauza 自己的 `NSStatusItem` tray image，并在开发态兜底 Dock icon。
- `apps/desktop/src-tauri/src/state.rs`：桌面端 runtime 调度主文件，保留 `RuntimeState` / `PauzaState`、当前 break 生命周期、snapshot 与本地化状态文案；settings schema 与 persistence 已拆到子模块。
- `apps/desktop/src-tauri/src/state/settings.rs`：`PauzaSettings`、相关 enum、shortcut、默认值与 sanitize helper 的真源，负责 settings schema 边界。
- `apps/desktop/src-tauri/src/state/persistence.rs`：`settings.json` 的 load / save / legacy migration helper，负责 host 设置持久化边界。
- `apps/desktop/src-tauri/src/state/tests.rs`：`state` 模块的 Rust 单测集合，从 `state.rs` 内联测试拆出，继续覆盖调度、阻塞、legacy migration 与 snapshot 状态边界。

## apps/site/
- `apps/site/index.html`：官网单页首页；当前采用纯白纸面背景、左上角终端提示头、可聚焦的无卡片打字机舞台与独立交互层，只保留一个下载按钮。
- `apps/site/favicon.svg`：官网与下载页共用的站点图标；当前已改为透明底，开口朝向左上，作为桌面端候选图标的几何来源。
- `apps/site/styles.css`：官网视觉层，负责纸面/网格质感、无卡片的 `80%` 舞台、更醒目的左上 prompt、浏览器宽度 `80%` 的输出区、统一 `LXGW WenKai Screen` 字体、以及更大更密的符号粒子和提醒气泡样式。
- `apps/site/script.js`：首页打字机与交互脚本，负责 `10s hold`、逐字输入/退格、pointer/scroll/click 反馈，以及以 `0 / 1 / # / @ / ！ / ¥ / $` 为主的更高密度粒子和点击短句气泡。
- `apps/site/copy.js`：官网文案真源，当前分为打字机长文案池与点击调侃短句池；长文案主要精选自桌面端 `locales/break-ideas/messages/zh-CN.json`，并补充网站专用短句。
- `apps/site/fonts/LXGWWenKaiScreen.ttf`：官网自带的 `LXGW WenKai Screen` 字体资源，用于实现“落霞孤鹜 / 霞鹜”风格。
- `apps/site/download/targets.js`：网站下载目标地址真源；首页 CTA 只连到站内稳定路由，真实 URL 统一在这里维护。
- `apps/site/.githooks/`：站点子目录自维护的 workflow hooks mirror，用于在单独发布站点时保留 commit / docs 门禁。
- `apps/site/docs/`：站点子目录的 release playbook、daily plans/logs/specs 与 changelog，用于独立维护站点发布闭环。
- `apps/site/scripts/`：站点子目录的 workflow mirror 与 `publish_site_release.py`，支持一键更新 pinned URL 并创建/上传 GitHub release。
- `apps/site/download/redirect.js`：下载跳转页的统一逻辑，负责 loading、fallback 和自动跳转。
- `apps/site/download/styles.css`：下载跳转页的共享视觉样式。
- `apps/site/download/*/index.html`：按平台拆分的稳定下载路由页面。

## test/
- `test/translations.js`：桌面端 `apps/desktop/src/locales/messages/*.json` 与 `apps/desktop/src/locales/break-ideas/messages/*.json` 的多语言资源一致性测试。
- `test/desktopBreakCopySource.js`：校验界面文案与 break ideas 已分层，`messages/*.json` 不再保留 `miniBreakIdeas / longBreakIdeas`，官方语言 tier 也与 break ideas registry 对齐。
- `test/desktopBreakIdeas.js`：校验 break ideas helper 只消费独立 registry，并按语言 fallback 解析 bundle 与稳定轮换文案。
- `test/desktopBreakCopyLayout.js`：校验 break prompt 的中英文分行规则，避免文案再次被从中间截断。
- `test/desktopSettingsControls.js`：校验设置页 preset 与数字草稿提交 helper 的交互边界。

## docs/
- `docs/RepositoryGuidelines.md`：仓库结构、命令与开发约束索引。
- `docs/CodeMap.md`：当前文件树职责地图。
- `docs/Architecture.md`：运行时架构摘要。
- `docs/RootMetadataArchive.md`：原根目录 README / LICENSE / 社区治理文件 / Linux 发布元数据的提炼归档。
- `docs/ReminderScheduling.md`：当前 reminder 调度设计文档，定义 smart / forced 的最小状态机、固定空档阈值、最长等待和自然休息边界。
- `docs/PauzaV1SixStepPlan.md`：Pauza 从 Stretchly fork 演进为“低打扰恢复节奏工具”的六步产品方案，覆盖定位、onboarding、主窗口、低打扰能力、反馈闭环与付费边界。
- `docs/UI.md`：窗口与交互结构摘要。
- `docs/SettingsInventory.md`：当前设置页、Tauri 真源隐藏设置、旧版候选设置与运行时动作的边界清单。
- `docs/CHANGELOG.md`：当前中文变更日志入口。
- `docs/commits/`：按日维护的 commit / merge 审计记录。
- `docs/plans/`、`docs/logs/`、`docs/specs/`：Codex 任务闭环文档。
