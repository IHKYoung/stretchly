# Task-ID: TID-20260407-current-worktree-commit

> 若本任务不涉及架构/契约/数据/并发边界，可在此注明 `N/A` 并保持该文件存在（流程门禁要求）。  

## Goals
- N/A（本任务不涉及新的架构、契约、数据或并发边界变更）

## Non-Goals
- 不修改 Electron / Tauri 业务架构。
- 不新增 API、schema、状态机或生命周期逻辑。

## Constraints & Assumptions
- 仅允许整理提交边界、忽略规则、docs/spec/evidence 与 Git 提交流程。
- 当前 staged diff 代表待提交主线，原始调试缓存可安全排除。

## System Boundaries
- Modules:
  - `.gitignore`
  - `docs/**`
  - `.githooks/**`
  - `scripts/record_commit_audit.py`
  - `scripts/validate_commit_message_task_id.py`
- Ownership:
  - 本任务只触及提交流程和文档沉淀层。
- Dependency direction:
  - Git hooks -> workflow scripts -> docs/commits 与 spec/logs/plans

## API / Contract
- Signatures / Endpoints:
  - N/A
- Request/Response schema (typed):
  - N/A
- Error model (codes, retryability):
  - commit hook 校验失败时直接阻断提交；无需新增错误码。

## Data Model / Storage
- 不新增数据模型；仅依赖 Git index/worktree 和现有 docs 目录结构。

## Invariants
- 正式提交不包含原始调试缓存。
- multi-task 提交必须使用 `TASK-ID-MULTIPLE`。
- `docs/commits/` 由主提交后的 audit 流程追加。

## Concurrency / Lifecycle / Memory Model
- 不适用；仅本地单进程 Git / 文档操作。

## Observability Plan (Debug-Driven)
- Logs:
  - `git diff --cached --stat`
  - `git status --short`
  - `.githooks/commit-msg`
  - `.githooks/post-commit`
- Metrics:
  - N/A
- Traces:
  - N/A
- Debug flags:
  - N/A

## Security & Privacy Considerations
- 不涉及密钥、网络请求、外部服务或敏感数据。

## Risks & Rollback
- Failure modes:
  - commit message Task-ID 归因错误被 hook 拦截。
  - 主提交后忘记补 audit-only commit，导致工作区残留 `docs/commits/` 脏改动。
- Rollback steps:
  - 重新整理 stage、修正文档后再提交。
  - 若 audit-only commit 内容有误，仅回退 `docs/commits/` 并重做。

## Acceptance Criteria (System)
- `.gitignore` 能屏蔽后续原始 Playwright / `__pycache__` 缓存。
- 提交流程可通过 hook 与 workflow validator 完整闭环。
- audit-only follow-up 能把 `docs/commits/` 正式落盘且不递归追加。

## Open Questions / Decision Requests
- 无。
