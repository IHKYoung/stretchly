# Task-ID: TID-20260403-settings-minimal-ui

## Test Strategy
- Unit:
  - 无新增独立单元测试；本轮是前端 IA/样式重构，重点依赖静态类型与可视化证据。
- Integration:
  - `npm --prefix apps/desktop run typecheck`
  - `npm --prefix apps/desktop run build`
- E2E (if applicable):
  - 浏览器 preview + Playwright 截图 / snapshot / console 作为交互旁证。

## Acceptance Criteria Coverage Map (AC → Tests)
- AC1 -> Playwright `settings-minimal-1440x810.png` 验证 `1/4 : 3/4` 分栏与单主面板。
- AC2 -> Playwright screenshot + snapshot 验证仅有 `节奏 / 提醒与打断 / 智能暂停 / 通用` 四个分类。
- AC3 -> 人工代码审查 `App.tsx` + screenshot，确认 overview/quick actions/shortcut editor 已移出主结构。
- AC4 -> `npm --prefix apps/desktop run typecheck` 与 `build`，并检查 `saveSettings()` 仍调用 `update_settings` 写入完整 `form`。
- AC5 -> `settings-minimal-snapshot-1440x810.md`、`browser-console.log` 与构建结果共同证明页面可渲染且无明显前端异常。

## Interaction Contract Coverage
- Interaction impact: direct  <!-- none | indirect | direct -->
- Primary flow -> tests/evidence: 左侧分类导航与右侧单面板由 `settings-minimal-1440x810.png` 证明。
- Fallback / secondary flow -> tests/evidence: 浏览器 preview 正常渲染，由 `typecheck/build` + Playwright snapshot 证明。
- Visible states / transitions -> tests/evidence: `synced` 状态、保存按钮 disabled 态、分类切换骨架由截图和 DOM snapshot 证明。
- Validator expectation: 已提供 direct interaction 的 primary flow / visible state 证据，不保留占位字段。

## Governance Gates
- Agent Config Validation: `python3 scripts/validate_agent_configs.py`
- Workflow Docs Validation: `python3 scripts/validate_workflow_docs.py --mode manual`
- Approval Escalation Owner: orchestrator

## False-pass Cases
- `DONE` 任务对应的 spec 仍保留 `TBD/INIT` 占位。
- `orchestrator` 未显式使用 `sandbox_mode = "danger-full-access"` 与 `approval_policy = "never"`，却仍宣称当前仓库运行在 aggressive 基线。
- 代码变更前未记录 `Source Basis`，导致实现依据不可追溯。
- 子 agent 未显式 `approval_policy = "never"`。
- `moderate/complex` 任务通过缩小 `Required Roles` 伪装为 trivial fallback。
- 已使用 `single-agent-fallback`，但 logs/plans 没有单独记录 `Execution Mode` / `Fallback Scope` / `Fallback Reason Code`。
- 需要受角色边界约束的文件系统写命令没有经过 `run_role_guard.py`，只在结案时补跑范围校验。
- `git add -- <explicit paths...>` 仍被包进 wrapper / helper script，导致运行时看不到裸命令前缀。
- `interaction_impact != none`，但 plan/testplan/ui spec 没有定义 primary flow / fallback flow / visible states / evidence coverage。

## Evidence Capture (UI / E2E)
- Required: yes   <!-- yes | no | partial -->
- Owner: evidence_collector
- Artifacts path: docs/specs/TID-20260403-settings-minimal-ui/evidence/
- What to capture:
  - Screenshots: `settings-minimal-1440x810.png`
  - Video/trace (optional): 无
  - HAR/console logs (optional): `browser-console.log`

## Quality Gates (Non-functional)
- a11y: 导航按钮、switch、segmented control、select 与 textarea 均保留原组件语义；break window 语义不变。
- perf budget: 不新增依赖、不新增复杂动画；生产构建成功。
- error handling / observability: 继续复用顶部错误条，不吞掉 command 失败信息。
- security / privacy: 不新增网络、权限、存储或敏感数据路径。

## Boundary / Invalid Input Cases
- 隐藏字段不在 UI 中编辑，但仍需随着完整 `settings` 保存回 Rust host。
- locale 新增键必须齐全，否则导航会回落为原始 key 字符串。
- `launch on login` 仍走单独命令路径，不应误并入 dirty/save 生命周期。

## Concurrency / Race Cases (if applicable)
- `snapshot` 轮询与本地 dirty form 并存时，`dirty=true` 不应覆盖用户未保存的本地编辑。

## Mocks & Test Data
- 使用浏览器 preview 自带 `previewSnapshot()` 作为稳定的 UI 证据数据源。

## Commands to Run
- `npm --prefix apps/desktop run typecheck`
- `npm --prefix apps/desktop run build`
- Playwright 打开 `http://127.0.0.1:43180/`，导出 screenshot / snapshot / console

## Expected Results
- PASS criteria:
  - TypeScript 类型检查通过。
  - 生产构建通过。
  - 截图可见极简 split view、四个分类和单主面板。
  - 控制台无新的前端错误。
- Outputs to keep (10~20 lines snippet):
  - `vite v7.3.1 building client environment for production...`
  - `✓ 1821 modules transformed.`
  - `✓ built in 1.42s`
