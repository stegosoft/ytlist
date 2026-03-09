# Branching Strategy

## Goal

Keep internal development history flexible while keeping the public GitHub history clean and release-focused.

For this repository:

- `dev` keeps the full working history
- `main` receives only the cleaned public snapshot intended for GitHub

## Branch Roles

- `dev`: full development branch
  - Keep day-to-day commits here
  - Experimental, iterative, or noisy commits are acceptable
  - This branch may stay local or private

- `main`: public release branch
  - Only keep cleaned, publication-ready commits here
  - Use this branch for GitHub visibility, release tags, and release workflows
  - Avoid pushing raw development history to this branch

## Recommended Workflow

1. Start and continue development on `dev`.
2. Commit freely while building features or fixing issues.
3. When preparing a public release, switch to `main`.
4. Bring changes from `dev` into `main` as a cleaned commit.
5. Push `main` to GitHub.
6. Create release tags from `main` only.

## Publishing Options

### Option A: Squash Publish

Use when `main` already exists and you want one clean release commit:

```bash
git checkout main
git merge --squash dev
git commit -m "chore: prepare public release"
git push origin main
```

### Option B: Rebuild Public History

Use when `main` contains noisy history and needs a clean restart:

```bash
git checkout dev
git checkout --orphan main
git add .
git commit -m "chore: publish initial open source release"
git push -u origin main
```

## Tagging Rules

- Create tags from `main` only
- Do not tag from `dev`
- If a tag must move after cleanup:

```bash
git tag -f v0.1.0
git push origin +refs/tags/v0.1.0:refs/tags/v0.1.0
```

## Notes

- Use `stegosoft@gmail.com` for public commits in this repository
- Remove `Co-authored-by` trailers if you do not want them shown publicly
- Do not use `git add .` if the repo contains private local files
