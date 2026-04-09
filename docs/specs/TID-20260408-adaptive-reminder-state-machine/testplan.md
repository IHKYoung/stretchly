# Task-ID: TID-20260408-adaptive-reminder-state-machine

## Test Strategy
- Unit: 在 `apps/desktop/src-tauri/src/state.rs` 追加 Rust 单测，直接覆盖 `tick()` 和 `RuntimeState` 的 pending-delivery 状态迁移。
- Integration: 运行 `cargo check --manifest-path apps/desktop/src-tauri/Cargo.toml` 验证 host 编译；运行 workflow docs validator 校验门禁字段。
- E2E (if applicable): 暂不做 Playwright/桌面 UI 自动化；本轮主风险在 host 调度逻辑，优先用 Rust 单测证明。

## Acceptance Criteria Coverage Map (AC → Tests)
- AC1 -> `due break stays pending while idle_ms is below opportunity threshold`
- AC2 -> `pending break starts once idle opportunity appears`; `soft nudge is emitted only once while break stays protected`
- AC3 -> `status text reflects waiting-for-opportunity state`; `last_action changes when waiting starts / nudge sent`
- AC4 -> `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`; `cargo check --manifest-path apps/desktop/src-tauri/Cargo.toml`; `python3 scripts/validate_workflow_docs.py --mode manual`

## Interaction Contract Coverage
- Interaction impact: direct  <!-- none | indirect | direct -->
- Primary flow -> tests/evidence: Rust 单测覆盖“到点但用户仍活跃时不立刻弹窗，进入 waiting-for-opportunity”；Evidence README 记录该状态机与测试输出。
- Fallback / secondary flow -> tests/evidence: Rust 单测覆盖“检测到 idle opportunity 后开始 break”；现有 blocker 逻辑保持不变并依赖既有实现约束。
- Visible states / transitions -> tests/evidence: 状态字符串断言覆盖普通倒计时/等待空档/soft nudge 后状态；Evidence README 说明这些状态如何映射到 tray/detail。

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
- Owner: orchestrator（single-agent-fallback）
- Artifacts path: docs/specs/TID-20260408-adaptive-reminder-state-machine/evidence/
- What to capture:
  - Screenshots: N/A（本轮不改设置页结构或 break window 视觉）
  - Video/trace (optional): N/A
  - HAR/console logs (optional): workflow / cargo 验证摘要 + Rust 单测结果片段

## Quality Gates (Non-functional)
- a11y: 无新增控件，不引入新的键盘/焦点路径。
- perf budget: `tick()` 仍保持 O(1) 判断，不新增高频系统调用或额外线程。
- error handling / observability: 状态机切换必须可通过 `status/statusDetail/last_action` 观察到。
- security / privacy: 不新增全局键盘监听、埋点或外部上传；只使用既有 idle / DND / app exclusion 信号。

## Boundary / Invalid Input Cases
- adaptive 阈值应有 sanitize/clamp，避免 `0` 或极大值导致永远不提醒或几乎立即提醒。
- break 到期后若用户立刻进入 focus / DND / app exclusion / natural break，pending 状态应被安全清空并走既有阻塞逻辑。
- hidden settings 缺失时应回落到默认值，不影响现有用户配置文件读取。

## Concurrency / Race Cases (if applicable)
- `tick()` 每秒执行一次，所有运行时读写仍必须通过同一 `Mutex<RuntimeState>`，避免 pending 状态与 break open/close 交叉写。
- waiting-for-opportunity 与 manual finish/current break 状态不可同时存在；开始 break 时必须清空 pending wait/nudge 标记。

## Mocks & Test Data
- 使用 `PauzaSettings::default()` 派生测试配置；用显式时间戳驱动 `tick(now, idle_ms, dnd, app_exclusion)`，避免真实系统时间依赖。

## Commands to Run
- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`
- `cargo check --manifest-path apps/desktop/src-tauri/Cargo.toml`
- `python3 scripts/validate_workflow_docs.py --mode manual`

## Expected Results
- PASS criteria:
  - due-hold / idle opportunity / soft nudge once 的单测全部通过。
  - host 编译通过，workflow docs 无门禁缺失。
- Outputs to keep (10~20 lines snippet):
  - `cargo test` 的新增测试名与 PASS 统计
  - `cargo check` 的完成摘要
  - `validate_workflow_docs.py` 的通过摘要
