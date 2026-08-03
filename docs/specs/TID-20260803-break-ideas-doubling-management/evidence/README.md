# Evidence

- `content-report.md`：本批数量、ID、类别、长度、体积、旧内容保持与人工抽样记录。
- 自动化证据：generator `50 languages / 50 bundles / 1 managed batch` 且连续两次 SHA-256 一致；Vitest `8 files / 132 tests`；typecheck PASS；Vite build PASS（`1827 modules`）；changed-file Standard、Python compile、`git diff --check` 与 workflow validator PASS；agent config validator 因无配置 SKIP。
- UI evidence blocker：Vite preview 于 `127.0.0.1:43179` 正常 ready，但当前 Browser runtime 返回空 browser 列表，无法采集三语最长完整休息截图。没有用 build 结果冒充视觉检查；现有分行单测、长度预算、typecheck 与 production build 作为非视觉证据保留。
