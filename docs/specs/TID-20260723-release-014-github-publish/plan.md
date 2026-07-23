# Task-ID: TID-20260723-release-014-github-publish

## Summary
- Title: 发布 0.1.4 macOS 版本到 GitHub Release
- Date: 2026-07-23
- Level: complex
- Lane: deep
- Execution Profile: merge-gate
- Status: REVIEW

## Requirement Brief
- Goal restatement: 将当前工作树冻结为 `v0.1.4`，更新本地版本/下载入口，生成并验证 macOS arm64 DMG，完成 commit、push、tag 与 GitHub Release。
- In-scope: 当前 break-ideas source/registry、README/CHANGELOG、site pinned URL、发布 task package、测试/构建、git/GitHub 发布动作。
- Out-of-scope: 0.1.5 预升、产品功能改动、依赖安装、历史重写、旧 release 删除。
- Assumptions: 当前 dirty 文件属于 0.1.4；`IHKYoung/Pauza` 通过显式 `v0.1.4` tag 定位 release source，不依赖其分叉的 `baseline`；用户本轮指令已授权 push/release。
- Risks: GitHub token 失效、签名 identity 缺失、双远端状态漂移、旧同名 DMG 被误用、tag/asset 冲突。
- Interaction impact: indirect
- Primary visible flow: 官网下载按钮命中 0.1.4 DMG。
- Fallback / secondary flow: latest API 失败或旧响应时保留 0.1.4 pinned URL。
- User-visible boundary: 下载目标和 GitHub Release 页面。
- Key visible states / transitions: 0.1.3 pinned -> 0.1.4 pinned；release absent -> published；失败保持显性且不宣称完成。

## Goal
- 建立源码、版本、产物、release 与官网入口一致且可验证的 0.1.4 发布链。

## Scope
- In-scope: `apps/desktop/src/locales/break-ideas/**` 当前改动、`apps/site/{index.html,download/targets.js}`、`README.md`、`docs/CHANGELOG.md`、本任务 docs、git/GitHub 状态。
- Out-of-scope: 其他产品代码、架构重构、跨平台安装包、依赖与凭据持久化。

## Source Basis (Read Before Code)
- Related code/files reviewed: 四个 version manifest、break-ideas source/registry、`scripts/sync_desktop_break_ideas.py`、`apps/site/scripts/publish_site_release.py`、site 下载入口、git remotes/status/history。
- Related docs/specs/logs reviewed: `docs/RepositoryGuidelines.md`、0.1.3 publish task、0.1.4 package/signed rebuild tasks、workflow task lifecycle/gates/commit audit、handover-commit skill。
- Why these are sufficient: 已覆盖版本事实源、当前差异、生成契约、构建入口、签名门禁、双远端拓扑、发布脚本与提交审计，可定义并验证本次最小发布边界。

## Acceptance Criteria (AC)
- AC1: 版本真源均为 0.1.4，当前未发布内容进入 0.1.4 changelog，site pinned URL 指向 0.1.4。
- AC2: break-ideas generator 幂等，测试、typecheck、Rust 测试、前端 build 与 workflow validators 通过。
- AC3: 从当前快照新建 `Pauza_0.1.4_aarch64.dmg`，记录大小、SHA-256 与真实签名/公证状态。
- AC4: 本地 commit、远端 `baseline`、`v0.1.4` tag、GitHub Release 与资产可追溯到同一快照；远端下载资产哈希与本地一致。

## Interaction Freeze (only when `interaction_impact != none`)
- Freeze status: locked
- Primary flow: 首页下载按钮导航到 0.1.4 Apple Silicon DMG。
- Fallback / secondary flow: latest API 失败、无匹配资产或返回旧版本时保留 0.1.4 pinned URL。
- Interaction authority / ownership boundary: GitHub Release 拥有资产可用性事实；site 只提供固定 URL 和 latest API 选择逻辑。
- Visible entrypoints / handoff cues: 首页 `.download-button`。
- In-scope interactions: href 与下载目标版本。
- Out-of-scope interactions: 视觉、文案、动画、下载后的安装流程。
- Interaction acceptance criteria: 初始/最终 href 均不得回退到 0.1.3；发布后 URL 返回目标资产。
- Validator expectation: 本任务已定义 primary/fallback/visible states 与 evidence coverage。

