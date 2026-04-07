# Task-ID: TID-20260403-settings-fixed-split

## Goals
- 用最小改动修正主设置页的全局布局边界，让 split ratio 由外层 shell 保证，而不是靠局部卡片挤压。
- 通过 Tauri 主窗口尺寸配置保证桌面交付态稳定处于适合 split view 的可用区间。
- 通过 desktop primitives 与设置页局部组件一起收紧圆角，避免只修一处、其余风格继续漂移。

## Non-Goals
- 不改变 `get_snapshot` / `update_settings` / `toggle_autostart` 等既有命令契约。
- 不改变 `PauzaSettings` 或 `DesktopSnapshot` 数据结构。
- 不改变 break prompt 或 host runtime。

## Constraints & Assumptions
- 只允许在 `apps/desktop/src/App.tsx`、`apps/desktop/src/components/ui/*` 与 `apps/desktop/src-tauri/tauri.conf.json` 内完成本轮核心实现。
- 现有 sidebar category、overview cards 和 save actions 需要原样保留语义。
- 窄屏堆叠不再是桌面主窗口的交付目标，因此可通过 min window size 解决。

## System Boundaries
- Modules:
  - `apps/desktop/src/App.tsx`：拥有前台 split layout、hero 收口与 save/status dock 的视觉组织
  - `apps/desktop/src/components/ui/*`：拥有 desktop 前台公共圆角和表单基元的默认风格
  - `apps/desktop/src-tauri/tauri.conf.json`：拥有主窗口默认尺寸与最小尺寸约束
- Ownership:
  - React 前台负责分栏、内容组织与克制化呈现；Tauri config 负责窗口物理尺寸；host commands 保持原边界不动
- Dependency direction:
  - `tauri.conf.json` -> 主窗口尺寸边界
  - `components/ui/*` -> `App.tsx` 的视觉基元
  - `App.tsx` -> 消费既有 snapshot / update commands，不反向修改 host schema

## API / Contract
- Signatures / Endpoints:
  - 无新增 API；继续使用既有 `invoke('get_snapshot')`、`invoke('update_settings')` 等命令
- Request/Response schema (typed):
  - `DesktopSnapshot` / `PauzaSettings` 结构不变
- Error model (codes, retryability):
  - 无新增错误码；前台继续以 error banner 暴露失败

## Data Model / Storage
- 无数据模型变更，无存储迁移。

## Invariants
- 设置字段不能丢失，也不能因为布局调整而被移到不可达位置。
- 保存/撤销/默认值动作必须仍然在主设置页主路径中可见。
- 桌面主窗口在默认与最小尺寸下都必须落入 split view。
- 收紧圆角和裁掉介绍文案不能影响现有控件语义与操作链路。

## Concurrency / Lifecycle / Memory Model
- 无新增并发或生命周期模型；仍沿用现有 snapshot 拉取与表单脏状态同步逻辑。

## Observability Plan (Debug-Driven)
- Logs:
  - `npm --prefix apps/desktop run typecheck`
  - `npm --prefix apps/desktop run build`
  - 浏览器 preview console log
- Metrics:
  - N/A
- Traces:
  - N/A
- Debug flags:
  - 本地 Vite preview 作为视觉调试入口

## Security & Privacy Considerations
- 无新增权限、隐私字段或网络面。

## Risks & Rollback
- Failure modes:
  - 提高最小窗口尺寸后，用户无法再把主窗口压缩到旧尺寸
  - save/status dock 并回主体后，顶部信息可能略显密集
  - 全局收紧圆角后，若 break prompt 或其他面板显得过硬，需要后续单独校正
- Rollback steps:
  - 回退 `App.tsx` 的双栏 shell
  - 回退 `components/ui/*` 的圆角调整
  - 回退 `tauri.conf.json` 的 `1440x810 / 1280x720` 尺寸配置

## Acceptance Criteria (System)
- 主设置页的全局布局只保留两列：sidebar 与 main column。
- `tauri.conf.json` 的主窗口尺寸改为接近 `16:9` 的默认/最小值。
- desktop 前台 primitives 不再默认使用高圆角 / 全 pill 风格。
- 无需改动 host contract 即可通过现有构建验证。

## Open Questions / Decision Requests
- 当前无需要用户额外决策的系统级问题。
