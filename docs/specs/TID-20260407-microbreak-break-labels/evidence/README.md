# Task-ID: TID-20260407-microbreak-break-labels

## Evidence Summary
- Artifact: `settings-terminology-snapshot.md`
- Artifact: `browser-console.log`
- Capture source: 2026-04-07 复用本地 `http://127.0.0.1:43179/` 浏览器 preview，提取设置页可访问性快照与控制台摘要。

## AC Mapping
- AC1: `settings-terminology-snapshot.md` 直接展示主设置页中“微休息”和“休息”两组标题、group label 与下方“提前提醒 / 延后”的对应标签。
- AC2: locale diff 与 `settings-terminology-snapshot.md` 共同证明 legacy `小憩` / `Mini break` / `Long break` 已从当前可见术语中退出。
- AC3: 本任务的 README / plan / daily log 已明确记录内部兼容标识保留策略，`README.md` 与 `docs/CHANGELOG.md` 同步更新。
- AC4: `browser-console.log` 无新增显著错误；其余验证见当日 `docs/logs/2026-04-07.md`。

## Interaction Freeze Check
- Primary flow: 用户在主设置页中直接看到“微休息 / 休息”的成对命名。
- Fallback flow: tray skip 与 runtime break 文案复用同一套 locale 术语，但本轮不改变原有交互顺序。
- Visible states: 快照覆盖微休息 / 休息开关开启态、预设分组标签，以及“提前提醒 / 延后”中的对应标签。
