# Scratch Workspaces

This directory contains local-only work that is not part of Pauza's supported
build, test, or release paths.

## `locale-translations/`

Historical South Asian locale translation generators and their intermediate
JSON/TXT files live here. The files remain flat because several one-off scripts
refer to sibling files by relative name.

The maintained locale sources of truth are:

- `apps/desktop/src/locales/messages/`
- `apps/desktop/src/locales/config/`
- `apps/desktop/src/locales/break-ideas/messages/`
- `apps/desktop/src/locales/break-ideas/registry.json`

Files under `locale-translations/` are intentionally ignored by Git. Some
scripts contain machine-specific absolute paths or expect temporary inputs, so
treat them as historical working material rather than reusable repository
tools. A new maintained locale tool belongs under `scripts/` and must use
repository-relative paths.
