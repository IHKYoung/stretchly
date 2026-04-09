# Repository Guidelines

## 项目结构与模块组织
- `app/`：归档中的旧 Electron 壳，仅保留为历史参考；当前默认运行、构建、测试链路都不再依赖它。
- `apps/desktop/components.json`、`apps/desktop/src/components/ui/*`、`apps/desktop/src/lib/utils.ts`：Tauri 前台的 Tailwind/shadcn 组件基座。
- `apps/desktop/src/App.tsx`、`apps/desktop/src/styles.css`：新的 Tauri 设置工作台与 break prompt 视觉层。
- `apps/desktop/src-tauri/src/*.rs`：新的 Rust host，负责设置持久化、调度、系统信号、tray、shortcut、notification、窗口命令与运行时状态。
- `apps/desktop/src/locales/messages/*.json`、`apps/desktop/src/locales/config/*.json`：桌面端 locale 真源；`scripts/sync_desktop_locales.py` 会据此生成共享 `registry.generated.json`。
- `apps/desktop/src/locales/break-message-copy.json`：break 消息页专属提示语的可编辑真源。
- `apps/desktop/legacy-utils/*.js`：已迁入 `apps/desktop` 的 JS 领域/兼容辅助模块，供根级 Vitest 与迁移期 source basis 使用。
- `test/*.js`：Vitest 测试集，按功能模块拆分；调整核心行为时优先补同名或相邻测试文件。
- `build/`、`graphics/`、`examples/`：安装包资源、图标源文件与平台集成样例。
- `docs/`：Codex 工作流文档、变更日志与项目理解资产；后续个性化开发建议持续维护这里的索引。
- `scripts/`、`.githooks/`：本仓库的工作流校验、任务脚手架与 Git hooks。

## 构建、测试与开发命令
- `npm start`：启动 `apps/desktop` 的 Tauri 桌面端。
- `npm run dev`：同 `npm start`，用于本地开发。
- `npm run desktop:install`：安装 `apps/desktop` 的前端与 Tauri 依赖。
- `npm run desktop:dev`：启动新的 Tauri 2 桌面壳（Vite `127.0.0.1:43179`，HMR `43180`）。
- `npm run desktop:build`：构建新的 Tauri 2 安装产物。
- `python3 graphics/generate_icon_assets.py`：根据 `graphics/app-icon.svg` 与 `graphics/tray-icon.svg` 重新生成 Tauri / packaging 图标资源；修改图标源文件后必须先跑这条命令。
- `npm test`：运行全部 Vitest 测试。
- `npm run coverage`：运行测试并按需生成覆盖率产物到 `coverage/`。
- `npm run lint`：执行 Standard 风格检查。
- `npm run pack`：当前等价于 `npm run desktop:build`。
- `npm run dist`：当前等价于 `npm run desktop:build`。
- `python3 scripts/ensure_workflow_ready.py --target . --hooks required`：补齐并校验 Codex 工作流资产。
- `python3 scripts/sync_desktop_locales.py`：从 `apps/desktop/src/locales/{messages,config}` 生成并校验桌面端共享 `registry.generated.json`。
- `python3 scripts/validate_workflow_docs.py`：检查 docs/specs、plans、logs 的门禁字段。
- `python3 scripts/validate_agent_configs.py`：检查多 Agent 配置；当前仓库无多 Agent 配置时会跳过。

## 开发约束与建议
- 代码主体使用 ESM 风格的原生 JavaScript；除 `vitest.config.ts` 外没有前端框架或 TypeScript 业务层。
- `apps/desktop` 是单一真源；新的桌面端能力、构建链路和测试依赖都应收口到这里，不要继续把 `app/**` 接回默认链路。
- 如果旧 `app/**` 里还有要保留的行为，先迁入 `apps/desktop/src-tauri/**`、`apps/desktop/src/**` 或 `apps/desktop/legacy-utils/**`，再继续修改。
- 修改 Tauri 前台时，同时考虑浏览器 preview fallback，避免只在原生 runtime 下可看。
- 新的 Tauri 前台默认以 Tailwind CSS v4 + shadcn 风格组件演进；新增交互应优先复用 `src/components/ui/*`，不要回退到整页手写样式类。
- 修改桌面端多语言时，只维护 `apps/desktop/src/locales/messages/*.json`、`apps/desktop/src/locales/config/*.json` 和 `apps/desktop/src/locales/break-message-copy.json`；不要重新引入 `overrides/` 或 `app/locales` 回灌。
- 修改 Tauri host 时，优先从 `state.rs -> engine.rs -> platform.rs -> shell.rs -> commands.rs` 这条链路理解调度、系统信号和窗口行为。
- 修改 macOS 图标时，不要只替换 PNG：
  - tray icon 应保持 template image 语义，并同时提供 1x / 2x 表示，优先保证小尺寸几何和安全边距，而不是把大图硬缩到菜单栏。
  - Dock icon 在正式 `.app` 中应优先使用 bundle 内的 `icon.icns`，运行时 patch 只作为未打包开发态的兜底。
  - `graphics/app-icon.svg` 需要保留足够安全边距；如果图标填满 1024 画布，Dock 中通常会比系统应用显得更大一圈。
- 设置项变更至少同步检查 `apps/desktop/src-tauri/src/state.rs`、`apps/desktop/src/App.tsx`、`apps/desktop/legacy-utils/defaultSettings.js`、托盘/休息窗口使用点，以及 locale registry / README / 文档说明。
- 当前工作区已有大量未提交改动；初始化和后续开发都应避免回滚与当前任务无关的文件。
