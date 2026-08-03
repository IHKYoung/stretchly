# Task-ID: TID-20260803-current-worktree-handover

## Test Strategy
- Unit: Rust snapshot/tray waiting tests；Vitest 设置导航、文字层级、三语节奏与 smart skip 文案测试。
- Integration: locale/break-ideas registry 生成、TypeScript typecheck、Vite production build、Rust 全量 tests、site JavaScript 语法与下载 source contract。
- E2E (if applicable): 复用 site 任务已有生产 evidence；本轮优先尝试本地 settings preview，若浏览器能力不可用则明确保留截图缺口。

## Acceptance Criteria Coverage Map (AC → Tests)
- AC1 -> Rust `snapshot_exposes_backend_smart_wait_countdown`、`waiting_countdown_is_visible_as_transient_feedback` 与相关 tray tests。
- AC2 -> `desktopSettingsNavigation.js`、`desktopSettingsTypography.js`、`translations.js`、locale generator、typecheck/build。
- AC3 -> `node --check apps/site/script.js`、下载 source contract 检查与 `apps/site` task evidence review。
- AC4 -> `.gitignore`/`git check-ignore`、全量 tests/build、workflow validators、`git diff --check`。
- AC5 -> staged path/task-id resolver、commit-msg hook、post-commit audit review。

## Interaction Contract Coverage
- Interaction impact: direct  <!-- none | indirect | direct -->
- Primary flow -> tests/evidence: Rust tests覆盖 due/waiting -> tray；Vitest/source tests覆盖 preset/custom/cadence；site task evidence覆盖 latest href。
- Fallback / secondary flow -> tests/evidence: Rust snapshot blocker gate review、custom empty draft helper test、site source contract覆盖 pinned fallback。
- Visible states / transitions -> tests/evidence: waiting countdown toggle test、custom focus/placeholder source contract、generated locale values与 production build；视觉截图结果记录在 evidence README。

## Governance Gates
- Agent Config Validation: `python3 scripts/validate_agent_configs.py`
- Workflow Docs Validation: `python3 scripts/validate_workflow_docs.py --mode manual`
- Approval Escalation Owner: orchestrator

## False-pass Cases
- `DONE` task package 仍含 `TBD/INIT`。
- snapshot 字段存在但由前端自行递减/合成，或 blocker 时继续显示旧 waiting。
- tray test 只覆盖常规 countdown，未覆盖 `show_time_to_break_in_tray=false`。
- 只改三份 locale source，未重建 registry 或未验证主语言 key。
- typography 测试只查 class name，不查关键 font size/weight。
- site 只检查 API URL，未检查 latest download base 与 pinned fallback。
- `scratch/locale-translations/**` 被误暂存。
- staged changes 命中两个 Task-ID，但 message 未声明精确 `TASK-ID-MULTIPLE` 集合。
- 主提交后遗漏 audit-only closure，留下 `docs/commits/2026-08-03.md` 脏改动。

## Evidence Capture (UI / E2E)
- Required: partial   <!-- yes | no | partial -->
- Owner: orchestrator
- Artifacts path: docs/specs/TID-20260803-current-worktree-handover/evidence/
- What to capture:
  - Screenshots: 尝试本地 settings preview；不可用则记录原因，不伪造截图。
  - Video/trace (optional): 不要求。
  - HAR/console logs (optional): site 复用 2026-07-24 task evidence；本轮保留本地语法/source contract 输出。

## Quality Gates (Non-functional)
- a11y: custom input 本地化 aria label、原生键盘提交/恢复路径保持；source/typecheck/build 验证。
- perf budget: 无依赖/网络轮询新增；snapshot 多一个可选整数，site 仍单次共享 promise。
- error handling / observability: blocker 返回 None；site failure warning + pinned；命令失败不得降级为成功。
- security / privacy: 无新权限/凭据/数据采集；公开资产名 URL 编码。

## Boundary / Invalid Input Cases
- due 前、smart waiting、blocker、forced/current break、regular tray countdown off；custom 空/0/超范围；locale key 缺失；site reject/empty/non-DMG asset。

## Concurrency / Race Cases (if applicable)
- snapshot 在 runtime lock 内一致读取；tray detail bucket 与 title tick 不新增 owner；custom blur/Enter 双触发保持幂等；site 初始化/点击共享 promise。

## Mocks & Test Data
- Rust 默认 snapshot 与固定 `89_001ms` waiting；三份真实主语言资源；preview snapshot；site pinned v0.1.4 与 source-level latest asset contract。

## Commands to Run
- `python3 scripts/sync_desktop_locales.py`
- `python3 scripts/sync_desktop_break_ideas.py`
- `npm test`
- `npm run typecheck`
- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`
- `npm --prefix apps/desktop run build`
- `node --check apps/site/script.js`
- `python3 scripts/validate_agent_configs.py`
- `python3 scripts/validate_workflow_kit_sync.py --repo-root .`
- `python3 scripts/validate_workflow_docs.py --mode manual`
- `git diff --check`、`git diff --cached --check`、staged Task-ID validator。

## Expected Results
- PASS criteria: generators 幂等；前端/Rust tests 全绿；typecheck/build/site syntax PASS；workflow 和 diff checks PASS；临时翻译目录被 ignore；主/audit 提交闭环。
- Outputs to keep (10~20 lines snippet): tests 数量、Rust 结果、Vite summary/known warning、50 locale bundles、workflow validator、commit hashes与残留列表。
