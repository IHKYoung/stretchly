# Task-ID: TID-20260803-release-015-github-publish

## Test Strategy
- Unit: 运行根测试、break ideas 内容治理测试和 desktop Rust 测试。
- Integration: 运行 locale/break registry 生成器幂等检查、TypeScript 检查、前端生产构建和 workflow validators。
- E2E: 构建签名公证 DMG，检查 bundle 版本/架构/签名/Gatekeeper/stapler/装载；创建 Release 后重新下载并比较 SHA-256；验证生产站点 latest/fallback 下载。

## Acceptance Criteria Coverage Map (AC -> Tests)
- AC1 -> 版本真源查询、README/CHANGELOG/release notes diff 检查。
- AC2 -> `npm test`、`npm run typecheck`、`cargo test`、`npm --prefix apps/desktop run build`、生成器幂等、workflow validators。
- AC3 -> `file`、`PlistBuddy`、`codesign --verify --deep --strict`、`spctl --assess`、`stapler validate`、`hdiutil attach/detach`、`shasum -a 256`。
- AC4 -> `git show-ref`、`git cat-file`、`gh release view`、GitHub API、远端下载 hash。
- AC5 -> 站点静态检查、站点仓库 staged diff、Vercel deployment/生产 HTTP、latest/download URL。
- AC6 -> `validate_workflow_docs.py`、commit audit 与 task evidence 检查。

## Interaction Contract Coverage
- Interaction impact: indirect
- Primary flow -> tests/evidence: GitHub latest API 的 `tag_name/assets` 与生产官网下载解析/HTTP 结果。
- Fallback / secondary flow -> tests/evidence: `index.html` 与 `download/targets.js` 固定 URL 静态断言，直接请求该 URL。
- Visible states / transitions -> tests/evidence: 既有站点测试/源码审查覆盖 loading、重复点击和错误分支；成功状态由线上 asset 跳转验证。

## Governance Gates
- Agent Config Validation: `python3 scripts/validate_agent_configs.py`
- Workflow Docs Validation: `python3 scripts/validate_workflow_docs.py --mode manual`
- Approval Escalation Owner: orchestrator

## False-pass Cases
- 只看到本地 DMG 存在，没有验证签名、公证、Gatekeeper 或远端下载 hash。
- GitHub Release 已创建但 tag 指向错误 commit，或 asset 仍是 v0.1.4。
- 官网源码更新了 fixed URL，但生产 latest API/下载链路未验证。
- 只验证 latest 路径，固定回退仍指向旧版本。
- 根或嵌套站点提交混入用户已有的无关 dirty 文件。
- task 标记 DONE 但仍有占位符、未记录 hash/URL/rollback 或 workflow validator 失败。

## Evidence Capture (UI / E2E)
- Required: yes
- Owner: orchestrator
- Artifacts path: docs/specs/TID-20260803-release-015-github-publish/evidence/
- What to capture:
  - Screenshots: 浏览器工具可用时采集官网下载入口；不可用时以生产 HTML、HTTP 响应和 GitHub metadata 代替并明确限制。
  - Video/trace: 不需要视频；保留 commit/tag/release/asset/deployment 追溯链。
  - HAR/console logs: 不强制；保留 curl headers、GitHub API JSON 摘要、下载 hash。

## Quality Gates (Non-functional)
- a11y: 不修改交互结构；站点既有键盘和焦点语义不得变化。
- perf budget: 不新增前端依赖或资源；只替换版本 URL。
- error handling / observability: latest 失败必须进入 fixed fallback，fixed 失败必须显式报错；发布命令退出码与远端 metadata 留证。
- security / privacy: 不打印或提交 Apple/GitHub 凭据；只发布 Developer ID 签名、公证通过的资产。

## Boundary / Invalid Input Cases
- latest release 不存在、tag 不匹配、assets 为空或名称不匹配时不得选择任意资产。
- 同名 tag/Release 已存在时停止，不覆盖。
- DMG 文件名、bundle version、架构或 hash 任一不一致时停止。
- GitHub remote branch 无法快进时停止。

## Concurrency / Race Cases
- tag/Release 创建前后分别查询远端，防止并发占用 `v0.1.5`。
- push 前确认 `origin/baseline` 仍是本地祖先；站点 push 前确认 nested origin 未前进。
- 构建从已提交快照进行，构建后若受管源码变化则停止 tag。

## Mocks & Test Data
- 不使用 mock 发布；GitHub/Apple/Vercel 均验证真实状态。
- 本地验证使用真实 `Pauza_0.1.5_aarch64.dmg`，远端验证下载到新临时目录，避免覆盖本地产物。

## Commands to Run
- `python3 scripts/sync_desktop_locales.py --check`
- `python3 scripts/sync_desktop_break_ideas.py --check`
- `npm test`
- `npm run typecheck`
- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml`
- `npm --prefix apps/desktop run build`
- `python3 scripts/validate_agent_configs.py`
- `python3 scripts/validate_workflow_docs.py --mode manual`
- `npm --prefix apps/desktop run tauri build -- --bundles app,dmg`
- `codesign --verify --deep --strict --verbose=2 <app>`
- `spctl --assess --type execute --verbose=4 <app>`
- `xcrun stapler validate <app-or-dmg>`
- `hdiutil attach <dmg>` / `hdiutil detach <mount>`
- `gh release view v0.1.5 --repo IHKYoung/Pauza --json ...`
- `curl` latest API/direct asset/production site，随后 `shasum -a 256 <downloaded-dmg>`。

## Expected Results
- PASS criteria: AC1-AC6 全部满足；所有不可降级命令退出 0；本地与远端 DMG hash 相同；官网不再解析 v0.1.4。
- Outputs to keep: 版本真源摘要、测试统计、签名 authority、Gatekeeper accepted、notary ticket valid、DMG mount 结果、SHA-256/size、tag target、Release URL、asset URL、生产 HTTP/deployment 状态。
