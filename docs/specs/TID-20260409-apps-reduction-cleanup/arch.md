# Task-ID: TID-20260409-apps-reduction-cleanup

> 若本任务不涉及架构/契约/数据/并发边界，可在此注明 `N/A` 并保持该文件存在（流程门禁要求）。  

## Goals
- 移除不再参与当前产品运行、构建、测试或日常开发判断的迁移层、示例目录、生成目录与 README 展示素材。
- 将根级社区治理、法律与 Linux 发布元数据抽取到 `docs/` 后再清理根文件。
- 保持 `apps/desktop` 作为唯一有效桌面端实现，并保留 `app/` 作为原版只读参考目录。

## Non-Goals
- 不改动当前 Tauri runtime 行为、Rust host 状态机或桌面端 locale 结构。
- 不删除 `app/` 原版参考代码。
- 不清理 `build/`、`graphics/` 等当前打包/图标真源。

## Constraints & Assumptions
- `apps/desktop/legacy-utils` 已不在当前运行时链路中，只被根级旧测试消费。
- `coverage/`、`output/` 属于可再生本地目录，不应被当成长期仓库资产。
- 根目录旧 PNG 仅承担历史 README 展示用途或可再生图标副产物，不是主体开发依赖。

## System Boundaries
- Modules:
  - `apps/desktop/**`
  - `app/**`
  - `test/**`
  - 根 `README.md`
  - `docs/{RepositoryGuidelines,CodeMap,CHANGELOG}.md`
- Ownership:
  - 当前产品实现归 `apps/desktop/**`
  - 原版参考归 `app/**`
  - 仓库索引与说明归 `docs/**`、`README.md`
- Dependency direction:
  - 默认开发链路只能从根 scripts 指向 `apps/desktop`
  - `app/**` 仅允许被人工阅读参考，不应再被默认测试/构建链路依赖
  - 根级 README / governance / release metadata 只要不进入默认开发链路，就应沉淀到 `docs/` 而不是继续散落在仓库根目录

## API / Contract
- Signatures / Endpoints: N/A，本任务不引入新的运行时 API 或外部契约。
- Request/Response schema (typed): N/A
- Error model (codes, retryability): 仅存在构建/测试/文档 gate 失败这类仓库级失败，不涉及产品对外错误模型。

## Data Model / Storage
- 不涉及数据模型或持久化迁移。

## Invariants
- `apps/desktop` 仍是唯一有效桌面端真源。
- `app/` 仍保留为原版参考目录。
- 当前打包图标链 `graphics/* -> apps/desktop/src-tauri/icons/* / build/*` 不受本轮清理影响。

## Concurrency / Lifecycle / Memory Model
- N/A，本任务仅做仓库结构减法，不涉及并发或运行时生命周期调整。

## Observability Plan (Debug-Driven)
- Logs:
  - 通过 `git diff --name-status`、`rg` 审计与 `npm test` 输出确认删除范围没有误伤当前链路。
- Metrics:
  - N/A
- Traces:
  - N/A
- Debug flags:
  - N/A

## Security & Privacy Considerations
- 仅删除非运行时仓库资产，不触碰用户数据、系统权限、外部服务或密钥。

## Risks & Rollback
- Failure modes:
  - 仍有根级脚本、测试或文档引用已删除目录。
  - README 重写过度，遗漏当前有效开发入口说明。
- Rollback steps:
  - 从 VCS 恢复 `apps/desktop/legacy-utils`、旧测试、`examples/`、根素材与 README/docs。
  - 重新执行 `npm test`、`npm --prefix apps/desktop run build` 与 docs gate 验证回退结果。

## Acceptance Criteria (System)
- `apps/desktop/legacy-utils` 与依赖它的根级旧测试退出默认链路。
- `coverage/`、`examples/`、`output/` 与根 README 展示素材已删除。
- 根级 `README.md`、`CONTRIBUTING.md`、`CODE_OF_CONDUCT.md`、`LICENSE`、`net.hovancik.Pauza.desktop`、`net.hovancik.Pauza.metainfo.xml` 已退出根目录，其有用信息已归档到 `docs/RootMetadataArchive.md`。
- `app/` 保留，且当前 docs 明确它仅是原版参考目录。
- 当前桌面端 typecheck/build/test 与 docs validator 均通过。

## Open Questions / Decision Requests
- 无；本轮范围由用户明确授权。
