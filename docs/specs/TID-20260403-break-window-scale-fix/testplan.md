# Task-ID: TID-20260403-break-window-scale-fix

## Test Strategy
- Unit:
  - 无新增独立单测；本轮以 Rust/TS 编译链与 preview DOM 证据为主。
- Integration:
  - `cargo check --manifest-path apps/desktop/src-tauri/Cargo.toml`
  - `npm --prefix apps/desktop run typecheck`
  - `npm --prefix apps/desktop run build`
- E2E (if applicable):
  - Playwright 打开 `?window=break`，导出 preview DOM snapshot。

## Acceptance Criteria Coverage Map (AC → Tests)
- AC1 -> 代码审查 `App.tsx`，确认 break 页不再使用内层 `Card` 窄容器。
- AC2 -> Playwright `break-window-preview-snapshot.md` 验证 preview break route 可直接显示模拟中的微休息。
- AC3 -> DOM snapshot 验证 break 页为顶部信息 + 主体内容 + 底部 CTA 三段，而非内层卡片。
- AC4 -> 代码审查 `shell.rs`，确认三档 window-mode 尺寸整体拉大。
- AC5 -> `cargo check`、`typecheck`、`build` 通过。

## Interaction Contract Coverage
- Interaction impact: direct  <!-- none | indirect | direct -->
- Primary flow -> tests/evidence: `break-window-preview-snapshot.md` 证明 break 页直接呈现倒计时、标题和 CTA。
- Fallback / secondary flow -> tests/evidence: preview route 使用 mock current break 作为 UI 验证兜底。
- Visible states / transitions -> tests/evidence: snapshot 中可见 badge、clock、倒计时、说明文案和三个操作按钮。
- Validator expectation: 证据字段已经补齐，不保留占位内容。

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
- Required: partial   <!-- yes | no | partial -->
- Owner: evidence_collector
- Artifacts path: docs/specs/TID-20260403-break-window-scale-fix/evidence/
- What to capture:
  - Screenshots: screenshot 工具仍卡在 fonts wait，本轮未保留图片
  - Video/trace (optional): 无
  - HAR/console logs (optional): DOM snapshot 为主

## Quality Gates (Non-functional)
- a11y: break 页按钮语义和顺序保持不变。
- perf budget: 不新增依赖，仅调整布局与尺寸。
- error handling / observability: 编译链与 preview 控制台无新增错误。
- security / privacy: 不新增权限、网络或数据路径。

## Boundary / Invalid Input Cases
- `currentBreak=null` 时 break 页仍能渲染 cleared fallback 文案。
- preview mock 只在 `?window=break` 且无 Tauri runtime 时启用。

## Concurrency / Race Cases (if applicable)
- 无新增并发状态；仍复用现有 break action 命令。

## Mocks & Test Data
- `previewSnapshot()` 在 break route 下构造一个模拟中的微休息。

## Commands to Run
- `cargo check --manifest-path apps/desktop/src-tauri/Cargo.toml`
- `npm --prefix apps/desktop run typecheck`
- `npm --prefix apps/desktop run build`
- Playwright 打开 `http://127.0.0.1:43179/?window=break` 并导出 DOM snapshot

## Expected Results
- PASS criteria:
  - Rust host 和前端构建通过。
  - preview break route 可见真实 break UI 结构。
  - break 页不再表现为窄卡片套在大窗口里。
- Outputs to keep (10~20 lines snippet):
  - `Finished dev profile [unoptimized + debuginfo] target(s) in 1.17s`
  - `✓ 1821 modules transformed.`
  - `✓ built in 1.28s`
