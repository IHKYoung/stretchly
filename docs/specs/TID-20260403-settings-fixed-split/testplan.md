# Task-ID: TID-20260403-settings-fixed-split

## Test Strategy
- Unit:
  - 不新增单元测试；本轮主要是前台布局、UI primitives 和窗口配置调整
- Integration:
  - `npm --prefix apps/desktop run typecheck`
  - `npm --prefix apps/desktop run build`
- E2E (if applicable):
  - 使用浏览器 preview + Playwright screenshot / snapshot 做界面旁证

## Acceptance Criteria Coverage Map (AC → Tests)
- AC1 -> `settings-fixed-split-1440x810-final.png` 与 `settings-fixed-snapshot-1440x810.md` 证明桌面交付态下形成双栏 split view
- AC2 -> `apps/desktop/src/App.tsx` 代码审查 + `settings-fixed-split-1440x810-final.png` 证明 save/status dock 并回主体，不再是全局第三列
- AC3 -> `apps/desktop/src-tauri/tauri.conf.json` 代码审查 + build 产物检查，确认默认/最小尺寸为 `1440x810` / `1280x720`
- AC4 -> `settings-fixed-split-1440x810-final.png`、`App.tsx` 与 `components/ui/*` 代码审查，确认 hero/section/sidebar 介绍被压缩、圆角整体收紧
- AC5 -> `npm --prefix apps/desktop run typecheck`、`npm --prefix apps/desktop run build` 与 browser console log

## Interaction Contract Coverage
- Interaction impact: direct
- Primary flow -> tests/evidence: `settings-fixed-split-1440x810-final.png` 展示 `sidebar -> main body -> save dock` 主路径
- Fallback / secondary flow -> tests/evidence: 通过 `tauri.conf.json` 的 `minWidth/minHeight` 保证桌面主窗口不进入窄屏堆叠；浏览器 preview 的窄视口回退仅作非阻塞开发旁证
- Visible states / transitions -> tests/evidence: 截图中可见 `category-active + synced badge`，DOM snapshot 保留结构旁证

## Governance Gates
- Agent Config Validation: `python3 scripts/validate_agent_configs.py`
- Workflow Docs Validation: `python3 scripts/validate_workflow_docs.py --mode manual`
- Approval Escalation Owner: orchestrator

## False-pass Cases
- `DONE` 任务对应的 spec 仍保留 `TBD/INIT` 占位。
- 界面虽然改成双栏，但 save/status dock 依旧以全局第三列形式存在。
- 只改了页面布局，没有同步提高主窗口最小尺寸，导致桌面交付态仍可跌回堆叠。
- 视觉收敛只改个别局部，`Card/Button/Input/Select/SegmentedControl` 仍保持旧的高圆角风格。

## Evidence Capture (UI / E2E)
- Required: partial
- Owner: orchestrator（single-agent-fallback）
- Artifacts path: docs/specs/TID-20260403-settings-fixed-split/evidence/
- What to capture:
  - Screenshots:
    - `settings-fixed-split-1440x810-final.png`
  - Video/trace (optional):
    - 不强制
  - HAR/console logs (optional):
    - `browser-console.log`
    - `settings-fixed-snapshot-1440x810.md`

## Quality Gates (Non-functional)
- a11y:
  - sidebar button 语义、button disabled 态和现有 form control focus ring 不回归
- perf budget:
  - 无新增前端依赖；build 通过即可
- error handling / observability:
  - 无新增错误码，浏览器 console 无新报错
- security / privacy:
  - 无新增权限、网络面或隐私字段

## Boundary / Invalid Input Cases
- 主窗口不能再被压到 `1280x720` 以下
- 浏览器 preview 小视口堆叠不作为桌面交付失败

## Concurrency / Race Cases (if applicable)
- 无新增并发风险，沿用既有 snapshot polling 与 dirty form 同步行为

## Mocks & Test Data
- 使用浏览器 preview 的默认 mock snapshot

## Commands to Run
- `npm --prefix apps/desktop run typecheck`
- `npm --prefix apps/desktop run build`
- `python3 scripts/validate_workflow_docs.py --mode manual`

## Expected Results
- PASS criteria:
  - typecheck/build 通过；截图显示双栏 split view 与更克制的组件圆角；workflow docs 校验通过
- Outputs to keep (10~20 lines snippet):
  - Vite build 摘要
  - workflow docs validator 输出
