# Task-ID: TID-20260723-release-014-github-publish

## Goals
- 将当前源码、版本说明、git tag、macOS DMG、GitHub Release 与官网固定下载地址收敛到同一个 `v0.1.4` 快照。

## Non-Goals
- 不升到 0.1.5，不改变应用运行时状态机，不新增分发平台，不重写 git 历史。

## Constraints & Assumptions
- 目标资产名固定为 `Pauza_0.1.4_aarch64.dmg`。
- release 仓库为 `IHKYoung/Pauza`；其 `baseline` 与当前代码历史不相交，因此 release 由显式推送的 `v0.1.4` tag 定位，不改写该分支。
- 当前 dirty break-ideas 文件属于本次 0.1.4 发布边界。

## System Boundaries
- Modules: version manifests、break-ideas source/registry、Tauri build、git commit/tag、GitHub Release、site pinned URL。
- Ownership: 本地 commit/tag 是源码事实源；Tauri 是产物生成者；GitHub Release 是分发状态事实源；官网只展示固定下载契约。
- Dependency direction: source commit -> build artifact -> hash/signature checks -> tag/push -> GitHub Release -> site download verification。

## API / Contract
- GitHub tag: `v0.1.4`。
- Release asset: `Pauza_0.1.4_aarch64.dmg`。
- Download URL: `https://github.com/IHKYoung/Pauza/releases/download/v0.1.4/Pauza_0.1.4_aarch64.dmg`。
- Error model: build、signing、authentication、push、release create/upload 与 download verification 分别报告，不把单一步骤成功合成为整体成功。

## Data Model / Storage
- 本任务不迁移用户数据；只新增 git/docs 记录、构建产物和 GitHub release 元数据。

## Invariants
- 四个版本真源保持 `0.1.4`。
- registry 必须由当前 break-ideas source 可重复生成且无 diff。
- release 资产 SHA-256 必须与本地已验证 DMG 一致。
- 官网固定链接必须与实际 release tag/资产名完全一致。

## Concurrency / Lifecycle / Memory Model
- 发布严格串行：冻结 -> 验证 -> commit -> build/签名核验 -> push/tag -> release -> 下载核验。
- 不并发上传同名资产，不在 release 成功前宣称站点下载可用。

## Observability Plan (Debug-Driven)
- Logs: 测试摘要、构建结果、签名/公证输出、SHA-256、push/tag/release metadata。
- Metrics: DMG 文件大小与 SHA-256。
- Traces: Git commit hash、tag target、release URL、asset API metadata。
- Debug flags: 无；失败时保留各步骤原始退出码和错误类别。

## Security & Privacy Considerations
- 不在日志或仓库中记录 GitHub token、Apple ID 密码或其他凭据值。
- GitHub 认证只申请完成 repo release/push 所需权限；Apple 凭据只由当前 shell/keychain 消费。

## Risks & Rollback
- Failure modes: token 失效、目标分支缺失、签名证书不可用、构建失败、tag 冲突、资产上传后哈希不一致。
- Rollback steps: 外部发布前修正本地提交；外部发布后用 revert 和站点链接回退，远端 release/tag 删除或替换需再次明确授权。

## Acceptance Criteria (System)
- commit/tag/DMG/release/site URL 全部对应 0.1.4，测试与 release-critical 验证通过，任何签名降级均显式披露。

## Open Questions / Decision Requests
- 无；签名与 GitHub 认证门禁均已解除，剩余步骤只允许 fast-forward branch push 与新 tag/release 创建。
