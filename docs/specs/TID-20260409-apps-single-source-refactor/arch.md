# Task-ID: TID-20260409-apps-single-source-refactor

## Goals
- 将桌面端默认运行、构建、测试和 locale 生成链路统一收口到 `apps/desktop`。
- 停止 `app/` 目录对当前桌面端的默认参与，只保留归档参考语义。
- 去掉 locale `overrides` 叠加模式，改为单层 `messages + config` 真源。

## Non-Goals
- 本轮不物理删除 `app/` 目录。
- 本轮不重写 Rust 调度状态机或 React 交互语义。
- 本轮不新增依赖，也不恢复 Electron 默认入口。

## Constraints & Assumptions
- 当前工作区存在大量未提交改动，不能用破坏性删除来“证明”重构完成。
- 必须保留现有桌面端功能与测试能力，不能因为目录迁移让 `npm test` 或 Tauri 构建断掉。
- break 页面专属提示语仍需要独立维护入口，但必须放在 `apps/desktop` 体系内。

## System Boundaries
- Modules:
  - `package.json`
  - `apps/desktop/src/**`
  - `apps/desktop/src-tauri/src/**`
  - `apps/desktop/src/locales/**`
  - `apps/desktop/legacy-utils/**`
  - `test/**`
  - `scripts/sync_desktop_locales.py`
  - 相关 README / docs / workflow 文档
- Ownership:
  - 桌面端运行时、文案、测试辅助与迁移兼容模块统一归 `apps/desktop/**` 管理。
  - `app/**` 仅保留只读参考语义，不再承担当前桌面端职责。
- Dependency direction:
  - 根 scripts/tests -> `apps/desktop/**`
  - locale generator -> `apps/desktop/src/locales/{messages,config}`
  - 前端 / Rust host -> `registry.generated.json`
  - 不允许新的默认链路反向依赖 `app/**`

## API / Contract
- Signatures / Endpoints:
  - `scripts/sync_desktop_locales.py` 输出固定结构的 `registry.generated.json`
  - `break-message-copy.ts` 只负责读取 `break-message-copy.json`
- Request/Response schema (typed):
  - `registry.generated.json = { defaultLanguage, languages, bundles }`
  - `break-message-copy.json = Record<language, BreakMessageCopy>`
- Error model (codes, retryability):
  - 缺失 message/config 对应关系时 locale sync 直接失败并退出
  - 缺失 break copy 语言时运行时回退到默认语言 `zh-CN`

## Data Model / Storage
- locale registry 由 `messages/*.json` 与 `config/*.json` 生成，不再经过 override merge。
- break 消息页专属提示语独立存储在 `break-message-copy.json`。
- 根级 Vitest 所需的 JS 领域辅助模块落在 `apps/desktop/legacy-utils/**`，不再从 `app/utils/**` 读取。

## Invariants
- 根 `package.json` 不再把 `app/main.js` 作为默认入口。
- 根级测试不再 import `app/utils/**` 或 `app/locales/**`。
- 桌面端 locale 生成脚本不再读取 `app/locales`、`app/preferences.html` 或 `overrides/`。
- `apps/desktop` 是当前桌面端唯一有效的源码真源。

## Concurrency / Lifecycle / Memory Model
- 本任务主要是源码归属与构建链路重构，不引入新的并发模型。
- 运行时生命周期仍沿用现有 Tauri host；本轮只调整其输入资产和仓库级引用边界。

## Observability Plan (Debug-Driven)
- Logs:
  - 使用 `rg` 审计剩余 `app/` / `overrides` 引用
  - 通过 locale sync 的显式失败消息暴露 message/config 漏配
- Metrics:
  - N/A
- Traces:
  - N/A
- Debug flags:
  - N/A

## Security & Privacy Considerations
- 本任务只改本地代码、测试和文档，不涉及新增外部服务、权限、敏感数据或网络密钥。

## Risks & Rollback
- Failure modes:
  - 根级测试引用迁移后若路径不一致，`npm test` 会立即失败。
  - locale source 扁平化后若 message/config 不匹配，`registry.generated.json` 无法生成。
  - 文档口径若未同步，仓库会继续表现为“架构已改，说明还停在旧世界”。
- Rollback steps:
  - 回退 `package.json`、`apps/desktop/**`、`test/**`、`scripts/sync_desktop_locales.py` 与相关 docs。
  - 重新生成 locale registry 并跑完 typecheck/build/test 后再确认回滚结束。

## Acceptance Criteria (System)
- AC1: 根运行/构建/测试链路不再依赖 `app/main.js`、`app/utils/**` 或 `app/locales/**`。
- AC2: 桌面端 locale registry 只由 `apps/desktop/src/locales/{messages,config}` 生成，`overrides/` 退出默认架构。
- AC3: break 消息页提示语可在 `apps/desktop/src/locales/break-message-copy.json` 中集中编辑。
- AC4: 仓库主文档与架构索引明确声明 `apps/desktop` 是当前单一真源。

## Open Questions / Decision Requests
- 当工作区清理完毕后，是否要继续做物理层面的 `app/` 删除和历史资源清仓。
