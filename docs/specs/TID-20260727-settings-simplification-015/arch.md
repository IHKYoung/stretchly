# Task-ID: TID-20260727-settings-simplification-015

> 若本任务不涉及架构/契约/数据/并发边界，可在此注明 `N/A` 并保持该文件存在（流程门禁要求）。

## Goals
- 保持后端设置契约不扩张的前提下，降低设置页默认决策数量，并让 profile、持久化默认值与调度测试一致。
- 明确提示语轮换的 UI wait 归属和已有配置兼容策略。

## Non-Goals
- 不新增设置字段、不迁移已有 `settings.json`、不改 smart reminder 的等待/超时模型。
- 不执行 DMG 打包、签名、公证、tag、push 或 GitHub Release。

## Constraints & Assumptions
- `DesktopSnapshot.settings` / Rust `PauzaSettings` 是设置事实源；React route 与 typewriter timeline 只负责展示。
- 已有用户的旧默认节奏可能显示为“自定义”，但值不得被静默改写。
- 精确档位是有依据的产品强度选择，不宣称为普适医疗处方。

## System Boundaries
- Modules: `App.tsx` 设置路由与 break prompt；`settings-controls.ts` profile；`break-ideas.ts` 轮换常量；Rust settings/runtime；locale registry。
- Ownership: Rust 持久化与调度，React 导航/呈现，locale JSON 文案，生成 registry 负责前后端共享。
- Dependency direction: locale/profile 常量 -> React 展示与提交；Rust defaults -> snapshot/runtime；前端不伪造后端运行进度。

## API / Contract
- Signatures / Endpoints: 复用 `get_snapshot`、`update_settings` 与现有 tray/runtime commands，无新 endpoint。
- Request/Response schema (typed): `PauzaSettings` 字段不变；profile 只 patch 六个节奏字段。
- Error model (codes, retryability): autosave 仍显式展示保存错误；本轮不改变串行/合并重试语义。

## Data Model / Storage
- schema 不变；新安装默认 `microbreak_interval_minutes=20`，完整休息仍为每 3 轮、5 分钟。已存在文件按原值载入。

## Invariants
- 三档完整休息周期均为约 60 分钟。
- 从任意主题页返回都到 `overview`；五个主题都可从首页直接进入。
- 微休息 prompt sequence 只有一条；完整休息 hold 从 title/body 全部打完后才开始计时。

## Concurrency / Lifecycle / Memory Model
- typewriter timer 由 `BreakWindow` effect 持有，route/break key 变化时清理所有 timeout；完整呈现后等待 60 秒，再进入 520ms 空白切换。

## Observability Plan (Debug-Driven)
- Logs: 前端测试、Rust 测试、Vite build 与 workflow validator 输出。
- Metrics: profile 精确值、默认 next-break due、测试计数。
- Traces: 本地 HTTP 200；浏览器后端不可用显式记录。
- Debug flags: 浏览器 preview 继续复用 `?window=break` 与 preview snapshot。

## Security & Privacy Considerations
- 无新网络请求、依赖、权限、凭据或用户数据字段。

## Risks & Rollback
- Failure modes: 旧配置被误迁移、详情入口丢失、轮换在打字结束前计时、版本真源漂移。
- Rollback steps: 对本提交执行普通 `git revert`；持久化 schema 未变，无数据回滚步骤。

## Acceptance Criteria (System)
- 版本真源一致为 `0.1.5`，profile/default/runtime 测试一致，locale generators 幂等，前端/Rust 测试与 build 通过。

## Open Questions / Decision Requests
- 后续是否构建并公开发布 `0.1.5` 需用户另行授权。
