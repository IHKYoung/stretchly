# Task-ID: TID-20260409-break-surface-i18n-reduction

## Summary
- Title: 收敛休息界面细节、接回完整多语言并审查 legacy app 资产
- Date: 2026-04-09
- Level: complex
- Lane: deep
- Execution Profile: sequential-phases
- Status: DONE

## Requirement Brief
- Goal restatement: 把当前 break prompt 和设置页继续收干净，修复玻璃感、16:9 窗口比例、壁纸完整预览、结束音效和设置文案；同时把前后端 i18n 从双文件硬编码改成按语言消息文件与语言配置文件驱动的结构，并审查 legacy `app/` 是否还能删。
- In-scope:
  - `apps/desktop/src/App.tsx` 的 break 视觉、设置文案、壁纸预览和音效设置/播放。
  - `apps/desktop/src-tauri/src/{state.rs,shell.rs,i18n.rs}` 的 settings schema、窗口 profile 与语言注册。
  - `apps/desktop/src/i18n.ts` 与 locale 目录结构，消除对 `zh-CN/en` 的硬编码。
  - legacy `app/` 的 locale / 入口 / 资源依赖审查，以及相应文档更新。
- Out-of-scope:
  - 重新设计 break 状态机、pause/focus 规则或 tray 行为。
  - 立即删除整个 legacy `app/` 目录而不做依赖审计。
  - 为所有 legacy 语言补齐全新的桌面端设置文案人工翻译。
- Assumptions:
  - 当前“纯净休息界面”的视觉基线保持不变，只做材质和比例优化。
  - 结束音效可以沿当前前端播放链路补齐，不必为此引入新依赖或新的原生音频宿主。
  - 旧 `app/locales/*.json` 仍是现阶段最完整的语言资产清单与未来迁移输入。
- Risks:
  - 如果结束音只在前端播放，窗口关闭时机可能影响个别边界场景的可听性。
  - 若把所有 legacy 语言立即暴露到新设置页，会因桌面端新增文案未全量翻译而再次出现混杂。
  - legacy `app/` 当前仍可能被根脚本、Electron 入口或历史资源链路引用，不能先删后看。
- Interaction impact: direct
- Primary visible flow: 用户在设置页调整休息窗口的背景、声音、显示方式与语言；休息开始/结束时看到更纯净的 break 界面并听到对应提示音。
- Fallback / secondary flow: 在浏览器 preview 或无 Tauri runtime 的情况下，设置页仍能正常预览 break 表面；语言未完整覆盖时回退到已完整覆盖的默认语言而不是暴露混杂 key。
- User-visible boundary: 休息窗口、设置页“休息窗口 / 休息音效 / 语言”相关区域，以及语言选择器。
- Key visible states / transitions:
  - break 运行中 -> 倒计时归零 -> 手动完成等待或直接结束。
  - 自定义壁纸已上传 -> 设置页预览完整显示。
  - 切换语言 -> 设置页与运行时文案同步切换且不露 key/占位符。

## Goal
- 完成当前桌面端 break / settings 的一轮收敛性打磨，并把 i18n 结构改成可扩展的长期方案，同时明确 legacy `app/` 的保留边界。

## Scope
- In-scope:
  - `apps/desktop/src/{App.tsx,i18n.ts,lib/break-prompt.ts,locales/**}`
  - `apps/desktop/src-tauri/src/{state.rs,shell.rs,i18n.rs}`
  - `docs/specs/TID-20260409-break-surface-i18n-reduction/**`
  - `docs/{plans,logs,UI,Architecture,SettingsInventory,CodeMap,CHANGELOG}.md`
- Out-of-scope:
  - Rust engine 调度逻辑与 reminder state machine
  - 非语言相关的 legacy Electron 运行时完整迁移

## Source Basis (Read Before Code)
- Related code/files reviewed:
  - `apps/desktop/src/App.tsx`
  - `apps/desktop/src/i18n.ts`
  - `apps/desktop/src/lib/break-prompt.ts`
  - `apps/desktop/src/locales/{zh-CN,en}.json`
  - `apps/desktop/src-tauri/src/{state.rs,shell.rs,i18n.rs,commands.rs}`
  - `app/preferences.html`
  - `app/preferences-renderer.js`
  - `app/utils/defaultSettings.js`
  - `app/locales/{en,zh-CN}.json`
