# Task-ID: TID-20260403-hig-tailwind-redesign

## Test Strategy
- Unit:
  - 无新增纯函数单测；本轮主要是 UI/样式系统重构
- Integration:
  - `npm --prefix apps/desktop run build`
- E2E (if applicable):
  - Playwright 浏览器预览设置页
  - Playwright mocked runtime break prompt

## Acceptance Criteria Coverage Map (AC → Tests)
- AC1 -> `npm --prefix apps/desktop run build` 通过；`settings-snapshot.md` 证明 Tailwind/shadcn 结构已替换旧布局
- AC2 -> `settings-redesign.png` + `settings-snapshot.md` 证明 overview hero、grouped settings、advanced accordion、save rail 全部落地
- AC3 -> `break-redesign.png` + `break-snapshot.md` 证明 break prompt 为单卡片环形倒计时与三按钮层级
- AC4 -> `browser-console-errors.log` 无错误；`npm --prefix apps/desktop run build` PASS

## Interaction Contract Coverage
- Interaction impact: direct
- Primary flow -> tests/evidence:
  - `settings-redesign.png`
  - `settings-snapshot.md`
- Fallback / secondary flow -> tests/evidence:
  - `break-redesign.png`
  - `break-snapshot.md`
- Visible states / transitions -> tests/evidence:
  - dirty/synced、advanced accordion、active break、break CTA、console error log

## Governance Gates
- Agent Config Validation: `python3 scripts/validate_agent_configs.py`
- Workflow Docs Validation: `python3 scripts/validate_workflow_docs.py --mode manual`
- Approval Escalation Owner: orchestrator

## False-pass Cases
- 只改 `App.tsx` / `styles.css`，但未真正接入 Tailwind/shadcn primitives
- 设置页截图漂亮，但 build 失败或 preview 已报错
- break prompt 只有空状态，没有 active break 证据
- docs/specs/logs/plans 仍保留 `TBD/INIT`

## Evidence Capture (UI / E2E)
- Required: yes
- Owner: orchestrator（single-agent-fallback）
- Artifacts path: docs/specs/TID-20260403-hig-tailwind-redesign/evidence/
- What to capture:
  - Screenshots:
    - `settings-redesign.png`
    - `break-redesign.png`
  - Video/trace (optional):
    - N/A
  - HAR/console logs (optional):
    - `browser-console-errors.log`
    - `settings-snapshot.md`
    - `break-snapshot.md`

## Quality Gates (Non-functional)
- a11y:
  - 使用原生 button / input / textarea 与 Radix switch/select/accordion，确保基础键盘可达性
- perf budget:
  - 构建成功且前端 bundle 未引入异常级别体积膨胀
- error handling / observability:
  - 无浏览器 console error
  - invoke 失败时仍有 error banner
- security / privacy:
  - 无新增外部请求或敏感持久化

## Boundary / Invalid Input Cases
- 数字输入通过 `clampNumber` 收敛到最小/最大值
- shortcut 文本允许为空，继续表示“不注册”
- mocked break prompt 仅作为证据路径，不影响生产逻辑

## Concurrency / Race Cases (if applicable)
- polling 与 dirty form 的竞态通过 `dirty` 闸门规避
- previewTransform 仅在浏览器 preview 下更新本地 snapshot

## Mocks & Test Data
- break prompt 证据使用 Playwright 注入 mocked `__TAURI_INTERNALS__` 和 `DesktopSnapshot`

## Commands to Run
- `npm --prefix apps/desktop run typecheck`
- `npm --prefix apps/desktop run build`
- 浏览器预览 + Playwright 截图 / snapshot / console log
- `python3 scripts/validate_workflow_docs.py --mode manual`

## Expected Results
- PASS criteria:
  - build 通过
  - 设置页与 break prompt 都有最终证据
  - docs 门禁通过
- Outputs to keep (10~20 lines snippet):
  - `vite v7.3.1 building client environment for production...`
  - `✓ built in ...`
