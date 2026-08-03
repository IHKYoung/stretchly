# Task-ID: TID-20260803-break-ideas-doubling-management

> 若本任务不涉及架构/契约/数据/并发边界，可在此注明 `N/A` 并保持该文件存在（流程门禁要求）。  

## Goals
- 在不改变 break runtime 的前提下，把 official locale 的提示池精确翻倍。
- 为新增内容建立可追踪、可失败显性的批次边界，后续扩充可以复用同一套校验。
- 保持 `messages/*.json` 为唯一正文真源，避免治理元数据演变成第二套内容库。

## Non-Goals
- 不修改 prompt 选择、完整休息轮换、打字机或 60 秒停留逻辑。
- 不改写现有 `aaa..anz` / `aaa..ajj` 文案，不补 legacy locale。
- 不引入运行时类别、远程内容服务、机器翻译依赖或新的用户设置。

## Constraints & Assumptions
- official locale 固定为 `zh-CN`、`zh-TW`、`en`，三者 ID 与 entry shape 必须一致。
- 本批 micro ID 为 `aoa..bbz`，long ID 为 `ajk..ast`；旧 ID 不在 manifest 中重复登记。
- micro 只有 `text`；long 必须且只能有 `title`、`text`。
- 类别只用于编审统计和相邻内容交错，不参与运行时选择。
- 简繁基础转换可使用系统 ICU，但台湾用词仍需人工校正；英文独立本地化。

## System Boundaries
- Modules: `break-ideas/messages`（正文）、`break-ideas/batches`（治理元数据）、`sync_desktop_break_ideas.py`（校验与构建）、`registry.generated.json`（运行时产物）、`lib/break-ideas.ts`（只读消费者）。
- Ownership: 编辑事实由三份 official message bundle 持有；批次归属由 manifest 持有；生成器负责拒绝不一致；runtime 只展示生成后的 entry。
- Dependency direction: `messages + registry.json + batches -> sync generator -> registry.generated.json -> runtime`。

## API / Contract
- Signatures / Endpoints: 无网络 API；保留现有 `breakIdeaEntries`、`pickBreakPromptEntry`、`rotateBreakPromptEntries` 签名。
- Request/Response schema (typed): manifest 包含 `batchId/sourceLanguage/officialLanguages/categories/targets/limits/entries`；entry 元数据仅为 `{ id, category }`。
- Error model (codes, retryability): 生成器以非零退出显性报告 locale/kind/id/约束；修正源文件后重跑，不做 fallback 或静默删项。

## Data Model / Storage
- `messages/<locale>.json`：`miniBreakIdeas[id] = {text}`；`longBreakIdeas[id] = {title,text}`。
- `batches/<batch>.json`：记录 before/added/after、首尾 ID、长度预算、类别连续上限，以及本批 ID/category；不含任何用户可见正文。
- `registry.generated.json`：继续由脚本机械生成，不手工编辑。

## Invariants
- official locale 的两个 ID 集合分别完全一致，且 entry 字段集合严格匹配 kind。
- 本批 ID 连续、唯一、与目标首尾及 added 数一致，并存在于所有 official locale。
- 同一 locale/kind 内归一化后的可见文案不重复；字段非空且符合 locale 长度预算。
- 完整休息的 manifest 类别不得超过配置的连续上限，避免顺序轮播连续同质。
- legacy bundle 与本批之前的 official entries 字节级不变。

## Concurrency / Lifecycle / Memory Model
- 静态构建期任务，无并发状态；生成器完成全部校验后才写 registry。
- runtime 生命周期、随机起点、轮换顺序和内存模型均不变。

## Observability Plan (Debug-Driven)
- Logs: 生成成功时输出语言/bundle/batch 数；失败时输出具体约束位置。
- Metrics: official count、batch added count、类别分布、字段字符数 min/p50/p90/max、registry 字节数。
- Traces: N/A（无异步或网络链路）。
- Debug flags: 沿用 `?window=break` 本地预览入口。

## Security & Privacy Considerations
- 全部内容为仓库内静态文案，不读取用户数据、不发送网络请求、不包含凭据。

## Risks & Rollback
- Failure modes: 批量模板化、翻译腔、错误健康断言、ID 漏项/重复、正文过长、generated registry 膨胀。
- Rollback steps: 按 manifest ID 范围移除新增 entry，并回退 batch/README/validator/test/registry/docs；无需迁移用户状态。

## Acceptance Criteria (System)
- 三个 official locale 精确达到 micro `728`、long `488`，并通过 shape/key parity/duplicate/length/batch sequence 校验。
- 旧内容与 legacy bundle 不变，registry 重建幂等，现有 runtime tests 继续通过。

## Open Questions / Decision Requests
- 无阻塞决策；未来新批次是否继续同等规模，由用户在下一次扩充时决定。
