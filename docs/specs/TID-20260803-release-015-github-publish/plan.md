# Task-ID: TID-20260803-release-015-github-publish

## Summary
- Title: 发布 0.1.5 macOS 版本到 GitHub Release
- Date: 2026-08-03
- Level: complex
- Lane: deep
- Execution Profile: merge-gate
- Status: DONE

## Requirement Brief
- Goal restatement: 把当前已完成且版本真源统一为 `0.1.5` 的工作树构建为签名、公证的 Apple Silicon DMG，发布到 `IHKYoung/Pauza` GitHub Release，并验证官网可以下载。
- In-scope: 发布文档；当前 break ideas 扩充与已提交前置功能；根分支快进提交；macOS arm64 签名/公证构建；`v0.1.5` tag 与 Release；官网 latest 路径和固定回退；远端下载回验。
- Out-of-scope: 新功能；`0.1.6` 版本提升；Universal/x86_64 包；App Store；自动更新服务；历史重写；覆盖站点仓库中无关的未提交文件；更新分叉的 `pauza/baseline`。
- Assumptions: 当前 Apple Developer 凭据仍有效；GitHub CLI 当前账号对 `IHKYoung/Pauza` 有写权限；官网已部署 latest release 解析逻辑。
- Risks: 签名或公证失败；构建产物与源码快照不一致；tag/Release 冲突；latest API 缓存；站点部署失败；嵌套站点仓库的无关改动被误纳入。
- Interaction impact: indirect
- Primary visible flow: 官网下载按钮 -> GitHub latest release API -> `v0.1.5` Apple Silicon DMG。
- Fallback / secondary flow: latest API 不可用 -> 固定 `v0.1.5` DMG URL；解析或跳转均失败 -> 页面显示错误并允许重试。
- User-visible boundary: 官网下载入口、GitHub Release 页面、macOS 安装包及 Gatekeeper 验证。
- Key visible states / transitions: 点击下载 -> 解析中 -> GitHub 下载；解析失败 -> 固定回退；回退失败 -> 可见错误。

## Goal
- 交付可追溯、可验证、可从官网获取的 `Pauza 0.1.5` Apple Silicon 安装包。

## Scope
- In-scope: 版本说明、发布审计、源码提交、构建签名、公证、DMG 校验、GitHub Release、官网下载链接更新与线上回验。
- Out-of-scope: 与发布无关的重构、依赖变更、用户数据迁移、删除旧版 Release、清理用户已有工作树。

## Source Basis (Read Before Code)
- Related code/files reviewed: 根与 desktop 的 `package*.json`、`Cargo.toml` / `Cargo.lock`、`tauri.conf.json`；`apps/site/index.html`、`apps/site/download/targets.js`、`apps/site/scripts/publish_site_release.py`；当前 break ideas 真源、生成器和测试；git remotes、tags、branch ancestry、签名身份与凭据存在性。
- Related docs/specs/logs reviewed: `README.md`、`docs/CHANGELOG.md`、`docs/specs/TID-20260723-release-014-github-publish/**`、`docs/specs/TID-20260727-settings-simplification-015/**`、`docs/specs/TID-20260803-break-ideas-doubling-management/**`、workflow 的 task lifecycle 与 gates、既有发布审计。
- Why these are sufficient: 这些文件覆盖版本真源、构建入口、签名公证惯例、远端拓扑、官网解析契约、本版用户可见改动和仓库门禁，足以限定一次不扩张范围的 `0.1.5` 发布。

## Acceptance Criteria (AC)
- AC1: 所有版本真源保持一致为 `0.1.5`，发布说明准确覆盖本版用户可见变化，未发布区被正确收口。
- AC2: 根仓库相关测试、类型检查、Rust 测试、前端构建、生成器幂等与 workflow 校验通过。
- AC3: `Pauza_0.1.5_aarch64.dmg` 内的 app 为 arm64、版本为 `0.1.5`，Developer ID 签名、Apple 公证票据、Gatekeeper 与 DMG 装载校验通过。
- AC4: `v0.1.5` annotated tag 与 GitHub Release 指向已验证源码快照，Release 只有预期 DMG，远端下载文件 SHA-256 与本地产物一致。
- AC5: 官网 latest 下载链路解析到 `v0.1.5`，固定回退同步到 `v0.1.5`，线上下载请求可达；站点无关改动未进入本次提交。
- AC6: task docs、daily log/plan 与 commit audit 记录最终 commit、tag、asset、hash、远端和回滚信息，且 workflow 校验通过。

## Interaction Freeze
- Freeze status: frozen
- Primary flow: 官网下载按钮调用 GitHub latest release API，选择名称符合 `Pauza_*_aarch64.dmg` 的资产并跳转下载。
- Fallback / secondary flow: API、JSON 或资产选择失败时使用固定 `v0.1.5` URL；固定链接也失败时显示既有错误提示，不静默指向旧版本。
- Interaction authority / ownership boundary: GitHub Release 管理发布版本和资产；官网只负责解析、回退和呈现；macOS 系统负责安装与签名信任判断。
- Visible entrypoints / handoff cues: 官网主下载按钮、下载解析状态、GitHub 资产响应、DMG 安装界面。
- In-scope interactions: latest 解析、固定回退、下载跳转和错误展示。
- Out-of-scope interactions: 官网视觉重做、下载器、自动更新、版本选择器。
- Interaction acceptance criteria: latest API 与固定链接都指向同一 `v0.1.5` 资产；线上下载响应有效且不回落到 `v0.1.4`。

