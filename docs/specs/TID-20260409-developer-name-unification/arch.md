# Task-ID: TID-20260409-developer-name-unification

## Goals
- 统一产品中的开发者显示名与元数据，从 `Jan Hovancik` 替换为 `Clarke Young`。
- 保持当前运行时行为、locale 结构与配置契约不变。

## Non-Goals
- 不修改许可证版权声明。
- 不调整项目 URL、邮箱、赞助链接或 bug tracker。
- 不在本任务内重命名 locale key，也不做新的 i18n 结构重构；后续 key rename 见 `TID-20260409-janh-key-rename`。

## Constraints & Assumptions
- 用户只给出了新的开发者姓名，未提供新的邮箱、主页或其他联系信息。
- 现有仓库已区分 `apps/desktop` 主链路和 `app/` 归档链路；本轮需要两边都统一显示名，避免用户 grep 时看到旧值。

## System Boundaries
- Modules:
  - `apps/desktop/src/locales/messages/*.json`
  - `app/locales/*.json`
  - `apps/desktop/src/locales/registry.generated.json`
  - `package.json`
  - `README.md`
  - `net.hovancik.Pauza.metainfo.xml`
- Ownership:
  - 仅更新显示名和元数据，不改变产品功能与运行时职责边界。
- Dependency direction:
  - locale source -> `registry.generated.json`
  - package/metainfo/README 为独立元数据出口，不反向影响运行时逻辑

## API / Contract
- Signatures / Endpoints:
  - N/A
- Request/Response schema (typed):
  - locale `preferences.about.clarkeY` 的显示值统一为 `Clarke Young`
- Error model (codes, retryability):
  - locale sync 若生成失败则直接中止并保留现有 registry

## Data Model / Storage
- 不新增字段；仅更新已有 locale value、package author、publisher display name 与 metainfo developer_name。

## Invariants
- 当时的 locale key 结构保持不变，只替换显示值；后续 key rename 见 `TID-20260409-janh-key-rename`。
- `LICENSE` 保持不动。
- 所有 desktop locale 统一重新生成 `registry.generated.json`。

## Concurrency / Lifecycle / Memory Model
- N/A，本任务不涉及并发或生命周期模型。

## Observability Plan (Debug-Driven)
- Logs:
  - 使用 `rg -n "Jan Hovancik|Clarke Young"` 审计替换范围
- Metrics:
  - N/A
- Traces:
  - N/A
- Debug flags:
  - N/A

## Security & Privacy Considerations
- 无新增权限、密钥、外部服务或用户数据处理。

## Risks & Rollback
- Failure modes:
  - locale 资源替换后 registry 未重生成，导致运行时仍显示旧值
  - 少量元数据出口未同步，仓库内仍残留旧显示名
- Rollback steps:
  - 回退 locale JSON、`package.json`、`README.md`、metainfo 与重新生成的 registry

## Acceptance Criteria (System)
- AC1: desktop locale source 与生成后的 registry 都不再显示 `Jan Hovancik` 作为当前开发者姓名。
- AC2: `package.json`、`README.md` 与 `net.hovancik.Pauza.metainfo.xml` 中的当前开发者显示名改为 `Clarke Young`。
- AC3: `LICENSE` 不被修改。

## Open Questions / Decision Requests
- 若后续需要统一邮箱、主页、赞助链接或版权归属，需要单独给出新的权威信息。
