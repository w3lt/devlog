# devlog documentation site

This is the Astro Starlight documentation site for
[`devlog`](https://github.com/w3lt/devlog), the Rust CLI published as the
`d3vlog` crate.

The docs project is intentionally self-contained under `docs/`. The repository
root `README.md` remains the GitHub and crates.io front page.

## Local development

Run commands from this `docs/` directory:

```bash
pnpm install
pnpm dev
pnpm check
pnpm build
pnpm preview
```

With `base: '/devlog'`, local dev and preview serve the site under
`/devlog`, for example:

```text
http://localhost:4321/devlog
```

## Deployment

The site deploys to GitHub Pages with `.github/workflows/docs.yml`.

The workflow builds this directory with `withastro/action@v3`, uploads the
Pages artifact, and deploys it with `actions/deploy-pages@v4`.

Before the first deploy, enable Pages in the GitHub repository:

```text
Settings -> Pages -> Source -> GitHub Actions
```

The deployed site is expected at:

```text
https://w3lt.github.io/devlog
```

## Validation

From the repository root, run:

```bash
pnpm --dir docs check
pnpm --dir docs build
cargo build
```
