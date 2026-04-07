# Task-ID: TID-20260403-settings-sidebar-layout

## Test Strategy
- Unit:
  - 无新增独立纯函数测试；本轮以现有前台类型检查和构建作为主验证
- Integration:
  - 通过 `App.tsx` 在 preview / build 链路中验证侧边栏分类切换、设置编辑和 save rail 未破坏编译
- E2E (if applicable):
  - 如时间允许，用浏览器预览采集主设置页成功态截图或 snapshot

## Acceptance Criteria Coverage Map (AC → Tests)
- AC1 -> 浏览器预览 / 代码审查确认存在 sidebar category 导航与分类切换；必要时采集截图
- AC2 -> 对照 `PauzaSettings` 字段与页面分类，代码审查确认字段未丢失；`npm --prefix apps/desktop run typecheck`
- AC3 -> 浏览器预览确认 dirty/synced、save/revert/defaults 仍存在；`npm --prefix apps/desktop run build`
- AC4 -> `npm --prefix apps/desktop run typecheck`；`npm --prefix apps/desktop run build`

## Interaction Contract Coverage
- Interaction impact: direct
- Primary flow -> tests/evidence: 主设置页打开后，通过 sidebar 切换分类并编辑设置，使用 UI 自检 + screenshot/snapshot
- Fallback / secondary flow -> tests/evidence: 缩窄窗口后的堆叠布局使用浏览器视口调整快速检查
- Visible states / transitions -> tests/evidence: dirty/synced/save disabled 通过 preview 状态和代码审查确认

## Governance Gates
- Agent Config Validation: `python3 scripts/validate_agent_configs.py`
- Workflow Docs Validation: `python3 scripts/validate_workflow_docs.py --mode manual`
- Approval Escalation Owner: orchestrator

## False-pass Cases
- sidebar 只是静态样式，没有真正的分类切换或 active 状态。
- 为了填充版面额外新增当前 host 不支持的设置项。
- save rail 或 defaults/revert 在新布局中被移除或不可达。
- 只改视觉，没有检查 `typecheck/build`。

## Evidence Capture (UI / E2E)
- Required: partial
- Owner: orchestrator
- Artifacts path: docs/specs/TID-20260403-settings-sidebar-layout/evidence/
- What to capture:
  - Screenshots: 主设置页侧边栏布局成功态
  - Video/trace (optional): N/A
  - HAR/console logs (optional): 如需要可补 browser console

## Quality Gates (Non-functional)
- a11y:
  - sidebar 用 button 语义；现有控件保留键盘可达性
- perf budget:
  - 不新增依赖，不引入明显更重的渲染路径
- error handling / observability:
  - error banner 保留
- security / privacy:
  - 无新增外部请求或敏感数据处理

## Boundary / Invalid Input Cases
- `showBreaksOnAllScreens=true` 时目标屏幕选择不应误显示
- `dirty=false` 时保存按钮必须禁用
- 语言切换后 sidebar 分类与标题仍能显示完整

## Concurrency / Race Cases (if applicable)
- `snapshot` 定时刷新期间，`dirty=true` 时不应覆盖本地 form 编辑态

## Mocks & Test Data
- 使用浏览器 preview 的默认 `previewSnapshot()`

## Commands to Run
- `npm --prefix apps/desktop run typecheck`
- `npm --prefix apps/desktop run build`
- `python3 scripts/validate_workflow_docs.py --mode manual`

## Expected Results
- PASS criteria:
  - typecheck/build 通过
  - 主设置页呈现 sidebar category 布局
  - 当前设置项仍完整存在且可编辑
- Outputs to keep (10~20 lines snippet):
  - build/typecheck 关键通过输出
