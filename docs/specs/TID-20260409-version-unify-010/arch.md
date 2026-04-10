# Task-ID: TID-20260409-version-unify-010

> 若本任务不涉及架构/契约/数据/并发边界，可在此注明 `N/A` 并保持该文件存在（流程门禁要求）。

## Goals
- 将当前仍然生效的产品/仓库版本入口统一到 `0.1.0`。

## Non-Goals
- 不修改历史 release 记录、archive 文档或依赖包自身版本。
- 不改 `apps/desktop/package.json`、`tauri.conf.json`、`Cargo.toml` 中已经正确的 `0.1.0`。

## Constraints & Assumptions
- 当前有效版本真源以根 `package.json`、根 `package-lock.json` 与 desktop / tauri / cargo 的版本字段为准。
- 历史 release 文本中的 `1.20.0` 不属于当前产品版本真源。

## System Boundaries
- Modules:
  - `package.json`
  - `package-lock.json`
  - 本任务 docs
- Ownership:
  - 根 package 元数据承接仓库级 `npm test` / `npm run build` 的显示版本与打包入口版本
- Dependency direction:
  - 根 `package.json` / `package-lock.json` 版本应与当前 desktop / tauri / cargo 版本一致

## API / Contract
- Signatures / Endpoints: N/A
- Request/Response schema (typed): N/A
- Error model (codes, retryability): N/A

## Data Model / Storage
- 不涉及数据模型或持久化迁移。

## Invariants
- 当前有效版本号统一为 `0.1.0`。
- 不改历史 release 记录。

## Concurrency / Lifecycle / Memory Model
- N/A

## Observability Plan (Debug-Driven)
- Logs:
  - 通过 grep、`npm test`、build 与 docs validator 输出确认版本统一结果。
- Metrics:
  - N/A
- Traces:
  - N/A
- Debug flags:
  - N/A

## Security & Privacy Considerations
- 仅修改版本元数据，不触碰用户数据、权限或外部系统。

## Risks & Rollback
- Failure modes:
  - 根 package 与 lock 版本不同步。
  - 仍有当前有效版本出口保留旧值。
- Rollback steps:
  - 回退 `package.json`、`package-lock.json` 与本任务 docs，然后重跑验证。

## Acceptance Criteria (System)
- 根 `package.json` 与 `package-lock.json` 顶层版本为 `0.1.0`。
- `apps/desktop/package.json`、`apps/desktop/src-tauri/tauri.conf.json`、`apps/desktop/src-tauri/Cargo.toml` 保持 `0.1.0`。
- `npm test`、`npm --prefix apps/desktop run build` 与 docs validator 通过。

## Open Questions / Decision Requests
- 无。
