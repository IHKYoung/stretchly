# Task-ID: TID-20260803-release-015-github-publish

## Goals
- 让单一 `0.1.5` 源码快照、签名公证 DMG、Git tag、GitHub Release 和官网下载目标可相互追溯。
- 保持 v0.1.4 可用，不通过覆盖、删除或历史改写完成发布。

## Non-Goals
- 不建立自动更新服务、跨架构构建矩阵或新的发布基础设施。
- 不修改桌面端运行时行为，不重做官网 UI，不处理嵌套站点仓库的无关 dirty 文件。

## Constraints & Assumptions
- 版本真源已统一为 `0.1.5`，公开最新版是 `v0.1.4`，远端不存在 `v0.1.5`。
- 当前构建宿主是 Apple Silicon macOS，发布资产命名为 `Pauza_0.1.5_aarch64.dmg`。
- Developer ID Application 身份及 Apple ID 公证变量存在；凭据内容不得写入日志或文档。
- GitHub release 仓库为 `IHKYoung/Pauza`；根 `origin/baseline` 可快进，`pauza/baseline` 历史分叉且不更新。

## System Boundaries
- Modules: 根源码仓库、`apps/desktop` Tauri 构建、Apple codesign/notary、GitHub tag/Release、`apps/site` 下载解析与 Vercel 部署。
- Ownership: git commit/tag 持有源码事实；Tauri/Cargo 持有 bundle 版本；Apple 签名与公证票据持有平台信任事实；GitHub Release 持有公开资产事实；官网只呈现和跳转。
- Dependency direction: committed source -> signed/notarized artifact -> tag/Release -> site latest resolver/pinned fallback -> user download。

## API / Contract
- Signatures / Endpoints: `GET https://api.github.com/repos/IHKYoung/Pauza/releases/latest`；`GET https://github.com/IHKYoung/Pauza/releases/latest/download/Pauza_0.1.5_aarch64.dmg`；固定回退为同一 tag asset URL。
- Request/Response schema: latest response 的 `tag_name` 必须为 `v0.1.5`，`assets[]` 至少包含且只发布预期 Apple Silicon DMG；资产 `name`、`size`、`browser_download_url` 与本地证据一致。
- Error model: API/网络/解析失败可重试并回退固定 URL；固定 URL 失败为用户可见下载失败；签名、公证、hash、tag 冲突均为不可降级发布失败。

## Data Model / Storage
- 发布元数据存于 git commit/tag、GitHub Release 和 task evidence；DMG 是构建产物，不进入 git。
- 记录字段包括 version、commit SHA、tag target、asset name/size/SHA-256、signature/notary verdict、release URL、site deployment/HTTP verdict。

## Invariants
- 所有版本真源、bundle version、tag、release title、asset filename 和官网 fixed fallback 必须表达 `0.1.5`。
- 远端下载得到的字节必须与本地已验证 DMG 的 SHA-256 相同。
- latest 与 fixed fallback 不得分别指向不同版本。
- 不提交凭据、临时挂载点、本地构建目录或站点无关 dirty 文件。

## Concurrency / Lifecycle / Memory Model
- 发布采用串行生命周期：门禁 -> source commit/push -> build/verify -> tag -> Release -> remote verify -> site fallback/deploy -> evidence closure。
- 创建 tag/Release 前再次查询同名远端对象；若被并发占用则停止，不覆盖。
- GitHub latest CDN/API 可能短暂缓存，使用 API metadata、direct asset 和生产站点多点回验。

## Observability Plan (Debug-Driven)
- Logs: 保存关键命令、exit status 和非敏感结果摘要。
- Metrics: DMG byte size、SHA-256、GitHub asset size、release published timestamp。
- Traces: commit -> tag target -> release tag -> asset URL -> downloaded hash。
- Debug flags: 不启用会暴露 Apple/GitHub 凭据的 verbose shell trace。

## Security & Privacy Considerations
- 只检查敏感环境变量是否存在，不输出值；命令行不展开密码到文档。
- 使用现有 GitHub/Apple 身份，不新增 token、服务或付费依赖。
- 通过 `codesign`、`spctl`、`stapler` 和远端 hash 防止发布未签名或被替换的包。

## Risks & Rollback
- Failure modes: 本地门禁失败、Apple 公证失败、Gatekeeper 拒绝、远端冲突、asset 上传不完整、latest 缓存、站点 deploy 失败。
- Rollback steps: 发布前任何失败直接停止；发布后源码/站点用后续提交修复，v0.1.4 保持可用；删除已发布远端对象不属于自动回滚，需要用户新授权。

## Acceptance Criteria (System)
- 从 root commit 到远端下载 DMG 的追溯链完整，且每个不可降级门禁都有可复现证据。
- 官网生产入口和固定回退最终都指向通过验证的 v0.1.5 asset。

## Open Questions / Decision Requests
- 无；Apple Silicon 单资产、GitHub Release 管理和官网下载已由用户目标与既有发布模式确定。
