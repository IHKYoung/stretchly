# Task-ID: TID-20260403-repo-cleanup-foundation

> 若本任务不涉及架构/契约/数据/并发边界，可在此注明 `N/A` 并保持该文件存在（流程门禁要求）。  

## Goals
- N/A。本任务不改变运行时架构、模块边界或数据契约。

## Non-Goals
- 不改动 `app/**` 核心逻辑。
- 不裁剪平台分发主路径或构建目标。

## Constraints & Assumptions
- 仅删除与产品主体无关的外围资产。
- 必须保证当前 Electron 主体代码与构建配置仍可继续演进。

## System Boundaries
- Modules: 仅涉及仓库外围文件与本地生成目录。
- Ownership: 不触碰 `app/**`、`test/**` 与 `package.json` 的行为语义。
- Dependency direction: 无变化。

## API / Contract
- N/A

## Data Model / Storage
- N/A

## Invariants
- 核心应用入口、运行路径和构建配置保持不变。
- 删除项必须满足“非核心、可恢复、无运行时引用”。

## Concurrency / Lifecycle / Memory Model
- N/A

## Observability Plan (Debug-Driven)
- Logs: 通过 daily plans/logs 记录删除边界与验证结果。
- Metrics: N/A
- Traces: N/A
- Debug flags: N/A

## Security & Privacy Considerations
- 不新增外部依赖、不引入新的网络面或权限面。

## Risks & Rollback
- Failure modes: 误删后续仍要用到的平台分发资产。
- Rollback steps: 从 VCS 恢复 tracked 文件；`coverage/` 等生成目录按需重新生成。

## Acceptance Criteria (System)
- 删除项不被当前产品主体直接引用。
- 删除后文档索引仍与仓库现状一致。

## Open Questions / Decision Requests
- 第二轮是否按 Apple-first 路线继续删除 Linux/Windows 分发资产，待技术路线决策。
