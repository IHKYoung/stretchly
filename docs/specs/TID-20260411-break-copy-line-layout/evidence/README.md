# Task-ID: TID-20260411-break-copy-line-layout

## Evidence Summary
- 类型：partial
- 原因：本轮是 break prompt 的前台排版优化；已通过 Vitest 和 browser preview 截图验证核心阅读体验，无需额外宿主层录屏。

## Captured Evidence
- `npm test`：PASS（`Test Files 5 passed (5)`、`Tests 62 passed (62)`）
- `npm --prefix apps/desktop run build`：PASS（`✓ 1826 modules transformed.`、`✓ built in 1.77s`，保留既有 chunk size warning）
- `python3 scripts/validate_workflow_docs.py --mode manual`：PASS（`[OK] Workflow docs validation passed for 2026-04-11 ✅`）
- `break-copy-preview.png`：
  - 路径：`docs/specs/TID-20260411-break-copy-line-layout/evidence/break-copy-preview.png`
  - 观察点：主文案已按整句/分句独立成行；第二句在逗号处分成两行，不再从任意字位硬断

## AC Mapping
- AC1：截图 `break-copy-preview.png` 与 `desktopBreakCopyLayout.js` 中文用例共同证明长中文句不再任意折字
- AC2：`desktopBreakCopyLayout.js` 英文多句 / 长句用例证明“一句一行”和长句分句换行都成立
- AC3：helper `apps/desktop/src/lib/break-copy-layout.ts` 与对应 Vitest 已落地
- AC4：`npm test`、`npm --prefix apps/desktop run build` 与 docs validator 均已通过
