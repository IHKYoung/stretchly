# Task-ID: TID-20260411-site-landing-page

## Meta
- Title: 搭建单页官网与下载入口
- Date: 2026-04-11
- Level: moderate  <!-- trivial | moderate | complex -->
- Lane: deep    <!-- fast | deep -->
- Execution Profile: single-task  <!-- single-task | sequential-phases | merge-gate -->
- Status: DONE  <!-- INIT | IN_PROGRESS | REVIEW | DONE | BLOCKED -->

## Links
- Plan (daily): ../../plans/2026-04-11.md
- Log (daily): ../../logs/2026-04-11.md
- Architecture Spec: ./arch.md
- UI Spec: ./ui.md
- Plan Summary: ./plan.md
- Test Plan: ./testplan.md
- Evidence Report: ./evidence/README.md

## Decision Log
- 官网先采用纯静态单页实现，落在 `apps/site`，避免为官网引入新的前端依赖和构建系统。
- 下载按钮不直接硬编码三方文件 URL，而是统一落到 `apps/site/download/` 下的稳定跳转页，后续无论切 GitHub Releases 还是对象存储，都只改一个配置文件。
- 页面视觉方向确定为“高端、克制、现代”，以大留白、柔和光晕、细节动画和简洁 copy 传达“更安静的久坐提醒”。
- 增补 `favicon.svg` 作为站点品牌图标，确保首页与下载路由页的控制台现实检查不再因缺失 favicon 产生噪音错误。

## Governance Notes
- Requirement Brief: 用户要求搭建一个单页官网，风格现代、高级、简洁，不需要长页面滚动；页面内直接承载产品介绍、能力摘要和下载按钮，并将下载跳转单独拆到一个文件夹管理。
- Interaction Impact: direct  <!-- none | indirect | direct -->
- Interaction Freeze: frozen；本轮只做单页官网首页和下载跳转页，不扩展到博客、文档站、多语言站点或真实部署流水线。
- Execution Safety Block: service_impact=仅限新增静态官网目录、根脚本接线、文档与本地预览验证；touches_running_service=no；backup_required=no；backup_plan=依赖本地静态服务器、浏览器现实检查与 workflow docs validator；rollback_plan=删除 `apps/site` 与相关 docs / scripts 接线后恢复；destructive_operations=none；operator_approval_required=no；rationale=不涉及线上服务、数据迁移、提权或外部付费动作。
- Approval Owner: orchestrator
- Execution Mode: single-agent-fallback
- Fallback Reason Code: other-blocked
- Escalation Summary: BLOCKED: 当前 session 未获用户显式 delegation 授权，遵循上层工具策略不调用 `spawn_agent`；本轮由单 agent 在受限范围内完成官网设计、实现、验证与文档闭环。
- Retention Decision: keep

## Notes
- 真实下载地址会集中写在 `apps/site/download/targets.js`；如果 GitHub Releases / R2 地址后续变化，不需要改 landing page 本身。
- 当前 `targets.js` 保持留空，用于验证“真实下载地址尚未配置”时的 fallback 交互；后续上线前只需填充对应 URL。
