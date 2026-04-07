# Task-ID: TID-20260403-hig-tailwind-redesign

## Goals
- 以最小后端影响完成前端视觉架构迁移：从单文件手写 CSS 体系迁到 Tailwind CSS v4 + shadcn 风格 primitives。
- 保留 `DesktopSnapshot` / `PauzaSettings` 和现有 Tauri commands，避免 UI 改版带来契约漂移。

## Non-Goals
- 不调整 Rust host 数据模型、调度状态机和窗口生命周期逻辑。
- 不新增前端全局状态库、设计 token 生成器或复杂主题系统。

## Constraints & Assumptions
- `App.tsx` 仍是前台编排入口，但组件 primitive 可以下沉到 `src/components/ui/*`。
- `?window=break` 仍沿用同一个 React 入口，通过 query param 区分渲染模式。
- 用户显式要求使用 Tailwind + shadcn，因此允许新增前端依赖。

## System Boundaries
- Modules:
  - `src/App.tsx`：组合主设置页与 break prompt
  - `src/components/ui/*`：Button/Card/Switch/Input/Textarea/Accordion/Select/Badge/SegmentedControl primitives
  - `src/styles.css`：Tailwind v4 tokens、base layer 与动效
  - `src/i18n.ts` + `src/locales/*`：文案 lookup 与时间格式化
- Ownership:
  - 仅 React/Tailwind 视觉层
- Dependency direction:
  - UI primitives -> `cn` utility
  - `App.tsx` -> UI primitives + `i18n` + `@tauri-apps/api/core`
  - 不反向依赖 Rust host 实现细节

## API / Contract
- Signatures / Endpoints:
  - `get_snapshot`
  - `update_settings`
  - `pause_breaks`
  - `resume_breaks`
  - `start_focus_session`
  - `clear_focus_session`
  - `toggle_autostart`
  - `finish_current_break`
  - `skip_current_break`
  - `postpone_current_break`
- Request/Response schema (typed):
  - 前台继续消费 `DesktopSnapshot`
  - 保存继续提交 `PauzaSettings`
- Error model (codes, retryability):
  - 本轮不新增 error code；Tauri invoke 失败时直接显示 `error banner`

## Data Model / Storage
- 数据结构保持不变：
  - `PauzaSettings`
  - `DesktopSnapshot`
  - `CurrentBreakSnapshot`
- 未新增本地存储键、缓存层或 schema migration

## Invariants
- 前台不是运行时真源；真源仍在 Tauri host
- `dirty=false` 时表单必须被外部 `snapshot.settings` 回填
- break window 仍由 URL query 决定，不额外引入路由系统

## Concurrency / Lifecycle / Memory Model
- `useEffect` 每 1s 拉取 `get_snapshot`
- `dirty` 作为本地编辑闸门，避免 polling 覆写用户未保存输入
- preview 模式继续用 `previewTransform` 做即时 UI 回显，不依赖额外 mock store

## Observability Plan (Debug-Driven)
- Logs:
  - `npm --prefix apps/desktop run build`
  - `docs/specs/TID-20260403-hig-tailwind-redesign/evidence/browser-console-errors.log`
- Metrics:
  - 无新增
- Traces:
  - 无新增
- Debug flags:
  - 浏览器 preview
  - Playwright mocked `__TAURI_INTERNALS__`

## Security & Privacy Considerations
- 无新增远程请求
- mocked runtime 仅用于浏览器证据采集，不进入生产构建
- 未新增敏感数据持久化或系统权限请求

## Risks & Rollback
- Failure modes:
  - Tailwind/shadcn 引入样式回归或 bundle 变大
  - mocked break prompt 与真实 Tauri runtime 细节不完全一致
- Rollback steps:
  - 回退 `apps/desktop/**` 本轮修改
  - 移除新依赖与 `components.json`
  - 恢复旧 `App.tsx` / `styles.css`

## Acceptance Criteria (System)
- Tauri command contract 不变
- `npm --prefix apps/desktop run build` 通过
- preview 模式与 mocked break evidence 都能渲染最终 UI

## Open Questions / Decision Requests
- 无
