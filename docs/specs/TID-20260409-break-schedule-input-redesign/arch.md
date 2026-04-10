# Task-ID: TID-20260409-break-schedule-input-redesign

## Goals
- 将节奏 preset 与手动输入拆成更稳定的组合：preset 负责常用值，手动输入负责自定义值。

## Non-Goals
- 不改后端设置 schema。

## Constraints & Assumptions
- 当前节奏页只有四组 preset 型输入：
  - 微休息间隔
  - 微休息时长
  - 长休息每几轮
  - 长休息时长

## System Boundaries
- Modules:
  - `apps/desktop/src/App.tsx`
  - `apps/desktop/src/lib/settings-controls.ts`
- Dependency direction:
  - helper -> App preset controls -> autosave settings

## Invariants
- preset 始终为 5 个候选项。
- 自定义输入允许清空草稿，但提交后仍遵守 min/max clamp。

## Risks & Rollback
- Failure modes:
  - 草稿输入与父级受控值不同步
- Rollback steps:
  - 回退 `App.tsx` 和 `settings-controls.ts`

## Acceptance Criteria (System)
- 自定义输入可先清空再录入。
- 4 组 preset 均为 5 项，且值分布更均匀。