- Related docs/specs/logs reviewed:
  - `docs/RepositoryGuidelines.md`
  - `docs/CodeMap.md`
  - `docs/logs/2026-04-08.md`
  - `docs/plans/2026-04-08.md`
  - `docs/specs/TID-20260408-break-prompt-purity-pass/*`
- Why these are sufficient:
  - 已覆盖当前 break/settings 前台实现、宿主 settings 真源、现有 i18n 双端加载、legacy 语言来源与上一轮 break prompt 设计边界，足以支撑本轮改动和 legacy `app/` 删除边界审查。

## Acceptance Criteria (AC)
- AC1: break prompt 保持纯净单列结构，同时卡片更通透，倒计时与条形进度仍为唯一主元素，不再出现旧的左右分栏和标签化提示。
- AC2: window 模式下的 break 窗口最小比例改为 16:9，且在小工作区内仍能安全居中显示。
- AC3: 设置页上传自定义壁纸后，预览区域完整显示图片，不拉伸、不裁掉主体。
- AC4: 设置页支持分别配置微休息/休息的开始音与结束音，休息结束时实际播放一次结束提示音。
- AC5: 设置页中文文案不再出现中英混杂、占位符或直接外露专业术语。
- AC6: 前后端 i18n 不再把支持语言与 bundle 写死在代码里；语言消息文件与语言配置文件按目录自动注册。
- AC7: 对 legacy `app/` 的保留边界给出明确结论：哪些仍被依赖、哪些可减、当前是否可整体删除。

## Interaction Freeze (only when `interaction_impact != none`)
- Freeze status: frozen for implementation
- Primary flow: 设置页调整休息窗口外观/声音/语言 -> 保存 -> break window 以对应材质、比例、文案和音效呈现。
- Fallback / secondary flow: 浏览器 preview 下的 break window 仍显示同一布局；未选择自定义壁纸时回退默认背景主题。
- Interaction authority / ownership boundary: 本轮仅调整已存在的 break window 与设置项，不新增新的操作入口、模式切换或宿主级按钮。
- Visible entrypoints / handoff cues: 设置页“休息窗口”“休息音效”“语言”分组；break window 中的倒计时、细进度条、完成/稍后/跳过动作。
- In-scope interactions:
  - break 卡片玻璃质感调整
  - 自定义壁纸预览显示逻辑
  - 开始/结束音选择与播放
  - 语言选择来源与显示
- Out-of-scope interactions:
  - 新增 break 动效、复杂 cue 卡片、额外的 break 操作流
  - 提醒状态机与严格模式定义变更
- Interaction acceptance criteria:
  - break 页面肉眼上比当前更通透、更克制
  - 设置页所有中文标签/说明保持纯中文用户表达
  - 语言切换不会露 key 或裸占位符
- Validator expectation: 交互字段已填写完整；Evidence required 需为 yes。

## Agent Governance
- Approval Owner: orchestrator
- Execution Profile: sequential-phases
- Orchestrator Execution Profile: aggressive baseline uses `sandbox_mode = "danger-full-access"` + `approval_policy = "never"`
- Required Roles: orchestrator,ui_designer,architect,coder,tester,scribe,evidence_collector,reality_checker
- Execution Mode Policy: multi-agent preferred; `single-agent-fallback` only when warmup is BLOCKED and scope / reason code are explicitly bounded
- Subagent Approval Policy: non-orchestrator agents must use `approval_policy = "never"`
- Escalation Route: subagent -> Orchestrator -> direct local execution | user (only for destructive or external-impact decisions)
- Safe-local Command Route: subagent -> Orchestrator -> bare repo-local command (for example `git add -- <explicit paths...>`)
- Approval Packet Fields: command / purpose / risk / rollback

