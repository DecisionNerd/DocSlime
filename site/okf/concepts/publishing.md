---
type: concept
title: Publishing
description: "How DocSlime docs stay ready for docmd.io without making DocSlime a publishing host."
source: "https://www.docslime.dev/publishing/"
path: /publishing/
updated: 2026-07-10
okf:
  generated_by: "@docmd/plugin-okf"
  generated_at: "2026-07-10T01:49:44.536Z"
---
---
title: "Publishing"
description: "How DocSlime docs stay ready for docmd.io without making DocSlime a publishing host."
---

# Publishing

DocSlime prepares an in-repo Markdown docs tree. It does not host, render, or deploy that
tree itself. The publishing boundary is intentionally thin: keep the Markdown clean, point
`docmd` at `docs/`, build the static site, then deploy the generated output through the
hosting path your project already uses.

## Boundary

| DocSlime owns | docmd.io owns |
|---|---|
| The `docs/` structure and templates | Static-site generation from Markdown |
| Product, design, requirements, architecture, testing, and ADR context | Navigation, search, theming, SEO, `llms.txt`, and output assets |
| Agent skills that fill, review, and tighten docs | Deployment helpers and platform-specific hosting guidance |

DocSlime should not duplicate the `docmd.io` publishing docs. When publishing behavior
changes, the official docs should remain the source of truth.

## Local DocSlime Site

This repository dogfoods the publishing path with `docmd.config.json`:

```sh
npm run build
```

That command runs `docmd build`, reads Markdown from `docs/`, and writes the static site to
`site/`.

## Official docmd.io References

- [docmd Quick Start](https://docs.docmd.io/getting-started/quick-start/) - local dev and
  production build basics.
- [docmd Deployment Overview](https://docs.docmd.io/deployment/) - deployment methods,
  static output, and production checklist.
- [Choosing Your Deployment Method](https://docs.docmd.io/guides/integrations/choosing-deployment-method/) -
  when to use deploy helpers such as Vercel, Netlify, Docker, NGINX, or Caddy.

For DocSlime projects, the main requirement is simple: keep the generated Markdown plain,
linked, and free of unfinished `LLM:` guidance before handing it to `docmd`.
