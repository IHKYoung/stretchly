# Task-ID: TID-20260408-break-prompt-parity-upgrade

> 若本任务不涉及架构/契约/数据/并发边界，可在此注明 `N/A` 并保持该文件存在（流程门禁要求）。  

## Goals
- 在不改 reminder / shell 状态机的前提下，把 break prompt 的体验配置重新收敛到 Tauri 当前的单一 settings/schema 实现里。

## Non-Goals
- 不新增 Tauri plugin 或外部依赖
- 不把本地图片路径直接暴露给 break window
- 不修改 engine、shell 和 break 生命周期状态机

## Constraints & Assumptions
- 当前壳未启用 `assetProtocol`，也未接入 dialog plugin
- break prompt 与偏好页都只消费同一个 `DesktopSnapshot`
- 自定义壁纸必须脱离“临时 object URL”并可跨会话持久化
- 音频播放保持前台一次性 effect，不引入后台播放器

## System Boundaries
- Modules:
  - `apps/desktop/src/App.tsx`：设置表单、break prompt UI、开始音 effect、自定义壁纸上传入口
  - `apps/desktop/src/lib/break-prompt.ts`：背景主题、cue 池、sound 映射、自定义壁纸压缩 helper
  - `apps/desktop/src-tauri/src/state.rs`：`PauzaSettings` 真源与 sanitize / migration
  - `apps/desktop/src/assets/audio/*.wav`：开始音资源
- Ownership:
  - 前台负责图片解码和压缩
  - Rust host 负责最终持久化和 snapshot 回传
- Dependency direction:
  - `state.rs` -> `DesktopSnapshot.settings`
  - `App.tsx` -> `break-prompt.ts`
  - `break-prompt.ts` -> bundled audio assets

## API / Contract
- Signatures / Endpoints:
  - `update_settings(settings: PauzaSettings) -> DesktopSnapshot`
  - `get_snapshot() -> DesktopSnapshot`
  - `prepareCustomBackdrop(file: File) -> Promise<string>`
- Request/Response schema (typed):
  - `PauzaSettings.breakBackdrop`
  - `PauzaSettings.breakCustomBackdropLabel`
  - `PauzaSettings.breakCustomBackdropDataUrl`
  - `PauzaSettings.breakIdeasEnabled`
  - `PauzaSettings.microbreakStartSound`
  - `PauzaSettings.longBreakStartSound`
  - `PauzaSettings.breakSoundVolume`
- Error model (codes, retryability):
  - 自定义壁纸解码失败时抛出前端普通错误提示
  - Rust 侧仍沿用 `Result<_, String>` 设置更新模型，无新增 error code

## Data Model / Storage
- `settings.json` 新增：
  - `breakBackdrop`
  - `breakCustomBackdropLabel`
  - `breakCustomBackdropDataUrl`
  - `breakIdeasEnabled`
  - `microbreakStartSound`
  - `longBreakStartSound`
  - `breakSoundVolume`
- legacy migration：
  - `ideas -> breakIdeasEnabled`
  - `miniBreakStartSound -> microbreakStartSound`
  - `volume(0~1 float) -> breakSoundVolume(0~100)`

## Invariants
- `breakSoundVolume` 始终限制在 `0..100`
- 当 `breakBackdrop == custom` 但没有可用图片数据时，设置自动回退到 `paper`
- 同一轮 break 的开始音只播放一次
- `silence` 必须稳定禁用开始音

## Concurrency / Lifecycle / Memory Model
- 自定义壁纸通过前端 `File -> Image -> Canvas -> data URL` 管线一次性处理，完成后释放 object URL
- break 开始音通过 `useEffect + useRef` 去重，按 `kind:startedAtMs` 只播放一次
- 本轮不改变 Tauri host 的线程模型和 break teardown 顺序

## Observability Plan (Debug-Driven)
- Logs:
  - 未新增 logger；若后续有问题，优先检查 `prepareCustomBackdrop()` 输出和 `state.rs::sanitized()` 回退逻辑
- Metrics:
  - N/A
- Traces:
  - N/A
- Debug flags:
  - N/A

## Security & Privacy Considerations
- 自定义壁纸只保存在用户本地 settings 文件中，不上传网络
- 不新增系统权限、远程资源拉取或外部服务调用
- data URL 方案避免了把本地绝对路径暴露给 break window 渲染层

## Risks & Rollback
- Failure modes:
  - 大图导致 settings 文件显著变大
  - 平台对自动播放更严格时，开始音可能被静默拒绝
  - 用户切到 custom 但清空图片后若没有回退逻辑，会出现空背景
- Rollback steps:
  - 回退 `App.tsx`、`break-prompt.ts`、`state.rs` 和 locale/doc 变更；必要时移除新增 settings 字段

## Acceptance Criteria (System)
- schema 可序列化 / 反序列化新字段且旧 settings 可迁移
- 自定义壁纸无需新依赖即可跨会话复用
- 前端 build、typecheck 与 Rust `cargo check` 通过

## Open Questions / Decision Requests
- 如果后续要支持“引用本地文件路径而非 data URL”，需要单独评估是否开启 `assetProtocol`、dialog plugin 与路径访问范围