## Execution Safety Block
- service_impact: 仅影响本地桌面端 break 前台、设置页、Tauri host settings schema、语言加载链路与 legacy 目录审查结果。
- touches_running_service: no
- backup_required: no
- backup_plan: 以 Git diff、`typecheck`、`build`、`cargo check`、docs validator 与 UI 证据为回滚/比对边界。
- rollback_plan: 回退 `apps/desktop/src/{App.tsx,i18n.ts,lib/break-prompt.ts,locales/**}`、`apps/desktop/src-tauri/src/{state.rs,shell.rs,i18n.rs}` 与本任务 docs；legacy `app/` 若仅做审查则无额外回滚动作。
- destructive_operations: 审查 legacy `app/` 删除可行性，但在审查完成前不直接删除目录；其余变更为本地代码与配置替换。
- operator_approval_required: no
- rationale: 不涉及线上服务、付费成本、外部副作用或提权；唯一潜在破坏性动作是 legacy 目录删除，本轮先审查结论后再决定。

## Task Breakdown & Definition of Done (DoD)
- [x] Task-1: 收敛 break prompt 与设置页当前体验
  - DoD: 玻璃感、16:9、壁纸完整预览、结束音效和中文文案清理均已落地，且不破坏现有纯净 break 结构。
- [x] Task-2: 抽离 i18n 为“消息文件 + 配置文件 + 自动注册”
  - DoD: 前后端均不再写死语言集合与 bundle；语言元信息和消息来源由目录结构驱动。
- [x] Task-3: 审查 legacy `app/` 目录是否还能删
  - DoD: 给出明确的可删/不可删边界，并把结论同步到 docs。

## Evidence Plan (UI / E2E)
- Evidence required: yes
- Owner: evidence_collector
- Artifact path: docs/specs/TID-20260409-break-surface-i18n-reduction/evidence/
- Interaction validation note: 覆盖 break 主界面玻璃感、设置页自定义壁纸预览与音效/语言相关区域，证明主交互和关键 visible states 已按 Freeze 收敛。
- Required states to capture:
  - loading: browser preview / desktop bootstrap 正常加载
  - empty: 未上传自定义壁纸时的空状态说明
  - error: 不要求刻意制造新错误态，但需确认无 key/占位符外露
  - disabled: preview 或保存中按钮禁用态不发生布局破坏
  - success: break prompt 成功渲染、壁纸完整预览成功、设置文案纯净

## Observability / Debug Plan
- Logs:
  - 继续复用当前前端保存/加载报错路径与 Rust settings 更新结果，不新增复杂 trace。
  - 若 i18n 注册发现缺 bundle / 缺 config，应以清晰错误信息回退到默认语言，而不是直接露 key。
- Error codes:
  - 维持字符串错误输出；本轮不新增新的原生 error code 枚举。
- Trace/metrics (optional):
- Debug flags (optional):

## Risks & Rollback
- Risks:
  - 结束音若仅依赖前端 `Audio`，在 break 关闭过快的路径上可能存在边界时序问题。
  - i18n 自动注册若前后端实现不一致，可能导致 language normalize / fallback 漂移。
  - legacy `app/` 当前工作区已有大量未提交变更，审查时必须避免误删用户已有修改。
- Rollback plan:
  - 逐文件回退本任务涉及的前台、Rust 和文档改动。
  - 若 i18n 新结构导致回归，先恢复到当前两语言 bundle，再保留 registry 脚本与审查结论单独推进。

## Sequential Phases
- phase_execution: sequential
- phase_confirmation_policy: no-intermediate-confirmation
- phase_stop_conditions:
  - 结束音无法在当前宿主链路可靠触发，需要回到设计层改成 Rust host 播放。
  - i18n 自动注册方案无法同时满足前端与 Rust host 的一致性。
  - 审查发现 legacy `app/` 仍是默认运行入口或关键资源真源，无法给出整体删除结论。

## Execution Plan
- Steps:
  1. 先落地 break / settings 当前体验修正，并补 settings schema。
  2. 把 i18n 改成目录化消息/配置方案，消除语言硬编码。
  3. 追踪根脚本、入口、资源与 locale 依赖，给出 legacy `app/` 减法结论。
  4. 运行前后端/Rust/doc 验证并补 UI 证据。

## Definition of Done (DoD)
- `plan.md` / `testplan.md` / `README.md` 不保留 `TBD/INIT` 占位进入 DONE。
- 授权 owner、升级路径、风险与回滚已写明。
- 相关验证命令与结果可在 logs/plans 中追溯。

## Approval
- Approval needed: yes
- Approved: yes
