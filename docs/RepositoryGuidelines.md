# Repository Guidelines

## 项目结构与模块组织
- `app/main.js`：旧 Electron 主进程入口，当前仍可运行，但后续只作为领域与兼容行为参考。
- `app/*.html`、`app/*-renderer.js`、`app/*-preload.mjs`：旧 Electron 界面链路。
- `app/utils/*.js`：后续迁移到 Rust host 前最重要的领域逻辑 source basis。
- `apps/desktop/components.json`、`apps/desktop/src/components/ui/*`、`apps/desktop/src/lib/utils.ts`：Tauri 前台的 Tailwind/shadcn 组件基座。
- `apps/desktop/src/App.tsx`、`apps/desktop/src/styles.css`：新的 Tauri 设置工作台与 break prompt 视觉层。
- `apps/desktop/src-tauri/src/*.rs`：新的 Rust host，负责设置持久化、调度、系统信号、tray、shortcut、notification、窗口命令与运行时状态。
- `app/locales/*.json`：i18next 文案资源；新增文案或设置项时需要同步校验多语言覆盖。
- `test/*.js`：Vitest 测试集，按功能模块拆分；调整核心行为时优先补同名或相邻测试文件。
- `build/`、`graphics/`、`examples/`：安装包资源、图标源文件与平台集成样例。
- `docs/`：Codex 工作流文档、变更日志与项目理解资产；后续个性化开发建议持续维护这里的索引。
- `scripts/`、`.githooks/`：本仓库的工作流校验、任务脚手架与 Git hooks。

## 构建、测试与开发命令
- `npm start`：以 Electron 正常模式启动应用。
- `npm run dev`：开发模式启动，附带日志与 `9222` 远程调试端口。
- `npm run desktop:install`：安装 `apps/desktop` 的前端与 Tauri 依赖。
- `npm run desktop:dev`：启动新的 Tauri 2 桌面壳（Vite `127.0.0.1:43179`，HMR `43180`）。
- `npm run desktop:build`：构建新的 Tauri 2 安装产物。
- `python3 graphics/generate_icon_assets.py`：根据 `graphics/app-icon.svg` 与 `graphics/tray-icon.svg` 重新生成 Tauri / packaging 图标资源；修改图标源文件后必须先跑这条命令。
- `npm test`：运行全部 Vitest 测试。
- `npm run coverage`：运行测试并按需生成覆盖率产物到 `coverage/`。
- `npm run lint`：执行 Standard 风格检查。
- `npm run pack`：用 `electron-builder` 生成未打包目录。
- `npm run dist`：生成正式安装包产物。
- `python3 scripts/ensure_workflow_ready.py --target . --hooks required`：补齐并校验 Codex 工作流资产。
- `python3 scripts/validate_workflow_docs.py`：检查 docs/specs、plans、logs 的门禁字段。
- `python3 scripts/validate_agent_configs.py`：检查多 Agent 配置；当前仓库无多 Agent 配置时会跳过。

## 开发约束与建议
- 代码主体使用 ESM 风格的原生 JavaScript；除 `vitest.config.ts` 外没有前端框架或 TypeScript 业务层。
- `apps/desktop` 是新的跨平台桌面壳起点；除非在修旧问题，否则不要继续为 Electron 主进程堆新的宿主层能力。
- 修改休息节奏、暂停逻辑或托盘行为前，先阅读 `app/main.js`、`app/breaksPlanner.js` 以及对应 `app/utils/*`，再决定迁移到 Rust host 的边界。
- 修改窗口 UI 时，按 `html -> preload -> renderer -> main IPC` 这条链路排查，不要只改渲染层。
- 修改 Tauri 前台时，同时考虑浏览器 preview fallback，避免只在原生 runtime 下可看。
- 新的 Tauri 前台默认以 Tailwind CSS v4 + shadcn 风格组件演进；新增交互应优先复用 `src/components/ui/*`，不要回退到整页手写样式类。
- 修改 Tauri host 时，优先从 `state.rs -> engine.rs -> platform.rs -> shell.rs -> commands.rs` 这条链路理解调度、系统信号和窗口行为。
- 修改 macOS 图标时，不要只替换 PNG：
  - tray icon 应保持 template image 语义，并同时提供 1x / 2x 表示，优先保证小尺寸几何和安全边距，而不是把大图硬缩到菜单栏。
  - Dock icon 在正式 `.app` 中应优先使用 bundle 内的 `icon.icns`，运行时 patch 只作为未打包开发态的兜底。
  - `graphics/app-icon.svg` 需要保留足够安全边距；如果图标填满 1024 画布，Dock 中通常会比系统应用显得更大一圈。
- 设置项变更至少同步检查 `app/utils/defaultSettings.js`、偏好页渲染逻辑、托盘/休息窗口使用点以及 README 中的配置说明。
- 当前工作区已有大量未提交改动；初始化和后续开发都应避免回滚与当前任务无关的文件。
