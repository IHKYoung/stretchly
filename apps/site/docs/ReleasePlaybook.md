# Release Playbook

## 目标

把 Pauza 的站点 pinned 下载链接与 GitHub Release 发布动作收敛成一个可重复执行的标准流程。

## 前置条件

- 已经安装并登录 `gh`
- 目标资产路径存在，例如：
  - `/Users/changkunyang/CKProjects/Pauza/apps/desktop/src-tauri/target/release/bundle/dmg/Pauza_0.1.3_aarch64.dmg`
- 当前仓库位于 `apps/site/`

## 一键命令

```bash
python3 scripts/publish_site_release.py \
  --asset /Users/changkunyang/CKProjects/Pauza/apps/desktop/src-tauri/target/release/bundle/dmg/Pauza_0.1.3_aarch64.dmg
```

## 脚本会做什么

1. 从 dmg 文件名解析版本号，例如 `0.1.3`
2. 自动推导：
   - tag: `v0.1.3`
   - release title: `v0.1.3`
   - release asset URL: `https://github.com/IHKYoung/Pauza/releases/download/v0.1.3/Pauza_0.1.3_aarch64.dmg`
3. 更新站点 pinned 下载链接：
   - `download/targets.js`
   - `index.html`
4. 调用 `gh`：
   - 若 release 不存在：`gh release create`
   - 若 release 已存在：`gh release upload --clobber`

## 常用变体

只更新站点 pinned 链接，不发 release：

```bash
python3 scripts/publish_site_release.py --asset /abs/path/Pauza_0.1.3_aarch64.dmg --skip-release
```

只演练命令，不实际执行：

```bash
python3 scripts/publish_site_release.py --asset /abs/path/Pauza_0.1.3_aarch64.dmg --dry-run
```

自定义说明文字：

```bash
python3 scripts/publish_site_release.py \
  --asset /abs/path/Pauza_0.1.3_aarch64.dmg \
  --notes "Pauza 0.1.3 release"
```

## 发布后建议动作

- 检查 `gh release view v<version> --repo IHKYoung/Pauza`
- 运行：
  - `python3 scripts/validate_agent_configs.py`
  - `python3 scripts/validate_workflow_docs.py --mode manual`
- 提交站点变更与 workflow 文档
- 推送当前分支

## 风险与回滚

- 若错误上传了同版本 release，可重新执行脚本覆盖资产
- 若站点 pinned 链接写错，回滚 `index.html` 与 `download/targets.js`
- 若需要整体回退，直接 `git revert` 对应站点变更 commit
