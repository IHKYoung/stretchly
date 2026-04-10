# Task-ID: TID-20260409-break-schedule-input-redesign

## Goals
- 让节奏输入既能快速点 preset，又能顺畅录入自定义值。

## Screens & User Flows
- Primary flow: 用户在 `节奏` 页点击常用 preset 或先清空输入框再录入自定义数字
- User-visible boundary: `微休息` 与 `休息` 的 4 组 preset rows

## Interaction States
- focus: 输入框可为空草稿
- active: Enter 提交当前草稿
- error: 非数字字符直接忽略，不污染当前草稿

## Accessibility (a11y)
- keyboard navigation: 输入框支持 Enter 提交、Escape 恢复当前值

## Acceptance Criteria (UI)
- 退格清空输入框时不再立刻被最小值回填。
- preset 与自定义输入同一行共存，不互相打架。
