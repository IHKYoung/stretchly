# Task-ID: TID-20260412-site-release-playbook

## Goals
- 提供一条可重复执行的站点 release 发布命令。
- 降低下次更新 pinned 下载链接、上传 GitHub release 资产时的人工遗漏成本。

## Non-Goals
- 不在脚本里自动生成 workflow task docs、commit message 或 push 当前分支。
- 不改站点运行时逻辑、页面样式或 GitHub release 命名规范。

## Constraints & Assumptions
- 仍然依赖本机已安装并完成认证的 `gh`。
- 当前站点只维护 Apple Silicon 的 `Pauza_<version>_aarch64.dmg` 下载目标。
- 目标仓库默认仍为 `IHKYoung/Pauza`，目标分支默认仍为 `baseline`。

## System Boundaries
- Modules: `.gitignore`、`scripts/publish_site_release.py`、`docs/ReleasePlaybook.md`、`README.md`、`docs/RepositoryGuidelines.md`、`docs/CodeMap.md`
- Ownership: 发布脚本由站点仓库维护；真正的 release 资产与 tag 仍在 GitHub 仓库托管。
- Dependency direction: 脚本先解析本地 dmg 与站点文件，再按需调用 `gh release view/create/upload`。

## API / Contract
- Signatures / Endpoints: `python3 scripts/publish_site_release.py --asset /abs/path/Pauza_<version>_aarch64.dmg`
- Request/Response schema (typed): 读取本地 asset basename 推导 `version` / `tag` / `release_url`；通过 `gh release view/create/upload` 与 GitHub 交互。
- Error model (codes, retryability): asset 不存在、文件名不匹配、`gh auth status` 失败、目标文件正则替换失败时直接退出并报错；已有 release 时改走 `gh release upload --clobber`。

## Data Model / Storage
- 无持久化存储；仅修改仓库内静态文件，并通过 GitHub release 资产作为外部发布结果。

## Invariants
- 脚本只能接受 `Pauza_<version>_aarch64.dmg` 命名的资产。
- `index.html` 与 `download/targets.js` 的 pinned URL 必须保持同一版本。
- 已有 release 再次执行时应可通过 `--clobber` 覆盖同名资产，而不是静默失败。

## Concurrency / Lifecycle / Memory Model
- 单次命令串行执行：解析参数 -> 更新站点文件 -> 调用 `gh`
- 无长生命周期进程、无共享内存或并发状态。

## Observability Plan (Debug-Driven)
- Logs: 直接打印 `[INFO]` / `[UPDATE]` / `[SKIP]` / `[DONE]` 与执行命令。
- Metrics: 不新增。
- Traces: 不新增。
- Debug flags: `--dry-run` 用于演练完整流程但不实际执行。

## Security & Privacy Considerations
- 凭证继续由 `gh auth login` 管理，不在脚本内读取或写入明文 token。
- 脚本不上传除指定 dmg 外的其它本地文件。

## Risks & Rollback
- Failure modes: 资产路径错误；命名不符合约定；正则未命中导致站点文件无法更新；`gh` 未认证；重复发布时上传了错误资产。
- Rollback steps: 回退脚本与文档 commit；如错误上传 release，可重新运行脚本覆盖资产，或在 GitHub 上删除错误 release/tag。

## Acceptance Criteria (System)
- 一条命令即可从 dmg 文件名推导版本并更新站点 pinned 下载链接。
- 当 GitHub release 已存在时，脚本能够改走 upload 覆盖路径。
- README / Repository Guidelines / Release Playbook 都记录了这条标准流程。

## Open Questions / Decision Requests
- 无。