## Agent Governance
- Approval Owner: orchestrator
- Execution Profile: merge-gate
- Orchestrator Execution Profile: 当前环境由产品权限策略管理，不宣称使用仓库模板中的 aggressive baseline。
- Required Roles: orchestrator,architect,coder,tester,scribe,reality_checker
- Execution Mode Policy: 上层策略要求用户明确请求才可启用 multi-agent。
- Subagent Approval Policy: 本任务不启动子 agent；如后续获得用户明确授权，任何非 orchestrator agent 必须使用 `approval_policy = "never"`。
- Escalation Route: 本地安全执行；签名/认证/外部发布异常交由用户决定。
- Safe-local Command Route: 由 orchestrator 直接执行明确路径命令。
- Approval Packet Fields: command / purpose / risk / rollback。

## Execution Safety Block
- service_impact: GitHub 分支/tag/release 与官网固定下载目标。
- touches_running_service: yes。
- backup_required: no；保留 v0.1.3 release 作为回退下载源。
- backup_plan: 发布前测试/build/hash/signature 校验并记录 commit/tag target。
- rollback_plan: 代码用后续 revert；site 恢复 v0.1.3；删除/替换远端 release/tag 必须重新获用户授权。
- destructive_operations: none。
- operator_approval_required: yes，已由用户明确授权当前 0.1.4 push/release；不包含删除或历史改写。
- rationale: 发布与 push 有外部副作用，但请求具体且边界清晰。

## Task Breakdown & Definition of Done (DoD)
- [x] Task-1: 冻结 0.1.4 本地内容与发布文档
  - DoD: 版本、changelog、README、site URL 和 release notes 对齐。
- [x] Task-2: 完成本地验证、构建与产物核验
  - DoD: 全部门禁通过，DMG 为当前构建并记录 hash/signature 状态。
- [ ] Task-3: 完成提交与 GitHub 发布闭环
  - DoD: commit/push/tag/release/asset/download 校验通过，审计记录闭合。

## Evidence Plan (UI / E2E)
- Evidence required: yes
- Owner: orchestrator
- Artifact path: docs/specs/TID-20260723-release-014-github-publish/evidence/
- Interaction validation note: 记录 site 两处 pinned URL、release API metadata、下载资产哈希与失败门禁。
- Required states to capture:
  - loading: pinned href 在 API 完成前即为 0.1.4。
  - empty: 无匹配资产时保留 0.1.4。
  - error: API/认证失败显性记录。
  - disabled: 不适用。
  - success: release URL 可见且下载资产哈希匹配。

## Observability / Debug Plan
- Logs: generator/tests/build/signature/hash/git/GitHub 输出摘要。
- Error codes: 保留命令退出码及 GitHub/Tauri/codesign 原始错误类别。
- Trace/metrics (optional): commit SHA、tag target、release URL、asset size、SHA-256。
- Debug flags (optional): 发布脚本先 dry-run，真正 upload 时避免重复修改 site。

## Risks & Rollback
- Risks: 无签名包误称签名、旧 bundle 误上传、token 失效、双远端 target 漂移、release 创建与代码 push 部分成功。
- Rollback plan: 每一步先核验前置状态；部分成功时停下并报告，不自动删除远端对象。

## Sequential Phases
- phase_execution: sequential
- phase_confirmation_policy: no-intermediate-confirmation（用户已授权同一具体发布操作；新增破坏性动作仍需确认）
- phase_stop_conditions:
  - 当前源码验证失败。
  - 无法恢复 GitHub 发布权限。
  - 签名 identity 缺失且用户不接受无签名发布。
  - 远端已有冲突的 v0.1.4 tag/release/asset。

## Execution Plan
- Steps:
  1. 对齐 changelog、README、site pinned URL、release notes 与 task package。
  2. 重建 registry，运行测试/typecheck/Rust/build/workflow gates。
  3. 从当前快照重新构建 macOS arm64 DMG并核验签名、staple、大小与 hash。
  4. 形成主提交和 audit-only 提交，将 `origin/baseline` fast-forward，并把同一新 tag 推到 `origin` 与 `pauza` 后创建 release；不写入分叉的 `pauza/baseline`。
  5. 通过 GitHub metadata 与重新下载 hash 核验发布结果。

## Definition of Done (DoD)
- spec 不保留未解析占位。
- 发布结果包含可复现命令、commit/tag/release/asset 证据与真实签名状态。
- 任何未完成门禁都阻止“发布完成”结论。

## Approval
- Approval needed: yes
- Approved: 用户于 2026-07-23 明确要求“这个版本打包发布到 github release”，并补充“同时更新一下本地的版本”。
