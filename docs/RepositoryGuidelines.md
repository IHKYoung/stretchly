# Repository Guidelines

## 项目结构与模块组织
- `README.md`：根目录快速入口，汇总当前有效的运行、测试与打包命令。
- `apps/site/`：Pauza 单页官网与下载跳转页，采用纯静态 HTML/CSS/JS 实现，后续适合直接作为 Vercel root directory。
- `apps/desktop/components.json`、`apps/desktop/src/components/ui/*`、`apps/desktop/src/lib/utils.ts`：Tauri 前台的 Tailwind/shadcn 组件基座。
- `apps/desktop/src/App.tsx`、`apps/desktop/src/styles.css`：新的 Tauri 设置工作台与 break prompt 视觉层。
- `apps/desktop/src-tauri/src/*.rs`：新的 Rust host，负责设置持久化、调度、系统信号、tray、shortcut、notification、窗口命令与运行时状态。
- `apps/desktop/src/locales/messages/*.json`、`apps/desktop/src/locales/config/*.json`：桌面端 locale 真源；所有 break prompt 文案也统一并入 `messages/*.json`，`scripts/sync_desktop_locales.py` 会据此生成共享 `registry.generated.json`。
- `test/*.js`：当前保留的 Vitest 测试只覆盖 desktop locale 真源与 break 文案单真源边界。
- `build/`、`graphics/`：安装包资源与图标源文件。
- `docs/`：Codex 工作流文档、变更日志、项目理解资产与根目录历史元数据归档；后续个性化开发建议持续维护这里的索引。
- `scripts/`、`.githooks/`：本仓库的工作流校验、任务脚手架与 Git hooks。

## 构建、测试与开发命令
- `npm run dev`：启动 `apps/desktop` 的 Tauri 桌面端。
- `npm run desktop:install`：安装 `apps/desktop` 的前端与 Tauri 依赖。
- `npm run desktop:dev`：启动新的 Tauri 2 桌面壳（Vite `127.0.0.1:43179`，HMR `43180`）。
- `npm run build`：根级短入口，转发到 `apps/desktop` 的 Tauri 2 打包链路。
- `npm run desktop:build`：显式构建新的 Tauri 2 安装产物。
- `npm run typecheck`：检查 `apps/desktop` 前端 TypeScript。
- `npm run site:dev`：使用本地静态服务器预览 `apps/site` 官网（默认 `127.0.0.1:43210`）。
- `python3 graphics/generate_icon_assets.py`：根据 `graphics/app-icon.svg` 与 `graphics/tray-icon.svg` 重新生成 Tauri / packaging 图标资源；修改图标源文件后必须先跑这条命令。
- `npm test`：运行当前保留的 desktop 相关 Vitest 测试。
- `npm run test:coverage`：运行测试并按需生成覆盖率产物到本地 `coverage/`；该目录属于临时生成物，不应作为长期仓库资产保留。
- `npm run test:watch`：以 watch 模式运行当前保留的 Vitest 测试。
- `npm run lint`：执行 Standard 风格检查。
- `python3 scripts/ensure_workflow_ready.py --target . --hooks required`：补齐并校验 Codex 工作流资产。
- `python3 scripts/sync_desktop_locales.py`：从 `apps/desktop/src/locales/{messages,config}` 生成并校验桌面端共享 `registry.generated.json`。
- `python3 scripts/validate_workflow_docs.py`：检查 docs/specs、plans、logs 的门禁字段。
- `python3 scripts/validate_agent_configs.py`：检查多 Agent 配置；当前仓库无多 Agent 配置时会跳过。

## 开发约束与建议
- 代码主体使用 ESM 风格的原生 JavaScript；除 `vitest.config.ts` 外没有前端框架或 TypeScript 业务层。
- `apps/desktop` 是单一真源；新的桌面端能力、构建链路和测试依赖都应收口到这里，不要继续把 `app/**` 接回默认链路。
- 旧 Electron 壳已从当前工作树移除；如果需要追溯历史行为，应从 git 历史或既有 docs/specs 中提炼后直接迁入 `apps/desktop/src-tauri/**` 或 `apps/desktop/src/**`，不要重新恢复双壳并行目录。
- `apps/site/**` 当前是纯静态站点，不应为了官网再引入新的 npm 依赖或复杂构建栈；下载目标地址统一维护在 `apps/site/download/targets.js`。
- 根 `package.json` 现在只保留一套短入口（`dev/build/typecheck`）和显式命名空间（`desktop:*`、`site:*`、`test:*`）；不要再回填 `start/pack/dist` 这类纯重复别名。
- 修改 Tauri 前台时，同时考虑浏览器 preview fallback，避免只在原生 runtime 下可看。
- 新的 Tauri 前台默认以 Tailwind CSS v4 + shadcn 风格组件演进；新增交互应优先复用 `src/components/ui/*`，不要回退到整页手写样式类。
- 修改桌面端多语言时，只维护 `apps/desktop/src/locales/messages/*.json` 与 `apps/desktop/src/locales/config/*.json`；不要重新引入 `break-message-copy.*`、`overrides/` 或 `app/locales` 回灌。
- `coverage/`、`output/` 这类本地生成目录以及旧根 README 展示素材都不属于主体开发资产；需要时按命令重新生成，不要长期堆在仓库根目录。
- 根 `README.md` 现在只保留当前有效的运行/打包入口说明；历史根 README、社区治理文件、LICENSE 与 Linux 发布元数据的归档参考统一看 `docs/RootMetadataArchive.md`。
- 修改 Tauri host 时，优先从 `state.rs -> engine.rs -> platform.rs -> shell.rs -> commands.rs` 这条链路理解调度、系统信号和窗口行为。
- 修改 macOS 图标时，不要只替换 PNG：
- tray icon 应保持 template image 语义，并同时提供 1x / 2x 表示，优先保证小尺寸几何和安全边距，而不是把大图硬缩到菜单栏。
  - Dock icon 在正式 `.app` 中应优先使用 bundle 内的 `icon.icns`，运行时 patch 只作为未打包开发态的兜底。
  - `graphics/app-icon.svg` 需要保留足够安全边距；如果图标填满 1024 画布，Dock 中通常会比系统应用显得更大一圈。
- 设置项变更至少同步检查 `apps/desktop/src-tauri/src/state.rs`、`apps/desktop/src/App.tsx`、托盘/休息窗口使用点，以及 locale registry / 当前 `docs/` 说明。
- 当前工作区已有大量未提交改动；初始化和后续开发都应避免回滚与当前任务无关的文件。
