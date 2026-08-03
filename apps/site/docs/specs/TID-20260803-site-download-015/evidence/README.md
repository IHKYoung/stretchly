# Site Download Evidence

发布前状态：

- Site branch: `baseline` at `78a1dea`，与 `origin/baseline` 一致。
- Primary route: GitHub latest release API。
- Previous pinned fallback: `v0.1.4/Pauza_0.1.4_aarch64.dmg`。
- Preserved unrelated dirty files: `copy.js`、`docs/ReleasePlaybook.md`、`docs/specs/_template/arch.md`、`docs/specs/_template/ui.md`。

最终 site commit、deployment 和 production HTTP 证据将在 GitHub `v0.1.5` Release 验证后回填。

## Pre-deploy Validation

- `node --check script.js` / `download/targets.js`: PASS。
- Static contract: `index.html` href 与 `targets.js` fallback 均为 `v0.1.5/Pauza_0.1.5_aarch64.dmg`；latest API 与 `_aarch64.dmg` matcher 保持。
- Runtime smoke: latest release override、API failure fallback、missing asset fallback 三路径 PASS。
- `git diff --check`: PASS。
- Workflow docs: PASS；agent config 未配置 multi-agent，按设计 SKIP。
- Existing unrelated dirty files remain unstaged candidates: `copy.js`、`docs/ReleasePlaybook.md`、`docs/specs/_template/arch.md`、`docs/specs/_template/ui.md`。