## Agent Governance
- Approval Owner: orchestrator
- Execution Profile: merge-gate
- Orchestrator Execution Profile: `sandbox_mode = "danger-full-access"` + `approval_policy = "never"`
- Required Roles: orchestrator,architect,coder,tester,scribe,reality_checker
- Execution Mode Policy: 当前环境规则禁止启动 sub-agent，采用有界 `single-agent-fallback`。
- Subagent Approval Policy: non-orchestrator agents must use `approval_policy = "never"`；本任务不启动 sub-agent。
- Escalation Route: orchestrator -> direct local execution；只有出现破坏性操作、远端冲突或授权范围变化时回到用户。
- Safe-local Command Route: orchestrator 直接执行显式路径的本地命令。
- Approval Packet Fields: command / purpose / risk / rollback。

## Execution Safety Block
- service_impact: 更新 GitHub 源码分支、tag、Release 资产及官网生产下载入口。
- touches_running_service: yes
- backup_required: no；现有 `v0.1.4` Release 保留，可作为稳定回退，不覆盖同名远端对象。
- backup_plan: 记录发布前 branch/tag/latest release/站点状态和本地 DMG hash；保留 v0.1.4。
- rollback_plan: 源码或站点问题使用后续 revert/修复提交；官网固定回退可恢复 v0.1.4；已公开 tag/Release 若需删除必须重新取得用户明确批准，默认以补丁版本修复。
- destructive_operations: none；不删除、不 force push、不 rebase、不覆盖资产。
- operator_approval_required: yes
- rationale: 用户已明确要求完成打包、官网下载和 GitHub Release 发布。

## Task Breakdown & Definition of Done (DoD)
- [x] Task-1: 收口发布元数据与任务文档
  - DoD: README、CHANGELOG、release notes、interaction/rollback/test contract 与版本真源一致。
- [x] Task-2: 提交并推送可构建源码快照
  - DoD: 门禁通过，提交审计闭环，`origin/baseline` 仅快进更新。
- [x] Task-3: 构建和验证 macOS Apple Silicon 安装包
  - DoD: app/DMG 的架构、版本、签名、公证、Gatekeeper、装载和 SHA-256 有证据。
- [x] Task-4: 创建并回验 GitHub Release
  - DoD: `v0.1.5` tag 与 Release 存在，远端下载 hash 等于本地 hash。
- [x] Task-5: 更新并验证官网下载
  - DoD: latest 与固定回退都解析到 v0.1.5，生产下载链路可达，无关站点改动未提交。

## Evidence Plan (UI / E2E)
- Evidence required: yes
- Owner: orchestrator
- Artifact path: docs/specs/TID-20260803-release-015-github-publish/evidence/
- Interaction validation note: 以 GitHub API/CLI、HTTP 响应、下载 hash、站点部署状态和固定目标源码作为发布证据。
- Required states to capture:
  - loading: 通过站点实现和静态测试确认解析期间保留既有 loading 行为。
  - empty: latest release 无匹配 DMG 时进入固定回退。
  - error: latest 与回退均失败时保留既有可见错误。
  - disabled: 下载解析期间避免重复触发的现有行为不回归。
  - success: 生产 latest 路径与固定 URL 均到达 v0.1.5，远端 DMG hash 匹配。

## Observability / Debug Plan
- Logs: Tauri build/notary 输出、git/gh 输出、HTTP headers、workflow validator、发布 evidence。
- Error codes: 命令退出码、GitHub API HTTP 状态、Apple notary submission status、Gatekeeper verdict。
- Trace/metrics: GitHub Release metadata、Vercel deployment metadata、DMG SHA-256/size。
- Debug flags: 不开启额外运行时 debug；构建失败时保留原始命令和末段日志。

## Risks & Rollback
- Risks: Apple 服务或 GitHub 暂时不可用；站点缓存；远端对象被并发创建；本地 dirty worktree 混入无关文件。
- Rollback plan: 发生冲突即停；按显式 path staging；保留 v0.1.4；发布后问题通过补丁版本或经批准的回退处理。

## Sequential Phases
- phase_execution: sequential
- phase_confirmation_policy: no-intermediate-confirmation
- phase_stop_conditions:
  - 测试、签名、公证、Gatekeeper、DMG 装载或远端下载 hash 任一失败。
  - `v0.1.5` tag/Release 在执行期间被其他提交占用。
  - 远端分支不再满足快进条件，或 GitHub/Apple 授权失效。

## Execution Plan
- Steps:
  1. 完成发布任务文档、README、CHANGELOG、release notes 与官网固定回退编辑。
  2. 执行内容生成、测试、类型检查、Rust 测试、前端构建和 workflow 门禁。
  3. 按 task scope 提交并完成 handover/commit audit，再快进推送根分支。
  4. 构建并验证签名公证的 arm64 app/DMG，记录 hash 与大小。
  5. 创建 annotated tag、GitHub Release，下载远端资产并做 hash/装载回验。
  6. 在嵌套站点仓库只提交下载兜底及其审计，推送并验证生产官网。
  7. 回填发布 evidence、关闭 task 与审计，再推送最终文档提交。

## Definition of Done (DoD)
- `plan.md` / `testplan.md` / `README.md` 不保留 `TBD/INIT` 占位进入 DONE。
- 授权 owner、升级路径、风险与回滚已写明。
- 相关验证命令与结果可在 logs/plans 中追溯。

## Approval
- Approval needed: yes
- Approved: yes；用户于 2026-08-03 明确要求发布安装包、官网可下载并由 GitHub Release 管理。
