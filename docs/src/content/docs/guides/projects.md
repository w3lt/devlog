---
title: Projects
description: Group and filter devlog entries with project tags.
---

Group related entries by tagging them with `-p` or `--project` when you add
them:

```console
$ devlog add "Bump rusqlite to 0.40" --project devlog
Added item "Bump rusqlite to 0.40"!
```

The project is created automatically the first time you mention it.

Use `devlog list --project <name>` to narrow your history to a single project:

```console
$ devlog list --project devlog
Wednesday, 2026-06-24 · 1 entry

  [~] 11:02  Fix flaky test in store.rs · devlog
      id: 019efa5e-5f2a-7eb0-9ed7-9980495715a5
```

A plain `devlog list` still shows everything, tagged or not.
